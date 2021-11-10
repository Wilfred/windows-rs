#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_UI_Controls`*"]
#[cfg(feature = "Win32_UI_Controls")]
#[inline]
pub unsafe fn CreateSecurityPage<'a, Param0: ::windows::runtime::IntoParam<'a, ISecurityInformation>>(psi: Param0) -> super::super::super::UI::Controls::HPROPSHEETPAGE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateSecurityPage(psi: ::windows::runtime::RawPtr) -> super::super::super::UI::Controls::HPROPSHEETPAGE;
        }
        ::core::mem::transmute(CreateSecurityPage(psi.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const DOBJ_COND_NTACLS: i32 = 8i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const DOBJ_RES_CONT: i32 = 1i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const DOBJ_RES_ROOT: i32 = 2i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const DOBJ_RIBBON_LAUNCH: i32 = 16i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const DOBJ_VOL_NTACLS: i32 = 4i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_Foundation`*"]
pub struct EFFPERM_RESULT_LIST {
    pub fEvaluated: super::super::super::Foundation::BOOLEAN,
    pub cObjectTypeListLength: u32,
    pub pObjectTypeList: *mut super::super::OBJECT_TYPE_LIST,
    pub pGrantedAccessList: *mut u32,
}
#[cfg(feature = "Win32_Foundation")]
impl EFFPERM_RESULT_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EFFPERM_RESULT_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EFFPERM_RESULT_LIST {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EFFPERM_RESULT_LIST").field("fEvaluated", &self.fEvaluated).field("cObjectTypeListLength", &self.cObjectTypeListLength).field("pObjectTypeList", &self.pObjectTypeList).field("pGrantedAccessList", &self.pGrantedAccessList).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EFFPERM_RESULT_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.fEvaluated == other.fEvaluated && self.cObjectTypeListLength == other.cObjectTypeListLength && self.pObjectTypeList == other.pObjectTypeList && self.pGrantedAccessList == other.pGrantedAccessList
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EFFPERM_RESULT_LIST {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EFFPERM_RESULT_LIST {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EditSecurity<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, ISecurityInformation>>(hwndowner: Param0, psi: Param1) -> super::super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EditSecurity(hwndowner: super::super::super::Foundation::HWND, psi: ::windows::runtime::RawPtr) -> super::super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EditSecurity(hwndowner.into_param().abi(), psi.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EditSecurityAdvanced<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, ISecurityInformation>>(hwndowner: Param0, psi: Param1, usipage: SI_PAGE_TYPE) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EditSecurityAdvanced(hwndowner: super::super::super::Foundation::HWND, psi: ::windows::runtime::RawPtr, usipage: SI_PAGE_TYPE) -> ::windows::runtime::HRESULT;
        }
        EditSecurityAdvanced(hwndowner.into_param().abi(), psi.into_param().abi(), ::core::mem::transmute(usipage)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEffectivePermission(pub ::windows::runtime::IUnknown);
impl IEffectivePermission {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_Foundation`*"]
    pub unsafe fn GetEffectivePermission<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSID>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(
        &self,
        pguidobjecttype: *const ::windows::runtime::GUID,
        pusersid: Param1,
        pszservername: Param2,
        psd: *mut super::super::SECURITY_DESCRIPTOR,
        ppobjecttypelist: *mut *mut super::super::OBJECT_TYPE_LIST,
        pcobjecttypelistlength: *mut u32,
        ppgrantedaccesslist: *mut *mut u32,
        pcgrantedaccesslistlength: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(pguidobjecttype),
            pusersid.into_param().abi(),
            pszservername.into_param().abi(),
            ::core::mem::transmute(psd),
            ::core::mem::transmute(ppobjecttypelist),
            ::core::mem::transmute(pcobjecttypelistlength),
            ::core::mem::transmute(ppgrantedaccesslist),
            ::core::mem::transmute(pcgrantedaccesslistlength),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IEffectivePermission {
    type Vtable = IEffectivePermission_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(945020022, 40757, 16508, [136, 161, 209, 147, 68, 54, 95, 188]);
}
impl ::core::convert::From<IEffectivePermission> for ::windows::runtime::IUnknown {
    fn from(value: IEffectivePermission) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEffectivePermission> for ::windows::runtime::IUnknown {
    fn from(value: &IEffectivePermission) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEffectivePermission {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEffectivePermission {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEffectivePermission_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidobjecttype: *const ::windows::runtime::GUID, pusersid: super::super::super::Foundation::PSID, pszservername: super::super::super::Foundation::PWSTR, psd: *mut super::super::SECURITY_DESCRIPTOR, ppobjecttypelist: *mut *mut super::super::OBJECT_TYPE_LIST, pcobjecttypelistlength: *mut u32, ppgrantedaccesslist: *mut *mut u32, pcgrantedaccesslistlength: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEffectivePermission2(pub ::windows::runtime::IUnknown);
impl IEffectivePermission2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_Foundation`*"]
    pub unsafe fn ComputeEffectivePermissionWithSecondarySecurity<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSID>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PSID>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(
        &self,
        psid: Param0,
        pdevicesid: Param1,
        pszservername: Param2,
        psecurityobjects: *mut SECURITY_OBJECT,
        dwsecurityobjectcount: u32,
        pusergroups: *const super::super::TOKEN_GROUPS,
        pauthzusergroupsoperations: *const super::AUTHZ_SID_OPERATION,
        pdevicegroups: *const super::super::TOKEN_GROUPS,
        pauthzdevicegroupsoperations: *const super::AUTHZ_SID_OPERATION,
        pauthzuserclaims: *const super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION,
        pauthzuserclaimsoperations: *const super::AUTHZ_SECURITY_ATTRIBUTE_OPERATION,
        pauthzdeviceclaims: *const super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION,
        pauthzdeviceclaimsoperations: *const super::AUTHZ_SECURITY_ATTRIBUTE_OPERATION,
        peffpermresultlists: *mut EFFPERM_RESULT_LIST,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::core::mem::transmute_copy(self),
            psid.into_param().abi(),
            pdevicesid.into_param().abi(),
            pszservername.into_param().abi(),
            ::core::mem::transmute(psecurityobjects),
            ::core::mem::transmute(dwsecurityobjectcount),
            ::core::mem::transmute(pusergroups),
            ::core::mem::transmute(pauthzusergroupsoperations),
            ::core::mem::transmute(pdevicegroups),
            ::core::mem::transmute(pauthzdevicegroupsoperations),
            ::core::mem::transmute(pauthzuserclaims),
            ::core::mem::transmute(pauthzuserclaimsoperations),
            ::core::mem::transmute(pauthzdeviceclaims),
            ::core::mem::transmute(pauthzdeviceclaimsoperations),
            ::core::mem::transmute(peffpermresultlists),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IEffectivePermission2 {
    type Vtable = IEffectivePermission2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2485103562, 56647, 20426, [144, 187, 176, 225, 2, 85, 242, 13]);
}
impl ::core::convert::From<IEffectivePermission2> for ::windows::runtime::IUnknown {
    fn from(value: IEffectivePermission2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEffectivePermission2> for ::windows::runtime::IUnknown {
    fn from(value: &IEffectivePermission2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEffectivePermission2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEffectivePermission2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEffectivePermission2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        psid: super::super::super::Foundation::PSID,
        pdevicesid: super::super::super::Foundation::PSID,
        pszservername: super::super::super::Foundation::PWSTR,
        psecurityobjects: *mut SECURITY_OBJECT,
        dwsecurityobjectcount: u32,
        pusergroups: *const super::super::TOKEN_GROUPS,
        pauthzusergroupsoperations: *const super::AUTHZ_SID_OPERATION,
        pdevicegroups: *const super::super::TOKEN_GROUPS,
        pauthzdevicegroupsoperations: *const super::AUTHZ_SID_OPERATION,
        pauthzuserclaims: *const super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION,
        pauthzuserclaimsoperations: *const super::AUTHZ_SECURITY_ATTRIBUTE_OPERATION,
        pauthzdeviceclaims: *const super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION,
        pauthzdeviceclaimsoperations: *const super::AUTHZ_SECURITY_ATTRIBUTE_OPERATION,
        peffpermresultlists: *mut EFFPERM_RESULT_LIST,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISecurityInformation(pub ::windows::runtime::IUnknown);
impl ISecurityInformation {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_Foundation`*"]
    pub unsafe fn GetObjectInformation(&self, pobjectinfo: *mut SI_OBJECT_INFO) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pobjectinfo)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_Foundation`*"]
    pub unsafe fn GetSecurity<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, requestedinformation: super::super::OBJECT_SECURITY_INFORMATION, ppsecuritydescriptor: *mut *mut super::super::SECURITY_DESCRIPTOR, fdefault: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(requestedinformation), ::core::mem::transmute(ppsecuritydescriptor), fdefault.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_Foundation`*"]
    pub unsafe fn SetSecurity(&self, securityinformation: super::super::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: *mut super::super::SECURITY_DESCRIPTOR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(securityinformation), ::core::mem::transmute(psecuritydescriptor)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_Foundation`*"]
    pub unsafe fn GetAccessRights(&self, pguidobjecttype: *const ::windows::runtime::GUID, dwflags: SECURITY_INFO_PAGE_FLAGS, ppaccess: *mut *mut SI_ACCESS, pcaccesses: *mut u32, pidefaultaccess: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguidobjecttype), ::core::mem::transmute(dwflags), ::core::mem::transmute(ppaccess), ::core::mem::transmute(pcaccesses), ::core::mem::transmute(pidefaultaccess)).ok()
    }
    #[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
    pub unsafe fn MapGeneric(&self, pguidobjecttype: *const ::windows::runtime::GUID, paceflags: *mut u8, pmask: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguidobjecttype), ::core::mem::transmute(paceflags), ::core::mem::transmute(pmask)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_Foundation`*"]
    pub unsafe fn GetInheritTypes(&self, ppinherittypes: *mut *mut SI_INHERIT_TYPE, pcinherittypes: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppinherittypes), ::core::mem::transmute(pcinherittypes)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    #[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    pub unsafe fn PropertySheetPageCallback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwnd: Param0, umsg: super::super::super::UI::Controls::PSPCB_MESSAGE, upage: SI_PAGE_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), ::core::mem::transmute(umsg), ::core::mem::transmute(upage)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISecurityInformation {
    type Vtable = ISecurityInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2522858336, 5887, 4560, [145, 203, 0, 170, 0, 187, 183, 35]);
}
impl ::core::convert::From<ISecurityInformation> for ::windows::runtime::IUnknown {
    fn from(value: ISecurityInformation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISecurityInformation> for ::windows::runtime::IUnknown {
    fn from(value: &ISecurityInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISecurityInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISecurityInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pobjectinfo: *mut SI_OBJECT_INFO) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, requestedinformation: super::super::OBJECT_SECURITY_INFORMATION, ppsecuritydescriptor: *mut *mut super::super::SECURITY_DESCRIPTOR, fdefault: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, securityinformation: super::super::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: *mut super::super::SECURITY_DESCRIPTOR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidobjecttype: *const ::windows::runtime::GUID, dwflags: SECURITY_INFO_PAGE_FLAGS, ppaccess: *mut *mut SI_ACCESS, pcaccesses: *mut u32, pidefaultaccess: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidobjecttype: *const ::windows::runtime::GUID, paceflags: *mut u8, pmask: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppinherittypes: *mut *mut SI_INHERIT_TYPE, pcinherittypes: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwnd: super::super::super::Foundation::HWND, umsg: super::super::super::UI::Controls::PSPCB_MESSAGE, upage: SI_PAGE_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls")))] usize,
);
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISecurityInformation2(pub ::windows::runtime::IUnknown);
impl ISecurityInformation2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_Foundation`*"]
    pub unsafe fn IsDaclCanonical(&self, pdacl: *mut super::super::ACL) -> super::super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdacl)))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn LookupSids(&self, csids: u32, rgpsids: *mut super::super::super::Foundation::PSID, ppdo: *mut ::core::option::Option<super::super::super::System::Com::IDataObject>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(csids), ::core::mem::transmute(rgpsids), ::core::mem::transmute(ppdo)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISecurityInformation2 {
    type Vtable = ISecurityInformation2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3284991412, 28552, 4562, [163, 206, 0, 192, 79, 177, 120, 42]);
}
impl ::core::convert::From<ISecurityInformation2> for ::windows::runtime::IUnknown {
    fn from(value: ISecurityInformation2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISecurityInformation2> for ::windows::runtime::IUnknown {
    fn from(value: &ISecurityInformation2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISecurityInformation2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISecurityInformation2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityInformation2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdacl: *mut super::super::ACL) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, csids: u32, rgpsids: *mut super::super::super::Foundation::PSID, ppdo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISecurityInformation3(pub ::windows::runtime::IUnknown);
impl ISecurityInformation3 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_Foundation`*"]
    pub unsafe fn GetFullResourceName(&self) -> ::windows::runtime::Result<super::super::super::Foundation::PWSTR> {
        let mut result__: <super::super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_Foundation`*"]
    pub unsafe fn OpenElevatedEditor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwnd: Param0, upage: SI_PAGE_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), ::core::mem::transmute(upage)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISecurityInformation3 {
    type Vtable = ISecurityInformation3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3805137356, 12733, 20367, [140, 139, 182, 65, 175, 81, 106, 26]);
}
impl ::core::convert::From<ISecurityInformation3> for ::windows::runtime::IUnknown {
    fn from(value: ISecurityInformation3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISecurityInformation3> for ::windows::runtime::IUnknown {
    fn from(value: &ISecurityInformation3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISecurityInformation3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISecurityInformation3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityInformation3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszresourcename: *mut super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwnd: super::super::super::Foundation::HWND, upage: SI_PAGE_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISecurityInformation4(pub ::windows::runtime::IUnknown);
impl ISecurityInformation4 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_Foundation`*"]
    pub unsafe fn GetSecondarySecurity(&self, psecurityobjects: *mut *mut SECURITY_OBJECT, psecurityobjectcount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(psecurityobjects), ::core::mem::transmute(psecurityobjectcount)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISecurityInformation4 {
    type Vtable = ISecurityInformation4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3935703152, 52500, 17953, [172, 228, 246, 60, 3, 229, 131, 228]);
}
impl ::core::convert::From<ISecurityInformation4> for ::windows::runtime::IUnknown {
    fn from(value: ISecurityInformation4) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISecurityInformation4> for ::windows::runtime::IUnknown {
    fn from(value: &ISecurityInformation4) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISecurityInformation4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISecurityInformation4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityInformation4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psecurityobjects: *mut *mut SECURITY_OBJECT, psecurityobjectcount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISecurityObjectTypeInfo(pub ::windows::runtime::IUnknown);
impl ISecurityObjectTypeInfo {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_Foundation`*"]
    pub unsafe fn GetInheritSource(&self, si: u32, pacl: *mut super::super::ACL, ppinheritarray: *mut *mut super::INHERITED_FROMA) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(si), ::core::mem::transmute(pacl), ::core::mem::transmute(ppinheritarray)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISecurityObjectTypeInfo {
    type Vtable = ISecurityObjectTypeInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4231030507, 31215, 17483, [145, 17, 209, 138, 117, 235, 242, 250]);
}
impl ::core::convert::From<ISecurityObjectTypeInfo> for ::windows::runtime::IUnknown {
    fn from(value: ISecurityObjectTypeInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISecurityObjectTypeInfo> for ::windows::runtime::IUnknown {
    fn from(value: &ISecurityObjectTypeInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISecurityObjectTypeInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISecurityObjectTypeInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityObjectTypeInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, si: u32, pacl: *mut super::super::ACL, ppinheritarray: *mut *mut super::INHERITED_FROMA) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SECURITY_INFO_PAGE_FLAGS(pub u32);
pub const SI_ADVANCED: SECURITY_INFO_PAGE_FLAGS = SECURITY_INFO_PAGE_FLAGS(16u32);
pub const SI_EDIT_AUDITS: SECURITY_INFO_PAGE_FLAGS = SECURITY_INFO_PAGE_FLAGS(2u32);
pub const SI_EDIT_PROPERTIES: SECURITY_INFO_PAGE_FLAGS = SECURITY_INFO_PAGE_FLAGS(128u32);
impl ::core::convert::From<u32> for SECURITY_INFO_PAGE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SECURITY_INFO_PAGE_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for SECURITY_INFO_PAGE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for SECURITY_INFO_PAGE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for SECURITY_INFO_PAGE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for SECURITY_INFO_PAGE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for SECURITY_INFO_PAGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_Foundation`*"]
pub struct SECURITY_OBJECT {
    pub pwszName: super::super::super::Foundation::PWSTR,
    pub pData: *mut ::core::ffi::c_void,
    pub cbData: u32,
    pub pData2: *mut ::core::ffi::c_void,
    pub cbData2: u32,
    pub Id: u32,
    pub fWellKnown: super::super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl SECURITY_OBJECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SECURITY_OBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SECURITY_OBJECT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SECURITY_OBJECT").field("pwszName", &self.pwszName).field("pData", &self.pData).field("cbData", &self.cbData).field("pData2", &self.pData2).field("cbData2", &self.cbData2).field("Id", &self.Id).field("fWellKnown", &self.fWellKnown).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SECURITY_OBJECT {
    fn eq(&self, other: &Self) -> bool {
        self.pwszName == other.pwszName && self.pData == other.pData && self.cbData == other.cbData && self.pData2 == other.pData2 && self.cbData2 == other.cbData2 && self.Id == other.Id && self.fWellKnown == other.fWellKnown
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SECURITY_OBJECT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SECURITY_OBJECT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SECURITY_OBJECT_ID_CENTRAL_ACCESS_RULE: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SECURITY_OBJECT_ID_CENTRAL_POLICY: u32 = 3u32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SECURITY_OBJECT_ID_OBJECT_SD: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SECURITY_OBJECT_ID_SHARE: u32 = 2u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_Foundation`*"]
pub struct SID_INFO {
    pub pSid: super::super::super::Foundation::PSID,
    pub pwzCommonName: super::super::super::Foundation::PWSTR,
    pub pwzClass: super::super::super::Foundation::PWSTR,
    pub pwzUPN: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl SID_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SID_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SID_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SID_INFO").field("pSid", &self.pSid).field("pwzCommonName", &self.pwzCommonName).field("pwzClass", &self.pwzClass).field("pwzUPN", &self.pwzUPN).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SID_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pSid == other.pSid && self.pwzCommonName == other.pwzCommonName && self.pwzClass == other.pwzClass && self.pwzUPN == other.pwzUPN
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SID_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SID_INFO {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_Foundation`*"]
pub struct SID_INFO_LIST {
    pub cItems: u32,
    pub aSidInfo: [SID_INFO; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl SID_INFO_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SID_INFO_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SID_INFO_LIST {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SID_INFO_LIST").field("cItems", &self.cItems).field("aSidInfo", &self.aSidInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SID_INFO_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.cItems == other.cItems && self.aSidInfo == other.aSidInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SID_INFO_LIST {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SID_INFO_LIST {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_Foundation`*"]
pub struct SI_ACCESS {
    pub pguid: *mut ::windows::runtime::GUID,
    pub mask: u32,
    pub pszName: super::super::super::Foundation::PWSTR,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl SI_ACCESS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SI_ACCESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SI_ACCESS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SI_ACCESS").field("pguid", &self.pguid).field("mask", &self.mask).field("pszName", &self.pszName).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SI_ACCESS {
    fn eq(&self, other: &Self) -> bool {
        self.pguid == other.pguid && self.mask == other.mask && self.pszName == other.pszName && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SI_ACCESS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SI_ACCESS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_ACCESS_CONTAINER: i32 = 262144i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_ACCESS_GENERAL: i32 = 131072i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_ACCESS_PROPERTY: i32 = 524288i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_ACCESS_SPECIFIC: i32 = 65536i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_CONTAINER: i32 = 4i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_EDIT_OWNER: i32 = 1i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_EDIT_PERMS: i32 = 0i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_Foundation`*"]
pub struct SI_INHERIT_TYPE {
    pub pguid: *mut ::windows::runtime::GUID,
    pub dwFlags: super::super::ACE_FLAGS,
    pub pszName: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl SI_INHERIT_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SI_INHERIT_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SI_INHERIT_TYPE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SI_INHERIT_TYPE").field("pguid", &self.pguid).field("dwFlags", &self.dwFlags).field("pszName", &self.pszName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SI_INHERIT_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.pguid == other.pguid && self.dwFlags == other.dwFlags && self.pszName == other.pszName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SI_INHERIT_TYPE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SI_INHERIT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_NO_ACL_PROTECT: i32 = 512i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_NO_TREE_APPLY: i32 = 1024i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_OBJECT_GUID: i32 = 65536i32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_Foundation`*"]
pub struct SI_OBJECT_INFO {
    pub dwFlags: SI_OBJECT_INFO_FLAGS,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub pszServerName: super::super::super::Foundation::PWSTR,
    pub pszObjectName: super::super::super::Foundation::PWSTR,
    pub pszPageTitle: super::super::super::Foundation::PWSTR,
    pub guidObjectType: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl SI_OBJECT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SI_OBJECT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SI_OBJECT_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SI_OBJECT_INFO").field("dwFlags", &self.dwFlags).field("hInstance", &self.hInstance).field("pszServerName", &self.pszServerName).field("pszObjectName", &self.pszObjectName).field("pszPageTitle", &self.pszPageTitle).field("guidObjectType", &self.guidObjectType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SI_OBJECT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.hInstance == other.hInstance && self.pszServerName == other.pszServerName && self.pszObjectName == other.pszObjectName && self.pszPageTitle == other.pszPageTitle && self.guidObjectType == other.guidObjectType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SI_OBJECT_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SI_OBJECT_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SI_OBJECT_INFO_FLAGS(pub u32);
pub const SI_AUDITS_ELEVATION_REQUIRED: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(33554432u32);
pub const SI_DISABLE_DENY_ACE: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(2147483648u32);
pub const SI_EDIT_EFFECTIVE: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(131072u32);
pub const SI_ENABLE_CENTRAL_POLICY: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(1073741824u32);
pub const SI_ENABLE_EDIT_ATTRIBUTE_CONDITION: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(536870912u32);
pub const SI_MAY_WRITE: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(268435456u32);
pub const SI_NO_ADDITIONAL_PERMISSION: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(2097152u32);
pub const SI_OWNER_ELEVATION_REQUIRED: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(67108864u32);
pub const SI_PERMS_ELEVATION_REQUIRED: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(16777216u32);
pub const SI_RESET_DACL: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(262144u32);
pub const SI_RESET_OWNER: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(1048576u32);
pub const SI_RESET_SACL: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(524288u32);
pub const SI_SCOPE_ELEVATION_REQUIRED: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(134217728u32);
pub const SI_VIEW_ONLY: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(4194304u32);
impl ::core::convert::From<u32> for SI_OBJECT_INFO_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SI_OBJECT_INFO_FLAGS {
    type Abi = Self;
}
impl ::core::ops::BitOr for SI_OBJECT_INFO_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for SI_OBJECT_INFO_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for SI_OBJECT_INFO_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for SI_OBJECT_INFO_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for SI_OBJECT_INFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_OWNER_READONLY: i32 = 64i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_OWNER_RECURSE: i32 = 256i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SI_PAGE_ACTIVATED(pub i32);
pub const SI_SHOW_DEFAULT: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(0i32);
pub const SI_SHOW_PERM_ACTIVATED: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(1i32);
pub const SI_SHOW_AUDIT_ACTIVATED: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(2i32);
pub const SI_SHOW_OWNER_ACTIVATED: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(3i32);
pub const SI_SHOW_EFFECTIVE_ACTIVATED: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(4i32);
pub const SI_SHOW_SHARE_ACTIVATED: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(5i32);
pub const SI_SHOW_CENTRAL_POLICY_ACTIVATED: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(6i32);
impl ::core::convert::From<i32> for SI_PAGE_ACTIVATED {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SI_PAGE_ACTIVATED {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_PAGE_TITLE: i32 = 2048i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SI_PAGE_TYPE(pub i32);
pub const SI_PAGE_PERM: SI_PAGE_TYPE = SI_PAGE_TYPE(0i32);
pub const SI_PAGE_ADVPERM: SI_PAGE_TYPE = SI_PAGE_TYPE(1i32);
pub const SI_PAGE_AUDIT: SI_PAGE_TYPE = SI_PAGE_TYPE(2i32);
pub const SI_PAGE_OWNER: SI_PAGE_TYPE = SI_PAGE_TYPE(3i32);
pub const SI_PAGE_EFFECTIVE: SI_PAGE_TYPE = SI_PAGE_TYPE(4i32);
pub const SI_PAGE_TAKEOWNERSHIP: SI_PAGE_TYPE = SI_PAGE_TYPE(5i32);
pub const SI_PAGE_SHARE: SI_PAGE_TYPE = SI_PAGE_TYPE(6i32);
impl ::core::convert::From<i32> for SI_PAGE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SI_PAGE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_READONLY: i32 = 8i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_RESET: i32 = 32i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_RESET_DACL_TREE: i32 = 16384i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_RESET_SACL_TREE: i32 = 32768i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_SERVER_IS_DC: i32 = 4096i32;