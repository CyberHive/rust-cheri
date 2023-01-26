use crate::traits::*;

use rustc_middle::ty::{self, subst::GenericArgKind, Ty};
use rustc_session::config::Lto;
use rustc_symbol_mangling::typeid_for_trait_ref;
use rustc_target::abi::call::FnAbi;

#[derive(Copy, Clone, Debug)]
pub struct VirtualIndex(u64);

impl<'a, 'tcx> VirtualIndex {
    pub fn from_index(index: usize) -> Self {
        VirtualIndex(index as u64)
    }

    pub fn get_fn<Bx: BuilderMethods<'a, 'tcx>>(
        self,
        bx: &mut Bx,
        llvtable: Bx::Value,
        ty: Ty<'tcx>,
        fn_abi: &FnAbi<'tcx, Ty<'tcx>>,
    ) -> Bx::Value {
        // Load the data pointer from the object.
        debug!("get_fn({llvtable:?}, {ty:?}, {self:?})");

        let dl = &bx.tcx().data_layout;

        let llty = bx.fn_ptr_backend_type(fn_abi);
        // TODO: Get the correct address space. Probably global?
        let llvtable = bx.pointercast(llvtable, bx.type_ptr_to_ext(llty, dl.default_address_space));

        if bx.cx().sess().opts.unstable_opts.virtual_function_elimination
            && bx.cx().sess().lto() == Lto::Fat
        {
            let typeid =
                bx.typeid_metadata(typeid_for_trait_ref(bx.tcx(), expect_dyn_trait_in_self(ty)));
            // TODO: val_size vs ty_size.
            let vtable_byte_offset = self.0 * dl.ptr_layout(Some(dl.instruction_address_space)).ty_size.bytes();
            let type_checked_load = bx.type_checked_load(llvtable, vtable_byte_offset, typeid);
            let func = bx.extract_value(type_checked_load, 0);
            bx.pointercast(func, llty)
        } else {
            let ptr_align = dl.ptr_layout(Some(dl.instruction_address_space)).align.abi;
            let gep = bx.inbounds_gep(llty, llvtable, &[bx.const_usize(self.0)]);
            let ptr = bx.load(llty, gep, ptr_align);
            bx.nonnull_metadata(ptr);
            // VTable loads are invariant.
            bx.set_invariant_load(ptr);
            ptr
        }
    }

    pub fn get_usize<Bx: BuilderMethods<'a, 'tcx>>(
        self,
        bx: &mut Bx,
        llvtable: Bx::Value,
    ) -> Bx::Value {
        // Load the data pointer from the object.
        debug!("get_int({:?}, {:?})", llvtable, self);

        let dl = &bx.tcx().data_layout;

        let llty = bx.type_isize();
        // TODO: Get the correct address space. Probably global?
        let llvtable = bx.pointercast(llvtable, bx.type_ptr_to_ext(llty, dl.default_address_space));
        // NOTE: This was previously `usize_align`. Was there some significance to that or is the
        // pointer alignment the correct thing to use here?
        let ptr_align = dl.ptr_layout(Some(dl.instruction_address_space)).align.abi;
        let gep = bx.inbounds_gep(llty, llvtable, &[bx.const_usize(self.0)]);
        let ptr = bx.load(llty, gep, ptr_align);
        // VTable loads are invariant.
        bx.set_invariant_load(ptr);
        ptr
    }
}

/// This takes a valid `self` receiver type and extracts the principal trait
/// ref of the type.
fn expect_dyn_trait_in_self<'tcx>(ty: Ty<'tcx>) -> ty::PolyExistentialTraitRef<'tcx> {
    for arg in ty.peel_refs().walk() {
        if let GenericArgKind::Type(ty) = arg.unpack() {
            if let ty::Dynamic(data, _, _) = ty.kind() {
                return data.principal().expect("expected principal trait object");
            }
        }
    }

    bug!("expected a `dyn Trait` ty, found {ty:?}")
}

/// Creates a dynamic vtable for the given type and vtable origin.
/// This is used only for objects.
///
/// The vtables are cached instead of created on every call.
///
/// The `trait_ref` encodes the erased self type. Hence if we are
/// making an object `Foo<dyn Trait>` from a value of type `Foo<T>`, then
/// `trait_ref` would map `T: Trait`.
#[instrument(level = "debug", skip(cx))]
pub fn get_vtable<'tcx, Cx: CodegenMethods<'tcx>>(
    cx: &Cx,
    ty: Ty<'tcx>,
    trait_ref: Option<ty::PolyExistentialTraitRef<'tcx>>,
) -> Cx::Value {
    let tcx = cx.tcx();

    // Check the cache.
    if let Some(&val) = cx.vtables().borrow().get(&(ty, trait_ref)) {
        return val;
    }

    let vtable_alloc_id = tcx.vtable_allocation((ty, trait_ref));
    let vtable_allocation = tcx.global_alloc(vtable_alloc_id).unwrap_memory();
    let vtable_const = cx.const_data_from_alloc(vtable_allocation);
    let dl = &cx.data_layout();
    let align = dl.ptr_layout(Some(dl.instruction_address_space)).align.abi;
    let vtable = cx.static_addr_of(vtable_const, align, Some("vtable"));

    cx.create_vtable_debuginfo(ty, trait_ref, vtable);
    cx.vtables().borrow_mut().insert((ty, trait_ref), vtable);
    vtable
}
