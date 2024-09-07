// Generated by `wit-bindgen` 0.25.0. DO NOT EDIT!
// Options used:
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod component {
        #[allow(dead_code)]
        pub mod node {
            #[allow(dead_code, clippy::all)]
            pub mod types {
                #[used]
                #[doc(hidden)]
                #[cfg(target_arch = "wasm32")]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[repr(u8)]
                #[derive(Clone, Copy, Eq, PartialEq)]
                pub enum Error {
                    NetworkError,
                    TbdexError,
                    NoTbdx,
                }
                impl Error {
                    pub fn name(&self) -> &'static str {
                        match self {
                            Error::NetworkError => "network-error",
                            Error::TbdexError => "tbdex-error",
                            Error::NoTbdx => "no-tbdx",
                        }
                    }
                    pub fn message(&self) -> &'static str {
                        match self {
                            Error::NetworkError => "",
                            Error::TbdexError => "",
                            Error::NoTbdx => "",
                        }
                    }
                }
                impl ::core::fmt::Debug for Error {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.debug_struct("Error")
                            .field("code", &(*self as i32))
                            .field("name", &self.name())
                            .field("message", &self.message())
                            .finish()
                    }
                }
                impl ::core::fmt::Display for Error {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        write!(f, "{} (error {})", self.name(), *self as i32)
                    }
                }

                impl std::error::Error for Error {}

                impl Error {
                    #[doc(hidden)]
                    pub unsafe fn _lift(val: u8) -> Error {
                        if !cfg!(debug_assertions) {
                            return ::core::mem::transmute(val);
                        }

                        match val {
                            0 => Error::NetworkError,
                            1 => Error::TbdexError,
                            2 => Error::NoTbdx,

                            _ => panic!("invalid enum discriminant"),
                        }
                    }
                }

                #[derive(Clone)]
                pub struct SocketAddress {
                    pub ip: _rt::String,
                    pub port: u16,
                }
                impl ::core::fmt::Debug for SocketAddress {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.debug_struct("SocketAddress")
                            .field("ip", &self.ip)
                            .field("port", &self.port)
                            .finish()
                    }
                }
                #[repr(u8)]
                #[derive(Clone, Copy, Eq, PartialEq)]
                pub enum BitcoinNetwork {
                    Mainnet,
                    Testnet,
                    Regtest,
                }
                impl ::core::fmt::Debug for BitcoinNetwork {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        match self {
                            BitcoinNetwork::Mainnet => {
                                f.debug_tuple("BitcoinNetwork::Mainnet").finish()
                            }
                            BitcoinNetwork::Testnet => {
                                f.debug_tuple("BitcoinNetwork::Testnet").finish()
                            }
                            BitcoinNetwork::Regtest => {
                                f.debug_tuple("BitcoinNetwork::Regtest").finish()
                            }
                        }
                    }
                }

                impl BitcoinNetwork {
                    #[doc(hidden)]
                    pub unsafe fn _lift(val: u8) -> BitcoinNetwork {
                        if !cfg!(debug_assertions) {
                            return ::core::mem::transmute(val);
                        }

                        match val {
                            0 => BitcoinNetwork::Mainnet,
                            1 => BitcoinNetwork::Testnet,
                            2 => BitcoinNetwork::Regtest,

                            _ => panic!("invalid enum discriminant"),
                        }
                    }
                }

                #[derive(Clone)]
                pub struct OfferingBargain {
                    pub fee: Option<_rt::String>,
                    pub estimated_settlement_time: u64,
                    pub id: _rt::String,
                    pub rate: _rt::String,
                }
                impl ::core::fmt::Debug for OfferingBargain {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.debug_struct("OfferingBargain")
                            .field("fee", &self.fee)
                            .field("estimated-settlement-time", &self.estimated_settlement_time)
                            .field("id", &self.id)
                            .field("rate", &self.rate)
                            .finish()
                    }
                }
                #[derive(Clone)]
                pub struct NodeConfig {
                    pub wallet_address: _rt::String,
                    pub wallet_filter: _rt::String,
                    pub genesis_blockhash: _rt::String,
                    pub network: BitcoinNetwork,
                    pub socket_address: SocketAddress,
                }
                impl ::core::fmt::Debug for NodeConfig {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.debug_struct("NodeConfig")
                            .field("wallet-address", &self.wallet_address)
                            .field("wallet-filter", &self.wallet_filter)
                            .field("genesis-blockhash", &self.genesis_blockhash)
                            .field("network", &self.network)
                            .field("socket-address", &self.socket_address)
                            .finish()
                    }
                }
                #[derive(Clone)]
                pub struct TbdexConfig {
                    pub pfi_uri: _rt::String,
                    pub vc_url: _rt::String,
                    pub acct_number: _rt::String,
                }
                impl ::core::fmt::Debug for TbdexConfig {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.debug_struct("TbdexConfig")
                            .field("pfi-uri", &self.pfi_uri)
                            .field("vc-url", &self.vc_url)
                            .field("acct-number", &self.acct_number)
                            .finish()
                    }
                }

                #[derive(Debug)]
                #[repr(transparent)]
                pub struct ClientNode {
                    handle: _rt::Resource<ClientNode>,
                }

                type _ClientNodeRep<T> = Option<T>;

                impl ClientNode {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `ClientNode`.
                    pub fn new<T: GuestClientNode>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _ClientNodeRep<T> = Some(val);
                        let ptr: *mut _ClientNodeRep<T> = _rt::Box::into_raw(_rt::Box::new(val));
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }

                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestClientNode>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }

                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestClientNode>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }

                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestClientNode>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }

                    #[doc(hidden)]
                    pub unsafe fn from_handle(handle: u32) -> Self {
                        Self {
                            handle: _rt::Resource::from_handle(handle),
                        }
                    }

                    #[doc(hidden)]
                    pub fn take_handle(&self) -> u32 {
                        _rt::Resource::take_handle(&self.handle)
                    }

                    #[doc(hidden)]
                    pub fn handle(&self) -> u32 {
                        _rt::Resource::handle(&self.handle)
                    }

                    // It's theoretically possible to implement the `GuestClientNode` trait twice
                    // so guard against using it with two different types here.
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(!cfg!(target_feature = "threads"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => assert!(
                                    ty == id,
                                    "cannot use two types with this resource type"
                                ),
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }

                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(handle as *mut _ClientNodeRep<T>);
                    }

                    fn as_ptr<T: GuestClientNode>(&self) -> *mut _ClientNodeRep<T> {
                        ClientNode::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }

                /// A borrowed version of [`ClientNode`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct ClientNodeBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a ClientNode>,
                }

                impl<'a> ClientNodeBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }

                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestClientNode>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }

                    // NB: mutable access is not allowed due to the component model allowing
                    // multiple borrows of the same resource.

                    fn as_ptr<T: 'static>(&self) -> *mut _ClientNodeRep<T> {
                        ClientNode::type_guard::<T>();
                        self.rep.cast()
                    }
                }

                unsafe impl _rt::WasmResource for ClientNode {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();

                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]component:node/types@0.1.0")]
                            extern "C" {
                                #[link_name = "[resource-drop]client-node"]
                                fn drop(_: u32);
                            }

                            drop(_handle);
                        }
                    }
                }

                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_constructor_client_node_cabi<T: GuestClientNode>(
                    arg0: *mut u8,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let len2 = l1;
                    let bytes2 = _rt::Vec::from_raw_parts(l0.cast(), len2, len2);
                    let l3 = *arg0.add(8).cast::<*mut u8>();
                    let l4 = *arg0.add(12).cast::<usize>();
                    let len5 = l4;
                    let bytes5 = _rt::Vec::from_raw_parts(l3.cast(), len5, len5);
                    let l6 = *arg0.add(16).cast::<*mut u8>();
                    let l7 = *arg0.add(20).cast::<usize>();
                    let len8 = l7;
                    let bytes8 = _rt::Vec::from_raw_parts(l6.cast(), len8, len8);
                    let l9 = i32::from(*arg0.add(24).cast::<u8>());
                    let l10 = *arg0.add(28).cast::<*mut u8>();
                    let l11 = *arg0.add(32).cast::<usize>();
                    let len12 = l11;
                    let bytes12 = _rt::Vec::from_raw_parts(l10.cast(), len12, len12);
                    let l13 = i32::from(*arg0.add(36).cast::<u16>());
                    let l14 = i32::from(*arg0.add(40).cast::<u8>());
                    let result24 = ClientNode::new(T::new(
                        NodeConfig {
                            wallet_address: _rt::string_lift(bytes2),
                            wallet_filter: _rt::string_lift(bytes5),
                            genesis_blockhash: _rt::string_lift(bytes8),
                            network: BitcoinNetwork::_lift(l9 as u8),
                            socket_address: SocketAddress {
                                ip: _rt::string_lift(bytes12),
                                port: l13 as u16,
                            },
                        },
                        match l14 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l15 = *arg0.add(44).cast::<*mut u8>();
                                    let l16 = *arg0.add(48).cast::<usize>();
                                    let len17 = l16;
                                    let bytes17 =
                                        _rt::Vec::from_raw_parts(l15.cast(), len17, len17);
                                    let l18 = *arg0.add(52).cast::<*mut u8>();
                                    let l19 = *arg0.add(56).cast::<usize>();
                                    let len20 = l19;
                                    let bytes20 =
                                        _rt::Vec::from_raw_parts(l18.cast(), len20, len20);
                                    let l21 = *arg0.add(60).cast::<*mut u8>();
                                    let l22 = *arg0.add(64).cast::<usize>();
                                    let len23 = l22;
                                    let bytes23 =
                                        _rt::Vec::from_raw_parts(l21.cast(), len23, len23);

                                    TbdexConfig {
                                        pfi_uri: _rt::string_lift(bytes17),
                                        vc_url: _rt::string_lift(bytes20),
                                        acct_number: _rt::string_lift(bytes23),
                                    }
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        },
                    ));
                    _rt::cabi_dealloc(arg0, 68, 4);
                    (result24).take_handle() as i32
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_client_node_get_balance_cabi<T: GuestClientNode>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let result0 =
                        T::get_balance(ClientNodeBorrow::lift(arg0 as u32 as usize).get());
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Ok(e) => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                            *ptr1.add(8).cast::<i64>() = _rt::as_i64(e);
                        }
                        Err(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            *ptr1.add(8).cast::<u8>() = (e.clone() as i32) as u8;
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_client_node_get_conversion_offer_cabi<
                    T: GuestClientNode,
                >(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let result0 =
                        T::get_conversion_offer(ClientNodeBorrow::lift(arg0 as u32 as usize).get());
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Ok(e) => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                            let OfferingBargain {
                                fee: fee2,
                                estimated_settlement_time: estimated_settlement_time2,
                                id: id2,
                                rate: rate2,
                            } = e;
                            match fee2 {
                                Some(e) => {
                                    *ptr1.add(8).cast::<u8>() = (1i32) as u8;
                                    let vec3 = (e.into_bytes()).into_boxed_slice();
                                    let ptr3 = vec3.as_ptr().cast::<u8>();
                                    let len3 = vec3.len();
                                    ::core::mem::forget(vec3);
                                    *ptr1.add(16).cast::<usize>() = len3;
                                    *ptr1.add(12).cast::<*mut u8>() = ptr3.cast_mut();
                                }
                                None => {
                                    *ptr1.add(8).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            *ptr1.add(24).cast::<i64>() = _rt::as_i64(estimated_settlement_time2);
                            let vec4 = (id2.into_bytes()).into_boxed_slice();
                            let ptr4 = vec4.as_ptr().cast::<u8>();
                            let len4 = vec4.len();
                            ::core::mem::forget(vec4);
                            *ptr1.add(36).cast::<usize>() = len4;
                            *ptr1.add(32).cast::<*mut u8>() = ptr4.cast_mut();
                            let vec5 = (rate2.into_bytes()).into_boxed_slice();
                            let ptr5 = vec5.as_ptr().cast::<u8>();
                            let len5 = vec5.len();
                            ::core::mem::forget(vec5);
                            *ptr1.add(44).cast::<usize>() = len5;
                            *ptr1.add(40).cast::<*mut u8>() = ptr5.cast_mut();
                        }
                        Err(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            *ptr1.add(8).cast::<u8>() = (e.clone() as i32) as u8;
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_client_node_get_conversion_offer<
                    T: GuestClientNode,
                >(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = i32::from(*arg0.add(8).cast::<u8>());
                            match l1 {
                                0 => (),
                                _ => {
                                    let l2 = *arg0.add(12).cast::<*mut u8>();
                                    let l3 = *arg0.add(16).cast::<usize>();
                                    _rt::cabi_dealloc(l2, l3, 1);
                                }
                            }
                            let l4 = *arg0.add(32).cast::<*mut u8>();
                            let l5 = *arg0.add(36).cast::<usize>();
                            _rt::cabi_dealloc(l4, l5, 1);
                            let l6 = *arg0.add(40).cast::<*mut u8>();
                            let l7 = *arg0.add(44).cast::<usize>();
                            _rt::cabi_dealloc(l6, l7, 1);
                        }
                        _ => (),
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_client_node_convert_amount_cabi<T: GuestClientNode>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: *mut u8,
                    arg4: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let len1 = arg4;
                    let bytes1 = _rt::Vec::from_raw_parts(arg3.cast(), len1, len1);
                    let result2 = T::convert_amount(
                        ClientNodeBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::string_lift(bytes0),
                        _rt::string_lift(bytes1),
                    );
                    let ptr3 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result2 {
                        Ok(e) => {
                            *ptr3.add(0).cast::<u8>() = (0i32) as u8;
                            let vec4 = (e.into_bytes()).into_boxed_slice();
                            let ptr4 = vec4.as_ptr().cast::<u8>();
                            let len4 = vec4.len();
                            ::core::mem::forget(vec4);
                            *ptr3.add(8).cast::<usize>() = len4;
                            *ptr3.add(4).cast::<*mut u8>() = ptr4.cast_mut();
                        }
                        Err(e) => {
                            *ptr3.add(0).cast::<u8>() = (1i32) as u8;
                            *ptr3.add(4).cast::<u8>() = (e.clone() as i32) as u8;
                        }
                    };
                    ptr3
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_client_node_convert_amount<
                    T: GuestClientNode,
                >(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                        _ => (),
                    }
                }
                pub trait Guest {
                    type ClientNode: GuestClientNode;
                }
                pub trait GuestClientNode: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }

                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]component:node/types@0.1.0")]
                            extern "C" {
                                #[link_name = "[resource-new]client-node"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }

                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }

                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]component:node/types@0.1.0")]
                            extern "C" {
                                #[link_name = "[resource-rep]client-node"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }

                    fn new(config: NodeConfig, tbdex_config: Option<TbdexConfig>) -> Self;
                    fn get_balance(&self) -> Result<i64, Error>;
                    fn get_conversion_offer(&self) -> Result<OfferingBargain, Error>;
                    fn convert_amount(
                        &self,
                        amount: _rt::String,
                        offer_id: _rt::String,
                    ) -> Result<_rt::String, Error>;
                }
                #[doc(hidden)]

                macro_rules! __export_component_node_types_0_1_0_cabi{
  ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

    #[export_name = "component:node/types@0.1.0#[constructor]client-node"]
    unsafe extern "C" fn export_constructor_client_node(arg0: *mut u8,) -> i32 {
      $($path_to_types)*::_export_constructor_client_node_cabi::<<$ty as $($path_to_types)*::Guest>::ClientNode>(arg0)
    }
    #[export_name = "component:node/types@0.1.0#[method]client-node.get-balance"]
    unsafe extern "C" fn export_method_client_node_get_balance(arg0: *mut u8,) -> *mut u8 {
      $($path_to_types)*::_export_method_client_node_get_balance_cabi::<<$ty as $($path_to_types)*::Guest>::ClientNode>(arg0)
    }
    #[export_name = "component:node/types@0.1.0#[method]client-node.get-conversion-offer"]
    unsafe extern "C" fn export_method_client_node_get_conversion_offer(arg0: *mut u8,) -> *mut u8 {
      $($path_to_types)*::_export_method_client_node_get_conversion_offer_cabi::<<$ty as $($path_to_types)*::Guest>::ClientNode>(arg0)
    }
    #[export_name = "cabi_post_component:node/types@0.1.0#[method]client-node.get-conversion-offer"]
    unsafe extern "C" fn _post_return_method_client_node_get_conversion_offer(arg0: *mut u8,) {
      $($path_to_types)*::__post_return_method_client_node_get_conversion_offer::<<$ty as $($path_to_types)*::Guest>::ClientNode>(arg0)
    }
    #[export_name = "component:node/types@0.1.0#[method]client-node.convert-amount"]
    unsafe extern "C" fn export_method_client_node_convert_amount(arg0: *mut u8,arg1: *mut u8,arg2: usize,arg3: *mut u8,arg4: usize,) -> *mut u8 {
      $($path_to_types)*::_export_method_client_node_convert_amount_cabi::<<$ty as $($path_to_types)*::Guest>::ClientNode>(arg0, arg1, arg2, arg3, arg4)
    }
    #[export_name = "cabi_post_component:node/types@0.1.0#[method]client-node.convert-amount"]
    unsafe extern "C" fn _post_return_method_client_node_convert_amount(arg0: *mut u8,) {
      $($path_to_types)*::__post_return_method_client_node_convert_amount::<<$ty as $($path_to_types)*::Guest>::ClientNode>(arg0)
    }

    const _: () = {
      #[doc(hidden)]
      #[export_name = "component:node/types@0.1.0#[dtor]client-node"]
      #[allow(non_snake_case)]
      unsafe extern "C" fn dtor(rep: *mut u8) {
        $($path_to_types)*::ClientNode::dtor::<
        <$ty as $($path_to_types)*::Guest>::ClientNode
        >(rep)
      }
    };

  };);
}
                #[doc(hidden)]
                pub(crate) use __export_component_node_types_0_1_0_cabi;
                #[repr(align(8))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 48]);
                static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 48]);
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::string::String;

    use core::fmt;
    use core::marker;
    use core::sync::atomic::{AtomicU32, Ordering::Relaxed};

    /// A type which represents a component model resource, either imported or
    /// exported into this component.
    ///
    /// This is a low-level wrapper which handles the lifetime of the resource
    /// (namely this has a destructor). The `T` provided defines the component model
    /// intrinsics that this wrapper uses.
    ///
    /// One of the chief purposes of this type is to provide `Deref` implementations
    /// to access the underlying data when it is owned.
    ///
    /// This type is primarily used in generated code for exported and imported
    /// resources.
    #[repr(transparent)]
    pub struct Resource<T: WasmResource> {
        // NB: This would ideally be `u32` but it is not. The fact that this has
        // interior mutability is not exposed in the API of this type except for the
        // `take_handle` method which is supposed to in theory be private.
        //
        // This represents, almost all the time, a valid handle value. When it's
        // invalid it's stored as `u32::MAX`.
        handle: AtomicU32,
        _marker: marker::PhantomData<T>,
    }

    /// A trait which all wasm resources implement, namely providing the ability to
    /// drop a resource.
    ///
    /// This generally is implemented by generated code, not user-facing code.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait WasmResource {
        /// Invokes the `[resource-drop]...` intrinsic.
        unsafe fn drop(handle: u32);
    }

    impl<T: WasmResource> Resource<T> {
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
            debug_assert!(handle != u32::MAX);
            Self {
                handle: AtomicU32::new(handle),
                _marker: marker::PhantomData,
            }
        }

        /// Takes ownership of the handle owned by `resource`.
        ///
        /// Note that this ideally would be `into_handle` taking `Resource<T>` by
        /// ownership. The code generator does not enable that in all situations,
        /// unfortunately, so this is provided instead.
        ///
        /// Also note that `take_handle` is in theory only ever called on values
        /// owned by a generated function. For example a generated function might
        /// take `Resource<T>` as an argument but then call `take_handle` on a
        /// reference to that argument. In that sense the dynamic nature of
        /// `take_handle` should only be exposed internally to generated code, not
        /// to user code.
        #[doc(hidden)]
        pub fn take_handle(resource: &Resource<T>) -> u32 {
            resource.handle.swap(u32::MAX, Relaxed)
        }

        #[doc(hidden)]
        pub fn handle(resource: &Resource<T>) -> u32 {
            resource.handle.load(Relaxed)
        }
    }

    impl<T: WasmResource> fmt::Debug for Resource<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resource")
                .field("handle", &self.handle)
                .finish()
        }
    }

    impl<T: WasmResource> Drop for Resource<T> {
        fn drop(&mut self) {
            unsafe {
                match self.handle.load(Relaxed) {
                    // If this handle was "taken" then don't do anything in the
                    // destructor.
                    u32::MAX => {}

                    // ... but otherwise do actually destroy it with the imported
                    // component model intrinsic as defined through `T`.
                    other => T::drop(other),
                }
            }
        }
    }
    pub use alloc_crate::boxed::Box;

    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr as *mut u8, layout);
    }

    pub fn as_i64<T: AsI64>(t: T) -> i64 {
        t.as_i64()
    }

    pub trait AsI64 {
        fn as_i64(self) -> i64;
    }

    impl<'a, T: Copy + AsI64> AsI64 for &'a T {
        fn as_i64(self) -> i64 {
            (*self).as_i64()
        }
    }

    impl AsI64 for i64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }

    impl AsI64 for u64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
}

