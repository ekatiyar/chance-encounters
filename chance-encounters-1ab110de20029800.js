let T=`undefined`,Z=`string`,a1=4,S=1,a0=16,a4=64,Y=`function`,_=`Object`,R=null,W=0,U=`utf-8`,P=Array,V=Error,$=FinalizationRegistry,a3=Promise,a2=Reflect,X=Uint8Array,Q=undefined;function B(b,c){try{return b.apply(this,c)}catch(b){a.__wbindgen_exn_store(e(b))}}var c=(a=>b[a]);var g=(a=>{const b=c(a);f(a);return b});var L=((a,b)=>{});var w=((b,c,d,e)=>{const f={a:b,b:c,cnt:S,dtor:d};const g=(...b)=>{f.cnt++;const c=f.a;f.a=W;try{return e(c,f.b,...b)}finally{if(--f.cnt===W){a.__wbindgen_export_2.get(f.dtor)(c,f.b);t.unregister(f)}else{f.a=c}}};g.original=f;t.register(g,f,f);return g});var x=((b,c,d)=>{a._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h8e9ba022b3bc4304(b,c,e(d))});var s=(a=>{const b=typeof a;if(b==`number`||b==`boolean`||a==R){return `${a}`};if(b==Z){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==R){return `Symbol`}else{return `Symbol(${b})`}};if(b==Y){const b=a.name;if(typeof b==Z&&b.length>W){return `Function(${b})`}else{return `Function`}};if(P.isArray(a)){const b=a.length;let c=`[`;if(b>W){c+=s(a[W])};for(let d=S;d<b;d++){c+=`, `+ s(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>S){d=c[S]}else{return toString.call(a)};if(d==_){try{return `Object(`+ JSON.stringify(a)+ `)`}catch(a){return _}};if(a instanceof V){return `${a.name}: ${a.message}\n${a.stack}`};return d});var r=(()=>{if(q===R||q.byteLength===W){q=new Int32Array(a.memory.buffer)};return q});var p=(a=>a===Q||a===R);var f=(a=>{if(a<132)return;b[a]=d;d=a});var e=(a=>{if(d===b.length)b.push(b.length+ S);const c=d;d=b[c];b[c]=a;return c});var k=((a,b)=>{a=a>>>W;return h.decode(j().subarray(a,a+ b))});var j=(()=>{if(i===R||i.byteLength===W){i=new X(a.memory.buffer)};return i});var C=((b,c,d,f)=>{a.wasm_bindgen__convert__closures__invoke2_mut__h14b1dd8c05bf8b23(b,c,e(d),e(f))});var o=((a,b,c)=>{if(c===Q){const c=m.encode(a);const d=b(c.length,S)>>>W;j().subarray(d,d+ c.length).set(c);l=c.length;return d};let d=a.length;let e=b(d,S)>>>W;const f=j();let g=W;for(;g<d;g++){const b=a.charCodeAt(g);if(b>127)break;f[e+ g]=b};if(g!==d){if(g!==W){a=a.slice(g)};e=c(e,d,d=g+ a.length*3,S)>>>W;const b=j().subarray(e+ g,e+ d);const f=n(a,b);g+=f.written;e=c(e,d,g,S)>>>W};l=g;return e});var z=((b,c,d)=>{a._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h308cc9f34acef4ff(b,c,e(d))});var K=(()=>{const b={};b.wbg={};b.wbg.__wbindgen_object_clone_ref=(a=>{const b=c(a);return e(b)});b.wbg.__wbindgen_object_drop_ref=(a=>{g(a)});b.wbg.__wbindgen_string_new=((a,b)=>{const c=k(a,b);return e(c)});b.wbg.__wbindgen_string_get=((b,d)=>{const e=c(d);const f=typeof e===Z?e:Q;var g=p(f)?W:o(f,a.__wbindgen_malloc,a.__wbindgen_realloc);var h=l;r()[b/a1+ S]=h;r()[b/a1+ W]=g});b.wbg.__wbg_new_abda76e883ba8a5f=(()=>{const a=new V();return e(a)});b.wbg.__wbg_stack_658279fe44541cf6=((b,d)=>{const e=c(d).stack;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a1+ S]=g;r()[b/a1+ W]=f});b.wbg.__wbg_error_f851667af71bcfc6=((b,c)=>{var d=A(b,c);if(b!==W){a.__wbindgen_free(b,c,S)};console.error(d)});b.wbg.__wbindgen_cb_drop=(a=>{const b=g(a).original;if(b.cnt--==S){b.a=W;return !0};const c=!1;return c});b.wbg.__wbindgen_is_undefined=(a=>{const b=c(a)===Q;return b});b.wbg.__wbindgen_is_null=(a=>{const b=c(a)===R;return b});b.wbg.__wbindgen_is_falsy=(a=>{const b=!c(a);return b});b.wbg.__wbg_instanceof_Window_f401953a2cf86220=(a=>{let b;try{b=c(a) instanceof Window}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_document_5100775d18896c16=(a=>{const b=c(a).document;return p(b)?W:e(b)});b.wbg.__wbg_body_edb1908d3ceff3a1=(a=>{const b=c(a).body;return p(b)?W:e(b)});b.wbg.__wbg_createComment_354ccab4fdc521ee=((a,b,d)=>{var f=A(b,d);const g=c(a).createComment(f);return e(g)});b.wbg.__wbg_createDocumentFragment_8c86903bbb0a3c3c=(a=>{const b=c(a).createDocumentFragment();return e(b)});b.wbg.__wbg_createElement_8bae7856a4bb7411=function(){return B(((a,b,d)=>{var f=A(b,d);const g=c(a).createElement(f);return e(g)}),arguments)};b.wbg.__wbg_createElementNS_556a62fb298be5a2=function(){return B(((a,b,d,f,g)=>{var h=A(b,d);var i=A(f,g);const j=c(a).createElementNS(h,i);return e(j)}),arguments)};b.wbg.__wbg_createTextNode_0c38fd80a5b2284d=((a,b,d)=>{var f=A(b,d);const g=c(a).createTextNode(f);return e(g)});b.wbg.__wbg_setinnerHTML_26d69b59e1af99c7=((a,b,d)=>{var e=A(b,d);c(a).innerHTML=e});b.wbg.__wbg_removeAttribute_1b10a06ae98ebbd1=function(){return B(((a,b,d)=>{var e=A(b,d);c(a).removeAttribute(e)}),arguments)};b.wbg.__wbg_setAttribute_3c9f6c303b696daa=function(){return B(((a,b,d,e,f)=>{var g=A(b,d);var h=A(e,f);c(a).setAttribute(g,h)}),arguments)};b.wbg.__wbg_before_210596e44d88649f=function(){return B(((a,b)=>{c(a).before(c(b))}),arguments)};b.wbg.__wbg_remove_49b0a5925a04b955=(a=>{c(a).remove()});b.wbg.__wbg_append_7ba9d5c2eb183eea=function(){return B(((a,b)=>{c(a).append(c(b))}),arguments)};b.wbg.__wbg_instanceof_ShadowRoot_9db040264422e84a=(a=>{let b;try{b=c(a) instanceof ShadowRoot}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_host_c667c7623404d6bf=(a=>{const b=c(a).host;return e(b)});b.wbg.__wbg_setdata_8c2b43af041cc1b3=((a,b,d)=>{var e=A(b,d);c(a).data=e});b.wbg.__wbg_addEventListener_53b787075bd5e003=function(){return B(((a,b,d,e)=>{var f=A(b,d);c(a).addEventListener(f,c(e))}),arguments)};b.wbg.__wbg_addEventListener_4283b15b4f039eb5=function(){return B(((a,b,d,e,f)=>{var g=A(b,d);c(a).addEventListener(g,c(e),c(f))}),arguments)};b.wbg.__wbg_view_7f0ce470793a340f=(a=>{const b=c(a).view;return p(b)?W:e(b)});b.wbg.__wbg_respond_b1a43b2e3a06d525=function(){return B(((a,b)=>{c(a).respond(b>>>W)}),arguments)};b.wbg.__wbg_files_8b6e6eff43af0f6d=(a=>{const b=c(a).files;return p(b)?W:e(b)});b.wbg.__wbg_value_47fe6384562f52ab=((b,d)=>{const e=c(d).value;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a1+ S]=g;r()[b/a1+ W]=f});b.wbg.__wbg_byobRequest_72fca99f9c32c193=(a=>{const b=c(a).byobRequest;return p(b)?W:e(b)});b.wbg.__wbg_close_184931724d961ccc=function(){return B((a=>{c(a).close()}),arguments)};b.wbg.__wbg_close_a994f9425dab445c=function(){return B((a=>{c(a).close()}),arguments)};b.wbg.__wbg_enqueue_ea194723156c0cc2=function(){return B(((a,b)=>{c(a).enqueue(c(b))}),arguments)};b.wbg.__wbg_target_2fc177e386c8b7b0=(a=>{const b=c(a).target;return p(b)?W:e(b)});b.wbg.__wbg_cancelBubble_c0aa3172524eb03c=(a=>{const b=c(a).cancelBubble;return b});b.wbg.__wbg_composedPath_58473fd5ae55f2cd=(a=>{const b=c(a).composedPath();return e(b)});b.wbg.__wbg_debug_5fb96680aecf5dc8=(a=>{console.debug(c(a))});b.wbg.__wbg_error_8e3928cfb8a43e2b=(a=>{console.error(c(a))});b.wbg.__wbg_info_530a29cb2e4e3304=(a=>{console.info(c(a))});b.wbg.__wbg_log_5bb5f88f245d7762=(a=>{console.log(c(a))});b.wbg.__wbg_warn_63bbae1730aead09=(a=>{console.warn(c(a))});b.wbg.__wbg_parentNode_6be3abff20e1a5fb=(a=>{const b=c(a).parentNode;return p(b)?W:e(b)});b.wbg.__wbg_childNodes_118168e8b23bcb9b=(a=>{const b=c(a).childNodes;return e(b)});b.wbg.__wbg_previousSibling_9708a091a3e6e03b=(a=>{const b=c(a).previousSibling;return p(b)?W:e(b)});b.wbg.__wbg_nextSibling_709614fdb0fb7a66=(a=>{const b=c(a).nextSibling;return p(b)?W:e(b)});b.wbg.__wbg_appendChild_580ccb11a660db68=function(){return B(((a,b)=>{const d=c(a).appendChild(c(b));return e(d)}),arguments)};b.wbg.__wbg_cloneNode_e19c313ea20d5d1d=function(){return B((a=>{const b=c(a).cloneNode();return e(b)}),arguments)};b.wbg.__wbg_length_d0a802565d17eec4=(a=>{const b=c(a).length;return b});b.wbg.__wbg_get_58f6d5f6aee3f846=((a,b)=>{const d=c(a)[b>>>W];return p(d)?W:e(d)});b.wbg.__wbg_readyState_f5192102c36cb272=(a=>{const b=c(a).readyState;return b});b.wbg.__wbg_result_77ceeec1e3a16df7=function(){return B((a=>{const b=c(a).result;return e(b)}),arguments)};b.wbg.__wbg_setonloadend_1a1d3155e6949495=((a,b)=>{c(a).onloadend=c(b)});b.wbg.__wbg_new_c1e4a76f0b5c28b8=function(){return B((()=>{const a=new FileReader();return e(a)}),arguments)};b.wbg.__wbg_readAsText_ac9afc9ae3f40e0a=function(){return B(((a,b)=>{c(a).readAsText(c(b))}),arguments)};b.wbg.__wbg_get_bd8e338fbd5f5cc8=((a,b)=>{const d=c(a)[b>>>W];return e(d)});b.wbg.__wbindgen_is_function=(a=>{const b=typeof c(a)===Y;return b});b.wbg.__wbg_newnoargs_e258087cd0daa0ea=((a,b)=>{var c=A(a,b);const d=new Function(c);return e(d)});b.wbg.__wbg_get_e3c254076557e348=function(){return B(((a,b)=>{const d=a2.get(c(a),c(b));return e(d)}),arguments)};b.wbg.__wbg_call_27c0f87801dedf93=function(){return B(((a,b)=>{const d=c(a).call(c(b));return e(d)}),arguments)};b.wbg.__wbg_self_ce0dbfc45cf2f5be=function(){return B((()=>{const a=self.self;return e(a)}),arguments)};b.wbg.__wbg_window_c6fb939a7f436783=function(){return B((()=>{const a=window.window;return e(a)}),arguments)};b.wbg.__wbg_globalThis_d1e6af4856ba331b=function(){return B((()=>{const a=globalThis.globalThis;return e(a)}),arguments)};b.wbg.__wbg_global_207b558942527489=function(){return B((()=>{const a=global.global;return e(a)}),arguments)};b.wbg.__wbg_new_28c511d9baebfa89=((a,b)=>{var c=A(a,b);const d=new V(c);return e(d)});b.wbg.__wbg_call_b3ca7c6051f9bec1=function(){return B(((a,b,d)=>{const f=c(a).call(c(b),c(d));return e(f)}),arguments)};b.wbg.__wbg_is_010fdc0f4ab96916=((a,b)=>{const d=Object.is(c(a),c(b));return d});b.wbg.__wbg_new_81740750da40724f=((a,b)=>{try{var c={a:a,b:b};var d=(a,b)=>{const d=c.a;c.a=W;try{return C(d,c.b,a,b)}finally{c.a=d}};const f=new a3(d);return e(f)}finally{c.a=c.b=W}});b.wbg.__wbg_resolve_b0083a7967828ec8=(a=>{const b=a3.resolve(c(a));return e(b)});b.wbg.__wbg_then_0c86a60e8fcfe9f6=((a,b)=>{const d=c(a).then(c(b));return e(d)});b.wbg.__wbg_buffer_12d079cc21e14bdb=(a=>{const b=c(a).buffer;return e(b)});b.wbg.__wbg_newwithbyteoffsetandlength_aa4a17c33a06e5cb=((a,b,d)=>{const f=new X(c(a),b>>>W,d>>>W);return e(f)});b.wbg.__wbg_set_a47bac70306a19a7=((a,b,d)=>{c(a).set(c(b),d>>>W)});b.wbg.__wbg_length_c20a40f15020d68a=(a=>{const b=c(a).length;return b});b.wbg.__wbg_buffer_dd7f74bc60f1faab=(a=>{const b=c(a).buffer;return e(b)});b.wbg.__wbg_byteLength_58f7b4fab1919d44=(a=>{const b=c(a).byteLength;return b});b.wbg.__wbg_byteOffset_81d60f7392524f62=(a=>{const b=c(a).byteOffset;return b});b.wbg.__wbg_set_1f9b04f170055d33=function(){return B(((a,b,d)=>{const e=a2.set(c(a),c(b),c(d));return e}),arguments)};b.wbg.__wbindgen_debug_string=((b,d)=>{const e=s(c(d));const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/a1+ S]=g;r()[b/a1+ W]=f});b.wbg.__wbindgen_throw=((a,b)=>{throw new V(k(a,b))});b.wbg.__wbindgen_rethrow=(a=>{throw g(a)});b.wbg.__wbindgen_memory=(()=>{const b=a.memory;return e(b)});b.wbg.__wbg_queueMicrotask_481971b0d87f3dd4=(a=>{queueMicrotask(c(a))});b.wbg.__wbg_queueMicrotask_3cbae2ec6b6cd3d6=(a=>{const b=c(a).queueMicrotask;return e(b)});b.wbg.__wbindgen_closure_wrapper199=((a,b,c)=>{const d=u(a,b,a4,v);return e(d)});b.wbg.__wbindgen_closure_wrapper201=((a,b,c)=>{const d=w(a,b,a4,x);return e(d)});b.wbg.__wbindgen_closure_wrapper460=((a,b,c)=>{const d=w(a,b,191,y);return e(d)});b.wbg.__wbindgen_closure_wrapper2709=((a,b,c)=>{const d=w(a,b,313,z);return e(d)});return b});var v=((b,c)=>{a._dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h3b5ee6c2ee868002(b,c)});var u=((b,c,d,e)=>{const f={a:b,b:c,cnt:S,dtor:d};const g=(...b)=>{f.cnt++;try{return e(f.a,f.b,...b)}finally{if(--f.cnt===W){a.__wbindgen_export_2.get(f.dtor)(f.a,f.b);f.a=W;t.unregister(f)}}};g.original=f;t.register(g,f,f);return g});var O=(async(b)=>{if(a!==Q)return a;if(typeof b===T){b=new URL(`chance-encounters_bg.wasm`,import.meta.url)};const c=K();if(typeof b===Z||typeof Request===Y&&b instanceof Request||typeof URL===Y&&b instanceof URL){b=fetch(b)};L(c);const {instance:d,module:e}=await J(await b,c);return M(d,e)});var A=((a,b)=>{if(a===W){return c(b)}else{return k(a,b)}});var J=(async(a,b)=>{if(typeof Response===Y&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===Y){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var M=((b,c)=>{a=b.exports;O.__wbindgen_wasm_module=c;q=R;i=R;a.__wbindgen_start();return a});var N=(b=>{if(a!==Q)return a;const c=K();L(c);if(!(b instanceof WebAssembly.Module)){b=new WebAssembly.Module(b)};const d=new WebAssembly.Instance(b,c);return M(d,b)});var y=((b,c,d)=>{a._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h9d53d3aea1e21ada(b,c,e(d))});let a;const b=new P(128).fill(Q);b.push(Q,R,!0,!1);let d=b.length;const h=typeof TextDecoder!==T?new TextDecoder(U,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw V(`TextDecoder not available`)}};if(typeof TextDecoder!==T){h.decode()};let i=R;let l=W;const m=typeof TextEncoder!==T?new TextEncoder(U):{encode:()=>{throw V(`TextEncoder not available`)}};const n=typeof m.encodeInto===Y?((a,b)=>m.encodeInto(a,b)):((a,b)=>{const c=m.encode(a);b.set(c);return {read:a.length,written:c.length}});let q=R;const t=typeof $===T?{register:()=>{},unregister:()=>{}}:new $(b=>{a.__wbindgen_export_2.get(b.dtor)(b.a,b.b)});const D=typeof $===T?{register:()=>{},unregister:()=>{}}:new $(b=>a.__wbg_intounderlyingbytesource_free(b>>>W));class E{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=W;D.unregister(this);return a}free(){const b=this.__destroy_into_raw();a.__wbg_intounderlyingbytesource_free(b)}type(){try{const e=a.__wbindgen_add_to_stack_pointer(-a0);a.intounderlyingbytesource_type(e,this.__wbg_ptr);var b=r()[e/a1+ W];var c=r()[e/a1+ S];var d=A(b,c);if(b!==W){a.__wbindgen_free(b,c,S)};return d}finally{a.__wbindgen_add_to_stack_pointer(a0)}}autoAllocateChunkSize(){const b=a.intounderlyingbytesource_autoAllocateChunkSize(this.__wbg_ptr);return b>>>W}start(b){a.intounderlyingbytesource_start(this.__wbg_ptr,e(b))}pull(b){const c=a.intounderlyingbytesource_pull(this.__wbg_ptr,e(b));return g(c)}cancel(){const b=this.__destroy_into_raw();a.intounderlyingbytesource_cancel(b)}}const F=typeof $===T?{register:()=>{},unregister:()=>{}}:new $(b=>a.__wbg_intounderlyingsink_free(b>>>W));class G{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=W;F.unregister(this);return a}free(){const b=this.__destroy_into_raw();a.__wbg_intounderlyingsink_free(b)}write(b){const c=a.intounderlyingsink_write(this.__wbg_ptr,e(b));return g(c)}close(){const b=this.__destroy_into_raw();const c=a.intounderlyingsink_close(b);return g(c)}abort(b){const c=this.__destroy_into_raw();const d=a.intounderlyingsink_abort(c,e(b));return g(d)}}const H=typeof $===T?{register:()=>{},unregister:()=>{}}:new $(b=>a.__wbg_intounderlyingsource_free(b>>>W));class I{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=W;H.unregister(this);return a}free(){const b=this.__destroy_into_raw();a.__wbg_intounderlyingsource_free(b)}pull(b){const c=a.intounderlyingsource_pull(this.__wbg_ptr,e(b));return g(c)}cancel(){const b=this.__destroy_into_raw();a.intounderlyingsource_cancel(b)}}export default O;export{E as IntoUnderlyingByteSource,G as IntoUnderlyingSink,I as IntoUnderlyingSource,N as initSync}