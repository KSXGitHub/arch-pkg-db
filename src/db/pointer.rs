use super::{Lookup, LookupMut, PackageDatabase};
use arch_pkg_text::value::Name;
use core::{
    ops::{Deref, DerefMut},
    pin::Pin,
};
use std::{rc::Rc, sync::Arc};

macro_rules! impl_base {
    ($container:ident $($lt:lifetime)?) => {
        impl<$($lt,)? Db: PackageDatabase + ?Sized> PackageDatabase for $container<$($lt,)? Db> {
            type Querier = Db::Querier;
        }
    };
}

macro_rules! impl_ref {
    ($container:ident $($lt:lifetime)?) => {
        impl<$($lt,)? Db: Lookup + ?Sized> Lookup for $container<$($lt,)? Db> {
            type Error = Db::Error;
            fn lookup(&self, name: Name<'_>) -> Result<&'_ Self::Querier, Self::Error> {
                Db::lookup(self, name)
            }
        }
    };
}

macro_rules! impl_mut {
    ($container:ident $($lt:lifetime)?) => {
        impl<$($lt,)? Db: LookupMut + ?Sized> LookupMut for $container<$($lt,)? Db> {
            type Error = Db::Error;
            fn lookup_mut(&mut self, name: Name<'_>) -> Result<&'_ mut Self::Querier, Self::Error> {
                Db::lookup_mut(self, name)
            }
        }
    };
}

type Ref<'a, Db> = &'a Db;
impl_base!(Ref 'a);
impl_ref!(Ref 'a);

type Mut<'a, Db> = &'a mut Db;
impl_base!(Mut 'a);
impl_mut!(Mut 'a);

impl_base!(Box);
impl_ref!(Box);
impl_mut!(Box);

impl_base!(Rc);
impl_ref!(Rc);

impl_base!(Arc);
impl_ref!(Arc);

impl<Ptr: Deref<Target: PackageDatabase>> PackageDatabase for Pin<Ptr> {
    type Querier = <Ptr::Target as PackageDatabase>::Querier;
}

impl<Ptr: Deref<Target: Lookup>> Lookup for Pin<Ptr> {
    type Error = <Ptr::Target as Lookup>::Error;
    fn lookup(&self, name: Name<'_>) -> Result<&'_ Self::Querier, Self::Error> {
        self.deref().lookup(name)
    }
}

impl<Ptr: DerefMut<Target: LookupMut + Unpin>> LookupMut for Pin<Ptr> {
    type Error = <Ptr::Target as LookupMut>::Error;
    fn lookup_mut(&mut self, name: Name<'_>) -> Result<&'_ mut Self::Querier, Self::Error> {
        self.deref_mut().lookup_mut(name)
    }
}