/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]

macro_rules! __export_nodeworld_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::exports::component::node::types::__export_component_node_types_0_1_0_cabi!($ty with_types_in $($path_to_types_root)*::exports::component::node::types);
  )
}
#[doc(inline)]
pub(crate) use __export_nodeworld_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.25.0:nodeworld:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 853] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xd5\x05\x01A\x02\x01\
A\x02\x01B\x1e\x01r\x02\x03keys\x05values\x04\0\x0ekey-value-pair\x03\0\0\x01m\x03\
\x0dnetwork-error\x0btbdex-error\x07no-tbdx\x04\0\x05error\x03\0\x02\x01r\x02\x02\
ips\x04port{\x04\0\x0esocket-address\x03\0\x04\x01m\x03\x07mainnet\x07testnet\x07\
regtest\x04\0\x0fbitcoin-network\x03\0\x06\x01ks\x01r\x04\x03fee\x08\x19estimate\
d-settlement-timew\x02ids\x04rates\x04\0\x10offering-bargain\x03\0\x09\x01r\x05\x0e\
wallet-addresss\x0dwallet-filters\x11genesis-blockhashs\x07network\x07\x0esocket\
-address\x05\x04\0\x0bnode-config\x03\0\x0b\x01r\x03\x07pfi-uris\x06vc-urls\x0ba\
cct-numbers\x04\0\x0ctbdex-config\x03\0\x0d\x04\0\x0bclient-node\x03\x01\x01k\x0e\
\x01i\x0f\x01@\x02\x06config\x0c\x0ctbdex-config\x10\0\x11\x04\0\x18[constructor\
]client-node\x01\x12\x01h\x0f\x01j\x01x\x01\x03\x01@\x01\x04self\x13\0\x14\x04\0\
\x1f[method]client-node.get-balance\x01\x15\x01j\x01\x0a\x01\x03\x01@\x01\x04sel\
f\x13\0\x16\x04\0([method]client-node.get-conversion-offer\x01\x17\x01j\x01s\x01\
\x03\x01@\x03\x04self\x13\x06amounts\x08offer-ids\0\x18\x04\0\"[method]client-no\
de.convert-amount\x01\x19\x04\x01\x1acomponent:node/types@0.1.0\x05\0\x04\x01\x1e\
component:node/nodeworld@0.1.0\x04\0\x0b\x0f\x01\0\x09nodeworld\x03\0\0\0G\x09pr\
oducers\x01\x0cprocessed-by\x02\x0dwit-component\x070.208.1\x10wit-bindgen-rust\x06\
0.25.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
