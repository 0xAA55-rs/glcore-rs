#![allow(non_camel_case_types)]
#![allow(dead_code)]
use std::{mem::transmute, ffi::{c_void, CStr}, fmt::Debug, ptr::null};
type khronos_float_t = f32;
type khronos_ssize_t = usize;
type khronos_intptr_t = usize;
type khronos_int16_t = i16;
type khronos_int8_t = i8;
type khronos_uint16_t = u16;
type khronos_int64_t = i64;
type khronos_uint64_t = u64;
type GLDEBUGPROC = extern "system" fn(GLenum, GLenum, GLuint, GLenum, GLsizei, *const GLchar, *const c_void);

type GLvoid = c_void;
type GLenum = u32;
type GLbitfield = u32;
type GLuint = u32;
type GLfloat = f32;
type GLint = i32;
type GLsizei = i32;
type GLdouble = f64;
type GLboolean = u8;
type GLubyte = u8;
type GLshort = i16;
type GLbyte = i8;
type GLushort = u16;
type PFNGLCULLFACEPROC = extern "system" fn(GLenum);
type PFNGLFRONTFACEPROC = extern "system" fn(GLenum);
type PFNGLHINTPROC = extern "system" fn(GLenum, GLenum);
type PFNGLLINEWIDTHPROC = extern "system" fn(GLfloat);
type PFNGLPOINTSIZEPROC = extern "system" fn(GLfloat);
type PFNGLPOLYGONMODEPROC = extern "system" fn(GLenum, GLenum);
type PFNGLSCISSORPROC = extern "system" fn(GLint, GLint, GLsizei, GLsizei);
type PFNGLTEXPARAMETERFPROC = extern "system" fn(GLenum, GLenum, GLfloat);
type PFNGLTEXPARAMETERFVPROC = extern "system" fn(GLenum, GLenum, *const GLfloat);
type PFNGLTEXPARAMETERIPROC = extern "system" fn(GLenum, GLenum, GLint);
type PFNGLTEXPARAMETERIVPROC = extern "system" fn(GLenum, GLenum, *const GLint);
type PFNGLTEXIMAGE1DPROC = extern "system" fn(GLenum, GLint, GLint, GLsizei, GLint, GLenum, GLenum, *const c_void);
type PFNGLTEXIMAGE2DPROC = extern "system" fn(GLenum, GLint, GLint, GLsizei, GLsizei, GLint, GLenum, GLenum, *const c_void);
type PFNGLDRAWBUFFERPROC = extern "system" fn(GLenum);
type PFNGLCLEARPROC = extern "system" fn(GLbitfield);
type PFNGLCLEARCOLORPROC = extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat);
type PFNGLCLEARSTENCILPROC = extern "system" fn(GLint);
type PFNGLCLEARDEPTHPROC = extern "system" fn(GLdouble);
type PFNGLSTENCILMASKPROC = extern "system" fn(GLuint);
type PFNGLCOLORMASKPROC = extern "system" fn(GLboolean, GLboolean, GLboolean, GLboolean);
type PFNGLDEPTHMASKPROC = extern "system" fn(GLboolean);
type PFNGLDISABLEPROC = extern "system" fn(GLenum);
type PFNGLENABLEPROC = extern "system" fn(GLenum);
type PFNGLFINISHPROC = extern "system" fn();
type PFNGLFLUSHPROC = extern "system" fn();
type PFNGLBLENDFUNCPROC = extern "system" fn(GLenum, GLenum);
type PFNGLLOGICOPPROC = extern "system" fn(GLenum);
type PFNGLSTENCILFUNCPROC = extern "system" fn(GLenum, GLint, GLuint);
type PFNGLSTENCILOPPROC = extern "system" fn(GLenum, GLenum, GLenum);
type PFNGLDEPTHFUNCPROC = extern "system" fn(GLenum);
type PFNGLPIXELSTOREFPROC = extern "system" fn(GLenum, GLfloat);
type PFNGLPIXELSTOREIPROC = extern "system" fn(GLenum, GLint);
type PFNGLREADBUFFERPROC = extern "system" fn(GLenum);
type PFNGLREADPIXELSPROC = extern "system" fn(GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *mut c_void);
type PFNGLGETBOOLEANVPROC = extern "system" fn(GLenum, *mut GLboolean);
type PFNGLGETDOUBLEVPROC = extern "system" fn(GLenum, *mut GLdouble);
type PFNGLGETERRORPROC = extern "system" fn() -> GLenum;
type PFNGLGETFLOATVPROC = extern "system" fn(GLenum, *mut GLfloat);
type PFNGLGETINTEGERVPROC = extern "system" fn(GLenum, *mut GLint);
type PFNGLGETSTRINGPROC = extern "system" fn(GLenum) -> *const GLubyte;
type PFNGLGETTEXIMAGEPROC = extern "system" fn(GLenum, GLint, GLenum, GLenum, *mut c_void);
type PFNGLGETTEXPARAMETERFVPROC = extern "system" fn(GLenum, GLenum, *mut GLfloat);
type PFNGLGETTEXPARAMETERIVPROC = extern "system" fn(GLenum, GLenum, *mut GLint);
type PFNGLGETTEXLEVELPARAMETERFVPROC = extern "system" fn(GLenum, GLint, GLenum, *mut GLfloat);
type PFNGLGETTEXLEVELPARAMETERIVPROC = extern "system" fn(GLenum, GLint, GLenum, *mut GLint);
type PFNGLISENABLEDPROC = extern "system" fn(GLenum) -> GLboolean;
type PFNGLDEPTHRANGEPROC = extern "system" fn(GLdouble, GLdouble);
type PFNGLVIEWPORTPROC = extern "system" fn(GLint, GLint, GLsizei, GLsizei);
const DEPTH_BUFFER_BIT: GLbitfield = 0x00000100;
const STENCIL_BUFFER_BIT: GLbitfield = 0x00000400;
const COLOR_BUFFER_BIT: GLbitfield = 0x00004000;
const FALSE: GLenum = 0;
const TRUE: GLenum = 1;
const POINTS: GLenum = 0x0000;
const LINES: GLenum = 0x0001;
const LINE_LOOP: GLenum = 0x0002;
const LINE_STRIP: GLenum = 0x0003;
const TRIANGLES: GLenum = 0x0004;
const TRIANGLE_STRIP: GLenum = 0x0005;
const TRIANGLE_FAN: GLenum = 0x0006;
const QUADS: GLenum = 0x0007;
const NEVER: GLenum = 0x0200;
const LESS: GLenum = 0x0201;
const EQUAL: GLenum = 0x0202;
const LEQUAL: GLenum = 0x0203;
const GREATER: GLenum = 0x0204;
const NOTEQUAL: GLenum = 0x0205;
const GEQUAL: GLenum = 0x0206;
const ALWAYS: GLenum = 0x0207;
const ZERO: GLenum = 0;
const ONE: GLenum = 1;
const SRC_COLOR: GLenum = 0x0300;
const ONE_MINUS_SRC_COLOR: GLenum = 0x0301;
const SRC_ALPHA: GLenum = 0x0302;
const ONE_MINUS_SRC_ALPHA: GLenum = 0x0303;
const DST_ALPHA: GLenum = 0x0304;
const ONE_MINUS_DST_ALPHA: GLenum = 0x0305;
const DST_COLOR: GLenum = 0x0306;
const ONE_MINUS_DST_COLOR: GLenum = 0x0307;
const SRC_ALPHA_SATURATE: GLenum = 0x0308;
const NONE: GLenum = 0;
const FRONT_LEFT: GLenum = 0x0400;
const FRONT_RIGHT: GLenum = 0x0401;
const BACK_LEFT: GLenum = 0x0402;
const BACK_RIGHT: GLenum = 0x0403;
const FRONT: GLenum = 0x0404;
const BACK: GLenum = 0x0405;
const LEFT: GLenum = 0x0406;
const RIGHT: GLenum = 0x0407;
const FRONT_AND_BACK: GLenum = 0x0408;
const NO_ERROR: GLenum = 0;
const INVALID_ENUM: GLenum = 0x0500;
const INVALID_VALUE: GLenum = 0x0501;
const INVALID_OPERATION: GLenum = 0x0502;
const OUT_OF_MEMORY: GLenum = 0x0505;
const CW: GLenum = 0x0900;
const CCW: GLenum = 0x0901;
const POINT_SIZE: GLenum = 0x0B11;
const POINT_SIZE_RANGE: GLenum = 0x0B12;
const POINT_SIZE_GRANULARITY: GLenum = 0x0B13;
const LINE_SMOOTH: GLenum = 0x0B20;
const LINE_WIDTH: GLenum = 0x0B21;
const LINE_WIDTH_RANGE: GLenum = 0x0B22;
const LINE_WIDTH_GRANULARITY: GLenum = 0x0B23;
const POLYGON_MODE: GLenum = 0x0B40;
const POLYGON_SMOOTH: GLenum = 0x0B41;
const CULL_FACE: GLenum = 0x0B44;
const CULL_FACE_MODE: GLenum = 0x0B45;
const FRONT_FACE: GLenum = 0x0B46;
const DEPTH_RANGE: GLenum = 0x0B70;
const DEPTH_TEST: GLenum = 0x0B71;
const DEPTH_WRITEMASK: GLenum = 0x0B72;
const DEPTH_CLEAR_VALUE: GLenum = 0x0B73;
const DEPTH_FUNC: GLenum = 0x0B74;
const STENCIL_TEST: GLenum = 0x0B90;
const STENCIL_CLEAR_VALUE: GLenum = 0x0B91;
const STENCIL_FUNC: GLenum = 0x0B92;
const STENCIL_VALUE_MASK: GLenum = 0x0B93;
const STENCIL_FAIL: GLenum = 0x0B94;
const STENCIL_PASS_DEPTH_FAIL: GLenum = 0x0B95;
const STENCIL_PASS_DEPTH_PASS: GLenum = 0x0B96;
const STENCIL_REF: GLenum = 0x0B97;
const STENCIL_WRITEMASK: GLenum = 0x0B98;
const VIEWPORT: GLenum = 0x0BA2;
const DITHER: GLenum = 0x0BD0;
const BLEND_DST: GLenum = 0x0BE0;
const BLEND_SRC: GLenum = 0x0BE1;
const BLEND: GLenum = 0x0BE2;
const LOGIC_OP_MODE: GLenum = 0x0BF0;
const DRAW_BUFFER: GLenum = 0x0C01;
const READ_BUFFER: GLenum = 0x0C02;
const SCISSOR_BOX: GLenum = 0x0C10;
const SCISSOR_TEST: GLenum = 0x0C11;
const COLOR_CLEAR_VALUE: GLenum = 0x0C22;
const COLOR_WRITEMASK: GLenum = 0x0C23;
const DOUBLEBUFFER: GLenum = 0x0C32;
const STEREO: GLenum = 0x0C33;
const LINE_SMOOTH_HINT: GLenum = 0x0C52;
const POLYGON_SMOOTH_HINT: GLenum = 0x0C53;
const UNPACK_SWAP_BYTES: GLenum = 0x0CF0;
const UNPACK_LSB_FIRST: GLenum = 0x0CF1;
const UNPACK_ROW_LENGTH: GLenum = 0x0CF2;
const UNPACK_SKIP_ROWS: GLenum = 0x0CF3;
const UNPACK_SKIP_PIXELS: GLenum = 0x0CF4;
const UNPACK_ALIGNMENT: GLenum = 0x0CF5;
const PACK_SWAP_BYTES: GLenum = 0x0D00;
const PACK_LSB_FIRST: GLenum = 0x0D01;
const PACK_ROW_LENGTH: GLenum = 0x0D02;
const PACK_SKIP_ROWS: GLenum = 0x0D03;
const PACK_SKIP_PIXELS: GLenum = 0x0D04;
const PACK_ALIGNMENT: GLenum = 0x0D05;
const MAX_TEXTURE_SIZE: GLenum = 0x0D33;
const MAX_VIEWPORT_DIMS: GLenum = 0x0D3A;
const SUBPIXEL_BITS: GLenum = 0x0D50;
const TEXTURE_1D: GLenum = 0x0DE0;
const TEXTURE_2D: GLenum = 0x0DE1;
const TEXTURE_WIDTH: GLenum = 0x1000;
const TEXTURE_HEIGHT: GLenum = 0x1001;
const TEXTURE_BORDER_COLOR: GLenum = 0x1004;
const DONT_CARE: GLenum = 0x1100;
const FASTEST: GLenum = 0x1101;
const NICEST: GLenum = 0x1102;
const BYTE: GLenum = 0x1400;
const UNSIGNED_BYTE: GLenum = 0x1401;
const SHORT: GLenum = 0x1402;
const UNSIGNED_SHORT: GLenum = 0x1403;
const INT: GLenum = 0x1404;
const UNSIGNED_INT: GLenum = 0x1405;
const FLOAT: GLenum = 0x1406;
const STACK_OVERFLOW: GLenum = 0x0503;
const STACK_UNDERFLOW: GLenum = 0x0504;
const CLEAR: GLenum = 0x1500;
const AND: GLenum = 0x1501;
const AND_REVERSE: GLenum = 0x1502;
const COPY: GLenum = 0x1503;
const AND_INVERTED: GLenum = 0x1504;
const NOOP: GLenum = 0x1505;
const XOR: GLenum = 0x1506;
const OR: GLenum = 0x1507;
const NOR: GLenum = 0x1508;
const EQUIV: GLenum = 0x1509;
const INVERT: GLenum = 0x150A;
const OR_REVERSE: GLenum = 0x150B;
const COPY_INVERTED: GLenum = 0x150C;
const OR_INVERTED: GLenum = 0x150D;
const NAND: GLenum = 0x150E;
const SET: GLenum = 0x150F;
const TEXTURE: GLenum = 0x1702;
const COLOR: GLenum = 0x1800;
const DEPTH: GLenum = 0x1801;
const STENCIL: GLenum = 0x1802;
const STENCIL_INDEX: GLenum = 0x1901;
const DEPTH_COMPONENT: GLenum = 0x1902;
const RED: GLenum = 0x1903;
const GREEN: GLenum = 0x1904;
const BLUE: GLenum = 0x1905;
const ALPHA: GLenum = 0x1906;
const RGB: GLenum = 0x1907;
const RGBA: GLenum = 0x1908;
const POINT: GLenum = 0x1B00;
const LINE: GLenum = 0x1B01;
const FILL: GLenum = 0x1B02;
const KEEP: GLenum = 0x1E00;
const REPLACE: GLenum = 0x1E01;
const INCR: GLenum = 0x1E02;
const DECR: GLenum = 0x1E03;
const VENDOR: GLenum = 0x1F00;
const RENDERER: GLenum = 0x1F01;
const VERSION: GLenum = 0x1F02;
const EXTENSIONS: GLenum = 0x1F03;
const NEAREST: GLint = 0x2600;
const LINEAR: GLint = 0x2601;
const NEAREST_MIPMAP_NEAREST: GLint = 0x2700;
const LINEAR_MIPMAP_NEAREST: GLint = 0x2701;
const NEAREST_MIPMAP_LINEAR: GLint = 0x2702;
const LINEAR_MIPMAP_LINEAR: GLint = 0x2703;
const TEXTURE_MAG_FILTER: GLenum = 0x2800;
const TEXTURE_MIN_FILTER: GLenum = 0x2801;
const TEXTURE_WRAP_S: GLenum = 0x2802;
const TEXTURE_WRAP_T: GLenum = 0x2803;
const REPEAT: GLint = 0x2901;

pub trait GL_1_0 {
	fn glCullFace(&self, _: GLenum) {
		panic!("OpenGL function pointer of `glCullFace` is NULL");
	}
	fn glFrontFace(&self, _: GLenum) {
		panic!("OpenGL function pointer of `glFrontFace` is NULL");
	}
	fn glHint(&self, _: GLenum, _: GLenum) {
		panic!("OpenGL function pointer of `glHint` is NULL");
	}
	fn glLineWidth(&self, _: GLfloat) {
		panic!("OpenGL function pointer of `glLineWidth` is NULL");
	}
	fn glPointSize(&self, _: GLfloat) {
		panic!("OpenGL function pointer of `glPointSize` is NULL");
	}
	fn glPolygonMode(&self, _: GLenum, _: GLenum) {
		panic!("OpenGL function pointer of `glPolygonMode` is NULL");
	}
	fn glScissor(&self, _: GLint, _: GLint, _: GLsizei, _: GLsizei) {
		panic!("OpenGL function pointer of `glScissor` is NULL");
	}
	fn glTexParameterf(&self, _: GLenum, _: GLenum, _: GLfloat) {
		panic!("OpenGL function pointer of `glTexParameterf` is NULL");
	}
	fn glTexParameterfv(&self, _: GLenum, _: GLenum, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glTexParameterfv` is NULL");
	}
	fn glTexParameteri(&self, _: GLenum, _: GLenum, _: GLint) {
		panic!("OpenGL function pointer of `glTexParameteri` is NULL");
	}
	fn glTexParameteriv(&self, _: GLenum, _: GLenum, _: *const GLint) {
		panic!("OpenGL function pointer of `glTexParameteriv` is NULL");
	}
	fn glTexImage1D(&self, _: GLenum, _: GLint, _: GLint, _: GLsizei, _: GLint, _: GLenum, _: GLenum, _: *const c_void) {
		panic!("OpenGL function pointer of `glTexImage1D` is NULL");
	}
	fn glTexImage2D(&self, _: GLenum, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLint, _: GLenum, _: GLenum, _: *const c_void) {
		panic!("OpenGL function pointer of `glTexImage2D` is NULL");
	}
	fn glDrawBuffer(&self, _: GLenum) {
		panic!("OpenGL function pointer of `glDrawBuffer` is NULL");
	}
	fn glClear(&self, _: GLbitfield) {
		panic!("OpenGL function pointer of `glClear` is NULL");
	}
	fn glClearColor(&self, _: GLfloat, _: GLfloat, _: GLfloat, _: GLfloat) {
		panic!("OpenGL function pointer of `glClearColor` is NULL");
	}
	fn glClearStencil(&self, _: GLint) {
		panic!("OpenGL function pointer of `glClearStencil` is NULL");
	}
	fn glClearDepth(&self, _: GLdouble) {
		panic!("OpenGL function pointer of `glClearDepth` is NULL");
	}
	fn glStencilMask(&self, _: GLuint) {
		panic!("OpenGL function pointer of `glStencilMask` is NULL");
	}
	fn glColorMask(&self, _: GLboolean, _: GLboolean, _: GLboolean, _: GLboolean) {
		panic!("OpenGL function pointer of `glColorMask` is NULL");
	}
	fn glDepthMask(&self, _: GLboolean) {
		panic!("OpenGL function pointer of `glDepthMask` is NULL");
	}
	fn glDisable(&self, _: GLenum) {
		panic!("OpenGL function pointer of `glDisable` is NULL");
	}
	fn glEnable(&self, _: GLenum) {
		panic!("OpenGL function pointer of `glEnable` is NULL");
	}
	fn glFinish(&self) {
		panic!("OpenGL function pointer of `glFinish` is NULL");
	}
	fn glFlush(&self) {
		panic!("OpenGL function pointer of `glFlush` is NULL");
	}
	fn glBlendFunc(&self, _: GLenum, _: GLenum) {
		panic!("OpenGL function pointer of `glBlendFunc` is NULL");
	}
	fn glLogicOp(&self, _: GLenum) {
		panic!("OpenGL function pointer of `glLogicOp` is NULL");
	}
	fn glStencilFunc(&self, _: GLenum, _: GLint, _: GLuint) {
		panic!("OpenGL function pointer of `glStencilFunc` is NULL");
	}
	fn glStencilOp(&self, _: GLenum, _: GLenum, _: GLenum) {
		panic!("OpenGL function pointer of `glStencilOp` is NULL");
	}
	fn glDepthFunc(&self, _: GLenum) {
		panic!("OpenGL function pointer of `glDepthFunc` is NULL");
	}
	fn glPixelStoref(&self, _: GLenum, _: GLfloat) {
		panic!("OpenGL function pointer of `glPixelStoref` is NULL");
	}
	fn glPixelStorei(&self, _: GLenum, _: GLint) {
		panic!("OpenGL function pointer of `glPixelStorei` is NULL");
	}
	fn glReadBuffer(&self, _: GLenum) {
		panic!("OpenGL function pointer of `glReadBuffer` is NULL");
	}
	fn glReadPixels(&self, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLenum, _: GLenum, _: *mut c_void) {
		panic!("OpenGL function pointer of `glReadPixels` is NULL");
	}
	fn glGetBooleanv(&self, _: GLenum, _: *mut GLboolean) {
		panic!("OpenGL function pointer of `glGetBooleanv` is NULL");
	}
	fn glGetDoublev(&self, _: GLenum, _: *mut GLdouble) {
		panic!("OpenGL function pointer of `glGetDoublev` is NULL");
	}
	fn glGetError(&self) -> GLenum {
		panic!("OpenGL function pointer of `glGetError` is NULL");
	}
	fn glGetFloatv(&self, _: GLenum, _: *mut GLfloat) {
		panic!("OpenGL function pointer of `glGetFloatv` is NULL");
	}
	fn glGetIntegerv(&self, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetIntegerv` is NULL");
	}
	fn glGetString(&self, _: GLenum) -> *const GLubyte {
		panic!("OpenGL function pointer of `glGetString` is NULL");
	}
	fn glGetTexImage(&self, _: GLenum, _: GLint, _: GLenum, _: GLenum, _: *mut c_void) {
		panic!("OpenGL function pointer of `glGetTexImage` is NULL");
	}
	fn glGetTexParameterfv(&self, _: GLenum, _: GLenum, _: *mut GLfloat) {
		panic!("OpenGL function pointer of `glGetTexParameterfv` is NULL");
	}
	fn glGetTexParameteriv(&self, _: GLenum, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetTexParameteriv` is NULL");
	}
	fn glGetTexLevelParameterfv(&self, _: GLenum, _: GLint, _: GLenum, _: *mut GLfloat) {
		panic!("OpenGL function pointer of `glGetTexLevelParameterfv` is NULL");
	}
	fn glGetTexLevelParameteriv(&self, _: GLenum, _: GLint, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetTexLevelParameteriv` is NULL");
	}
	fn glIsEnabled(&self, _: GLenum) -> GLboolean {
		panic!("OpenGL function pointer of `glIsEnabled` is NULL");
	}
	fn glDepthRange(&self, _: GLdouble, _: GLdouble) {
		panic!("OpenGL function pointer of `glDepthRange` is NULL");
	}
	fn glViewport(&self, _: GLint, _: GLint, _: GLsizei, _: GLsizei) {
		panic!("OpenGL function pointer of `glViewport` is NULL");
	}
	fn get_version(&self) -> (&'static str, u32, u32, u32);
	fn get_vendor(&self) -> &'static str;
	fn get_renderer(&self) -> &'static str;
	fn get_versionstr(&self) -> &'static str;
}

#[derive(Debug, Clone, Copy)]
pub struct Version10 {
	spec: &'static str,
	major_version: u32,
	minor_version: u32,
	release_version: u32,
	vendor: &'static str,
	renderer: &'static str,
	version: &'static str,
	available: bool,
	cullface: PFNGLCULLFACEPROC,
	frontface: PFNGLFRONTFACEPROC,
	hint: PFNGLHINTPROC,
	linewidth: PFNGLLINEWIDTHPROC,
	pointsize: PFNGLPOINTSIZEPROC,
	polygonmode: PFNGLPOLYGONMODEPROC,
	scissor: PFNGLSCISSORPROC,
	texparameterf: PFNGLTEXPARAMETERFPROC,
	texparameterfv: PFNGLTEXPARAMETERFVPROC,
	texparameteri: PFNGLTEXPARAMETERIPROC,
	texparameteriv: PFNGLTEXPARAMETERIVPROC,
	teximage1d: PFNGLTEXIMAGE1DPROC,
	teximage2d: PFNGLTEXIMAGE2DPROC,
	drawbuffer: PFNGLDRAWBUFFERPROC,
	clear: PFNGLCLEARPROC,
	clearcolor: PFNGLCLEARCOLORPROC,
	clearstencil: PFNGLCLEARSTENCILPROC,
	cleardepth: PFNGLCLEARDEPTHPROC,
	stencilmask: PFNGLSTENCILMASKPROC,
	colormask: PFNGLCOLORMASKPROC,
	depthmask: PFNGLDEPTHMASKPROC,
	disable: PFNGLDISABLEPROC,
	enable: PFNGLENABLEPROC,
	finish: PFNGLFINISHPROC,
	flush: PFNGLFLUSHPROC,
	blendfunc: PFNGLBLENDFUNCPROC,
	logicop: PFNGLLOGICOPPROC,
	stencilfunc: PFNGLSTENCILFUNCPROC,
	stencilop: PFNGLSTENCILOPPROC,
	depthfunc: PFNGLDEPTHFUNCPROC,
	pixelstoref: PFNGLPIXELSTOREFPROC,
	pixelstorei: PFNGLPIXELSTOREIPROC,
	readbuffer: PFNGLREADBUFFERPROC,
	readpixels: PFNGLREADPIXELSPROC,
	getbooleanv: PFNGLGETBOOLEANVPROC,
	getdoublev: PFNGLGETDOUBLEVPROC,
	geterror: PFNGLGETERRORPROC,
	getfloatv: PFNGLGETFLOATVPROC,
	getintegerv: PFNGLGETINTEGERVPROC,
	getstring: PFNGLGETSTRINGPROC,
	getteximage: PFNGLGETTEXIMAGEPROC,
	gettexparameterfv: PFNGLGETTEXPARAMETERFVPROC,
	gettexparameteriv: PFNGLGETTEXPARAMETERIVPROC,
	gettexlevelparameterfv: PFNGLGETTEXLEVELPARAMETERFVPROC,
	gettexlevelparameteriv: PFNGLGETTEXLEVELPARAMETERIVPROC,
	isenabled: PFNGLISENABLEDPROC,
	depthrange: PFNGLDEPTHRANGEPROC,
	viewport: PFNGLVIEWPORTPROC,
}

impl GL_1_0 for Version10 {
	#[inline(always)]
	fn glCullFace(&self, mode: GLenum) {
		(self.cullface)(mode)
	}
	#[inline(always)]
	fn glFrontFace(&self, mode: GLenum) {
		(self.frontface)(mode)
	}
	#[inline(always)]
	fn glHint(&self, target: GLenum, mode: GLenum) {
		(self.hint)(target, mode)
	}
	#[inline(always)]
	fn glLineWidth(&self, width: GLfloat) {
		(self.linewidth)(width)
	}
	#[inline(always)]
	fn glPointSize(&self, size: GLfloat) {
		(self.pointsize)(size)
	}
	#[inline(always)]
	fn glPolygonMode(&self, face: GLenum, mode: GLenum) {
		(self.polygonmode)(face, mode)
	}
	#[inline(always)]
	fn glScissor(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
		(self.scissor)(x, y, width, height)
	}
	#[inline(always)]
	fn glTexParameterf(&self, target: GLenum, pname: GLenum, param: GLfloat) {
		(self.texparameterf)(target, pname, param)
	}
	#[inline(always)]
	fn glTexParameterfv(&self, target: GLenum, pname: GLenum, params: *const GLfloat) {
		(self.texparameterfv)(target, pname, params)
	}
	#[inline(always)]
	fn glTexParameteri(&self, target: GLenum, pname: GLenum, param: GLint) {
		(self.texparameteri)(target, pname, param)
	}
	#[inline(always)]
	fn glTexParameteriv(&self, target: GLenum, pname: GLenum, params: *const GLint) {
		(self.texparameteriv)(target, pname, params)
	}
	#[inline(always)]
	fn glTexImage1D(&self, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void) {
		(self.teximage1d)(target, level, internalformat, width, border, format, type_, pixels)
	}
	#[inline(always)]
	fn glTexImage2D(&self, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void) {
		(self.teximage2d)(target, level, internalformat, width, height, border, format, type_, pixels)
	}
	#[inline(always)]
	fn glDrawBuffer(&self, buf: GLenum) {
		(self.drawbuffer)(buf)
	}
	#[inline(always)]
	fn glClear(&self, mask: GLbitfield) {
		(self.clear)(mask)
	}
	#[inline(always)]
	fn glClearColor(&self, red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
		(self.clearcolor)(red, green, blue, alpha)
	}
	#[inline(always)]
	fn glClearStencil(&self, s: GLint) {
		(self.clearstencil)(s)
	}
	#[inline(always)]
	fn glClearDepth(&self, depth: GLdouble) {
		(self.cleardepth)(depth)
	}
	#[inline(always)]
	fn glStencilMask(&self, mask: GLuint) {
		(self.stencilmask)(mask)
	}
	#[inline(always)]
	fn glColorMask(&self, red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) {
		(self.colormask)(red, green, blue, alpha)
	}
	#[inline(always)]
	fn glDepthMask(&self, flag: GLboolean) {
		(self.depthmask)(flag)
	}
	#[inline(always)]
	fn glDisable(&self, cap: GLenum) {
		(self.disable)(cap)
	}
	#[inline(always)]
	fn glEnable(&self, cap: GLenum) {
		(self.enable)(cap)
	}
	#[inline(always)]
	fn glFinish(&self) {
		(self.finish)()
	}
	#[inline(always)]
	fn glFlush(&self) {
		(self.flush)()
	}
	#[inline(always)]
	fn glBlendFunc(&self, sfactor: GLenum, dfactor: GLenum) {
		(self.blendfunc)(sfactor, dfactor)
	}
	#[inline(always)]
	fn glLogicOp(&self, opcode: GLenum) {
		(self.logicop)(opcode)
	}
	#[inline(always)]
	fn glStencilFunc(&self, func: GLenum, ref_: GLint, mask: GLuint) {
		(self.stencilfunc)(func, ref_, mask)
	}
	#[inline(always)]
	fn glStencilOp(&self, fail: GLenum, zfail: GLenum, zpass: GLenum) {
		(self.stencilop)(fail, zfail, zpass)
	}
	#[inline(always)]
	fn glDepthFunc(&self, func: GLenum) {
		(self.depthfunc)(func)
	}
	#[inline(always)]
	fn glPixelStoref(&self, pname: GLenum, param: GLfloat) {
		(self.pixelstoref)(pname, param)
	}
	#[inline(always)]
	fn glPixelStorei(&self, pname: GLenum, param: GLint) {
		(self.pixelstorei)(pname, param)
	}
	#[inline(always)]
	fn glReadBuffer(&self, src: GLenum) {
		(self.readbuffer)(src)
	}
	#[inline(always)]
	fn glReadPixels(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *mut c_void) {
		(self.readpixels)(x, y, width, height, format, type_, pixels)
	}
	#[inline(always)]
	fn glGetBooleanv(&self, pname: GLenum, data: *mut GLboolean) {
		(self.getbooleanv)(pname, data)
	}
	#[inline(always)]
	fn glGetDoublev(&self, pname: GLenum, data: *mut GLdouble) {
		(self.getdoublev)(pname, data)
	}
	#[inline(always)]
	fn glGetError(&self) -> GLenum {
		(self.geterror)()
	}
	#[inline(always)]
	fn glGetFloatv(&self, pname: GLenum, data: *mut GLfloat) {
		(self.getfloatv)(pname, data)
	}
	#[inline(always)]
	fn glGetIntegerv(&self, pname: GLenum, data: *mut GLint) {
		(self.getintegerv)(pname, data)
	}
	#[inline(always)]
	fn glGetString(&self, name: GLenum) -> *const GLubyte {
		(self.getstring)(name)
	}
	#[inline(always)]
	fn glGetTexImage(&self, target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *mut c_void) {
		(self.getteximage)(target, level, format, type_, pixels)
	}
	#[inline(always)]
	fn glGetTexParameterfv(&self, target: GLenum, pname: GLenum, params: *mut GLfloat) {
		(self.gettexparameterfv)(target, pname, params)
	}
	#[inline(always)]
	fn glGetTexParameteriv(&self, target: GLenum, pname: GLenum, params: *mut GLint) {
		(self.gettexparameteriv)(target, pname, params)
	}
	#[inline(always)]
	fn glGetTexLevelParameterfv(&self, target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat) {
		(self.gettexlevelparameterfv)(target, level, pname, params)
	}
	#[inline(always)]
	fn glGetTexLevelParameteriv(&self, target: GLenum, level: GLint, pname: GLenum, params: *mut GLint) {
		(self.gettexlevelparameteriv)(target, level, pname, params)
	}
	#[inline(always)]
	fn glIsEnabled(&self, cap: GLenum) -> GLboolean {
		(self.isenabled)(cap)
	}
	#[inline(always)]
	fn glDepthRange(&self, n: GLdouble, f: GLdouble) {
		(self.depthrange)(n, f)
	}
	#[inline(always)]
	fn glViewport(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
		(self.viewport)(x, y, width, height)
	}
	#[inline(always)]
	fn get_version(&self) -> (&'static str, u32, u32, u32) {
		(self.spec, self.major_version, self.minor_version, self.release_version)
	}
	#[inline(always)]
	fn get_vendor(&self) -> &'static str {
		self.vendor
	}
	#[inline(always)]
	fn get_renderer(&self) -> &'static str {
		self.renderer
	}
	#[inline(always)]
	fn get_versionstr(&self) -> &'static str {
		self.version
	}
}

impl Version10 {
	pub fn new(get_proc_address: &impl Fn(&'static str) -> *const c_void) -> Self {
		let mut ret = Self {
			available: true,
			spec: "unknown",
			major_version: 0,
			minor_version: 0,
			release_version: 0,
			vendor: "unknown",
			renderer: "unknown",
			version: "unknown",
			cullface: unsafe{transmute(get_proc_address("glCullFace"))},
			frontface: unsafe{transmute(get_proc_address("glFrontFace"))},
			hint: unsafe{transmute(get_proc_address("glHint"))},
			linewidth: unsafe{transmute(get_proc_address("glLineWidth"))},
			pointsize: unsafe{transmute(get_proc_address("glPointSize"))},
			polygonmode: unsafe{transmute(get_proc_address("glPolygonMode"))},
			scissor: unsafe{transmute(get_proc_address("glScissor"))},
			texparameterf: unsafe{transmute(get_proc_address("glTexParameterf"))},
			texparameterfv: unsafe{transmute(get_proc_address("glTexParameterfv"))},
			texparameteri: unsafe{transmute(get_proc_address("glTexParameteri"))},
			texparameteriv: unsafe{transmute(get_proc_address("glTexParameteriv"))},
			teximage1d: unsafe{transmute(get_proc_address("glTexImage1D"))},
			teximage2d: unsafe{transmute(get_proc_address("glTexImage2D"))},
			drawbuffer: unsafe{transmute(get_proc_address("glDrawBuffer"))},
			clear: unsafe{transmute(get_proc_address("glClear"))},
			clearcolor: unsafe{transmute(get_proc_address("glClearColor"))},
			clearstencil: unsafe{transmute(get_proc_address("glClearStencil"))},
			cleardepth: unsafe{transmute(get_proc_address("glClearDepth"))},
			stencilmask: unsafe{transmute(get_proc_address("glStencilMask"))},
			colormask: unsafe{transmute(get_proc_address("glColorMask"))},
			depthmask: unsafe{transmute(get_proc_address("glDepthMask"))},
			disable: unsafe{transmute(get_proc_address("glDisable"))},
			enable: unsafe{transmute(get_proc_address("glEnable"))},
			finish: unsafe{transmute(get_proc_address("glFinish"))},
			flush: unsafe{transmute(get_proc_address("glFlush"))},
			blendfunc: unsafe{transmute(get_proc_address("glBlendFunc"))},
			logicop: unsafe{transmute(get_proc_address("glLogicOp"))},
			stencilfunc: unsafe{transmute(get_proc_address("glStencilFunc"))},
			stencilop: unsafe{transmute(get_proc_address("glStencilOp"))},
			depthfunc: unsafe{transmute(get_proc_address("glDepthFunc"))},
			pixelstoref: unsafe{transmute(get_proc_address("glPixelStoref"))},
			pixelstorei: unsafe{transmute(get_proc_address("glPixelStorei"))},
			readbuffer: unsafe{transmute(get_proc_address("glReadBuffer"))},
			readpixels: unsafe{transmute(get_proc_address("glReadPixels"))},
			getbooleanv: unsafe{transmute(get_proc_address("glGetBooleanv"))},
			getdoublev: unsafe{transmute(get_proc_address("glGetDoublev"))},
			geterror: unsafe{transmute(get_proc_address("glGetError"))},
			getfloatv: unsafe{transmute(get_proc_address("glGetFloatv"))},
			getintegerv: unsafe{transmute(get_proc_address("glGetIntegerv"))},
			getstring: unsafe{transmute(get_proc_address("glGetString"))},
			getteximage: unsafe{transmute(get_proc_address("glGetTexImage"))},
			gettexparameterfv: unsafe{transmute(get_proc_address("glGetTexParameterfv"))},
			gettexparameteriv: unsafe{transmute(get_proc_address("glGetTexParameteriv"))},
			gettexlevelparameterfv: unsafe{transmute(get_proc_address("glGetTexLevelParameterfv"))},
			gettexlevelparameteriv: unsafe{transmute(get_proc_address("glGetTexLevelParameteriv"))},
			isenabled: unsafe{transmute(get_proc_address("glIsEnabled"))},
			depthrange: unsafe{transmute(get_proc_address("glDepthRange"))},
			viewport: unsafe{transmute(get_proc_address("glViewport"))},
		};
		ret.fetch_version();
		ret
	}
	#[inline(always)]
	fn fetch_version(&mut self) {
		self.vendor = unsafe{CStr::from_ptr(self.glGetString(VENDOR) as *const i8)}.to_str().unwrap();
		self.renderer = unsafe{CStr::from_ptr(self.glGetString(RENDERER) as *const i8)}.to_str().unwrap();
		self.version = unsafe{CStr::from_ptr(self.glGetString(VERSION) as *const i8)}.to_str().unwrap();
		self.spec = "OpenGL";
		let mut verstr = self.version;
		if let Some((left, right)) = verstr.rsplit_once(' ') {
			self.spec = left;
			verstr = right;
		}
		let mut v: Vec<&str> = verstr.split('.').collect();
		v.resize(3, "0");
		v = v.into_iter().map(|x|if x == "" {"0"} else {x}).collect();
		self.major_version = v[0].parse().unwrap();
		self.minor_version = v[1].parse().unwrap();
		self.release_version = v[2].parse().unwrap();
	}
	#[inline(always)]
	pub fn get_available(&self) -> bool {
		self.available
	}
}

impl Default for Version10 {
	fn default() -> Self {
		Self {
			available: false,
			spec: "unknown",
			major_version: 0,
			minor_version: 0,
			release_version: 0,
			vendor: "unknown",
			renderer: "unknown",
			version: "unknown",
			cullface: null(),
			frontface: null(),
			hint: null(),
			linewidth: null(),
			pointsize: null(),
			polygonmode: null(),
			scissor: null(),
			texparameterf: null(),
			texparameterfv: null(),
			texparameteri: null(),
			texparameteriv: null(),
			teximage1d: null(),
			teximage2d: null(),
			drawbuffer: null(),
			clear: null(),
			clearcolor: null(),
			clearstencil: null(),
			cleardepth: null(),
			stencilmask: null(),
			colormask: null(),
			depthmask: null(),
			disable: null(),
			enable: null(),
			finish: null(),
			flush: null(),
			blendfunc: null(),
			logicop: null(),
			stencilfunc: null(),
			stencilop: null(),
			depthfunc: null(),
			pixelstoref: null(),
			pixelstorei: null(),
			readbuffer: null(),
			readpixels: null(),
			getbooleanv: null(),
			getdoublev: null(),
			geterror: null(),
			getfloatv: null(),
			getintegerv: null(),
			getstring: null(),
			getteximage: null(),
			gettexparameterfv: null(),
			gettexparameteriv: null(),
			gettexlevelparameterfv: null(),
			gettexlevelparameteriv: null(),
			isenabled: null(),
			depthrange: null(),
			viewport: null(),
		}
	}
}

type GLclampf = khronos_float_t;
type GLclampd = f64;
type PFNGLDRAWARRAYSPROC = extern "system" fn(GLenum, GLint, GLsizei);
type PFNGLDRAWELEMENTSPROC = extern "system" fn(GLenum, GLsizei, GLenum, *const c_void);
type PFNGLGETPOINTERVPROC = extern "system" fn(GLenum, *mut *mut c_void);
type PFNGLPOLYGONOFFSETPROC = extern "system" fn(GLfloat, GLfloat);
type PFNGLCOPYTEXIMAGE1DPROC = extern "system" fn(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLint);
type PFNGLCOPYTEXIMAGE2DPROC = extern "system" fn(GLenum, GLint, GLenum, GLint, GLint, GLsizei, GLsizei, GLint);
type PFNGLCOPYTEXSUBIMAGE1DPROC = extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei);
type PFNGLCOPYTEXSUBIMAGE2DPROC = extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei);
type PFNGLTEXSUBIMAGE1DPROC = extern "system" fn(GLenum, GLint, GLint, GLsizei, GLenum, GLenum, *const c_void);
type PFNGLTEXSUBIMAGE2DPROC = extern "system" fn(GLenum, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *const c_void);
type PFNGLBINDTEXTUREPROC = extern "system" fn(GLenum, GLuint);
type PFNGLDELETETEXTURESPROC = extern "system" fn(GLsizei, *const GLuint);
type PFNGLGENTEXTURESPROC = extern "system" fn(GLsizei, *mut GLuint);
type PFNGLISTEXTUREPROC = extern "system" fn(GLuint) -> GLboolean;
const COLOR_LOGIC_OP: GLenum = 0x0BF2;
const POLYGON_OFFSET_UNITS: GLenum = 0x2A00;
const POLYGON_OFFSET_POINT: GLenum = 0x2A01;
const POLYGON_OFFSET_LINE: GLenum = 0x2A02;
const POLYGON_OFFSET_FILL: GLenum = 0x8037;
const POLYGON_OFFSET_FACTOR: GLenum = 0x8038;
const TEXTURE_BINDING_1D: GLenum = 0x8068;
const TEXTURE_BINDING_2D: GLenum = 0x8069;
const TEXTURE_INTERNAL_FORMAT: GLenum = 0x1003;
const TEXTURE_RED_SIZE: GLenum = 0x805C;
const TEXTURE_GREEN_SIZE: GLenum = 0x805D;
const TEXTURE_BLUE_SIZE: GLenum = 0x805E;
const TEXTURE_ALPHA_SIZE: GLenum = 0x805F;
const DOUBLE: GLenum = 0x140A;
const PROXY_TEXTURE_1D: GLenum = 0x8063;
const PROXY_TEXTURE_2D: GLenum = 0x8064;
const R3_G3_B2: GLenum = 0x2A10;
const RGB4: GLenum = 0x804F;
const RGB5: GLenum = 0x8050;
const RGB8: GLenum = 0x8051;
const RGB10: GLenum = 0x8052;
const RGB12: GLenum = 0x8053;
const RGB16: GLenum = 0x8054;
const RGBA2: GLenum = 0x8055;
const RGBA4: GLenum = 0x8056;
const RGB5_A1: GLenum = 0x8057;
const RGBA8: GLenum = 0x8058;
const RGB10_A2: GLenum = 0x8059;
const RGBA12: GLenum = 0x805A;
const RGBA16: GLenum = 0x805B;
const VERTEX_ARRAY: GLenum = 0x8074;

pub trait GL_1_1 {
	fn glDrawArrays(&self, _: GLenum, _: GLint, _: GLsizei) {
		panic!("OpenGL function pointer of `glDrawArrays` is NULL");
	}
	fn glDrawElements(&self, _: GLenum, _: GLsizei, _: GLenum, _: *const c_void) {
		panic!("OpenGL function pointer of `glDrawElements` is NULL");
	}
	fn glGetPointerv(&self, _: GLenum, _: *mut *mut c_void) {
		panic!("OpenGL function pointer of `glGetPointerv` is NULL");
	}
	fn glPolygonOffset(&self, _: GLfloat, _: GLfloat) {
		panic!("OpenGL function pointer of `glPolygonOffset` is NULL");
	}
	fn glCopyTexImage1D(&self, _: GLenum, _: GLint, _: GLenum, _: GLint, _: GLint, _: GLsizei, _: GLint) {
		panic!("OpenGL function pointer of `glCopyTexImage1D` is NULL");
	}
	fn glCopyTexImage2D(&self, _: GLenum, _: GLint, _: GLenum, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLint) {
		panic!("OpenGL function pointer of `glCopyTexImage2D` is NULL");
	}
	fn glCopyTexSubImage1D(&self, _: GLenum, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei) {
		panic!("OpenGL function pointer of `glCopyTexSubImage1D` is NULL");
	}
	fn glCopyTexSubImage2D(&self, _: GLenum, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei) {
		panic!("OpenGL function pointer of `glCopyTexSubImage2D` is NULL");
	}
	fn glTexSubImage1D(&self, _: GLenum, _: GLint, _: GLint, _: GLsizei, _: GLenum, _: GLenum, _: *const c_void) {
		panic!("OpenGL function pointer of `glTexSubImage1D` is NULL");
	}
	fn glTexSubImage2D(&self, _: GLenum, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLenum, _: GLenum, _: *const c_void) {
		panic!("OpenGL function pointer of `glTexSubImage2D` is NULL");
	}
	fn glBindTexture(&self, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glBindTexture` is NULL");
	}
	fn glDeleteTextures(&self, _: GLsizei, _: *const GLuint) {
		panic!("OpenGL function pointer of `glDeleteTextures` is NULL");
	}
	fn glGenTextures(&self, _: GLsizei, _: *mut GLuint) {
		panic!("OpenGL function pointer of `glGenTextures` is NULL");
	}
	fn glIsTexture(&self, _: GLuint) -> GLboolean {
		panic!("OpenGL function pointer of `glIsTexture` is NULL");
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Version11 {
	available: bool,
	drawarrays: PFNGLDRAWARRAYSPROC,
	drawelements: PFNGLDRAWELEMENTSPROC,
	getpointerv: PFNGLGETPOINTERVPROC,
	polygonoffset: PFNGLPOLYGONOFFSETPROC,
	copyteximage1d: PFNGLCOPYTEXIMAGE1DPROC,
	copyteximage2d: PFNGLCOPYTEXIMAGE2DPROC,
	copytexsubimage1d: PFNGLCOPYTEXSUBIMAGE1DPROC,
	copytexsubimage2d: PFNGLCOPYTEXSUBIMAGE2DPROC,
	texsubimage1d: PFNGLTEXSUBIMAGE1DPROC,
	texsubimage2d: PFNGLTEXSUBIMAGE2DPROC,
	bindtexture: PFNGLBINDTEXTUREPROC,
	deletetextures: PFNGLDELETETEXTURESPROC,
	gentextures: PFNGLGENTEXTURESPROC,
	istexture: PFNGLISTEXTUREPROC,
}

impl GL_1_1 for Version11 {
	#[inline(always)]
	fn glDrawArrays(&self, mode: GLenum, first: GLint, count: GLsizei) {
		(self.drawarrays)(mode, first, count)
	}
	#[inline(always)]
	fn glDrawElements(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void) {
		(self.drawelements)(mode, count, type_, indices)
	}
	#[inline(always)]
	fn glGetPointerv(&self, pname: GLenum, params: *mut *mut c_void) {
		(self.getpointerv)(pname, params)
	}
	#[inline(always)]
	fn glPolygonOffset(&self, factor: GLfloat, units: GLfloat) {
		(self.polygonoffset)(factor, units)
	}
	#[inline(always)]
	fn glCopyTexImage1D(&self, target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint) {
		(self.copyteximage1d)(target, level, internalformat, x, y, width, border)
	}
	#[inline(always)]
	fn glCopyTexImage2D(&self, target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint) {
		(self.copyteximage2d)(target, level, internalformat, x, y, width, height, border)
	}
	#[inline(always)]
	fn glCopyTexSubImage1D(&self, target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) {
		(self.copytexsubimage1d)(target, level, xoffset, x, y, width)
	}
	#[inline(always)]
	fn glCopyTexSubImage2D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
		(self.copytexsubimage2d)(target, level, xoffset, yoffset, x, y, width, height)
	}
	#[inline(always)]
	fn glTexSubImage1D(&self, target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) {
		(self.texsubimage1d)(target, level, xoffset, width, format, type_, pixels)
	}
	#[inline(always)]
	fn glTexSubImage2D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) {
		(self.texsubimage2d)(target, level, xoffset, yoffset, width, height, format, type_, pixels)
	}
	#[inline(always)]
	fn glBindTexture(&self, target: GLenum, texture: GLuint) {
		(self.bindtexture)(target, texture)
	}
	#[inline(always)]
	fn glDeleteTextures(&self, n: GLsizei, textures: *const GLuint) {
		(self.deletetextures)(n, textures)
	}
	#[inline(always)]
	fn glGenTextures(&self, n: GLsizei, textures: *mut GLuint) {
		(self.gentextures)(n, textures)
	}
	#[inline(always)]
	fn glIsTexture(&self, texture: GLuint) -> GLboolean {
		(self.istexture)(texture)
	}
}

impl Version11 {
	pub fn new(base: impl GL_1_0, get_proc_address: &impl Fn(&'static str) -> *const c_void) -> Self {
		let (spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 10100 {
			return Self::default();
		}
		Self {
			available: true,
			drawarrays: unsafe{transmute(get_proc_address("glDrawArrays"))},
			drawelements: unsafe{transmute(get_proc_address("glDrawElements"))},
			getpointerv: unsafe{transmute(get_proc_address("glGetPointerv"))},
			polygonoffset: unsafe{transmute(get_proc_address("glPolygonOffset"))},
			copyteximage1d: unsafe{transmute(get_proc_address("glCopyTexImage1D"))},
			copyteximage2d: unsafe{transmute(get_proc_address("glCopyTexImage2D"))},
			copytexsubimage1d: unsafe{transmute(get_proc_address("glCopyTexSubImage1D"))},
			copytexsubimage2d: unsafe{transmute(get_proc_address("glCopyTexSubImage2D"))},
			texsubimage1d: unsafe{transmute(get_proc_address("glTexSubImage1D"))},
			texsubimage2d: unsafe{transmute(get_proc_address("glTexSubImage2D"))},
			bindtexture: unsafe{transmute(get_proc_address("glBindTexture"))},
			deletetextures: unsafe{transmute(get_proc_address("glDeleteTextures"))},
			gentextures: unsafe{transmute(get_proc_address("glGenTextures"))},
			istexture: unsafe{transmute(get_proc_address("glIsTexture"))},
		}
	}
	#[inline(always)]
	pub fn get_available(&self) -> bool {
		self.available
	}
}

impl Default for Version11 {
	fn default() -> Self {
		Self {
			available: false,
			drawarrays: null(),
			drawelements: null(),
			getpointerv: null(),
			polygonoffset: null(),
			copyteximage1d: null(),
			copyteximage2d: null(),
			copytexsubimage1d: null(),
			copytexsubimage2d: null(),
			texsubimage1d: null(),
			texsubimage2d: null(),
			bindtexture: null(),
			deletetextures: null(),
			gentextures: null(),
			istexture: null(),
		}
	}
}

type PFNGLDRAWRANGEELEMENTSPROC = extern "system" fn(GLenum, GLuint, GLuint, GLsizei, GLenum, *const c_void);
type PFNGLTEXIMAGE3DPROC = extern "system" fn(GLenum, GLint, GLint, GLsizei, GLsizei, GLsizei, GLint, GLenum, GLenum, *const c_void);
type PFNGLTEXSUBIMAGE3DPROC = extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLenum, *const c_void);
type PFNGLCOPYTEXSUBIMAGE3DPROC = extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei);
const UNSIGNED_BYTE_3_3_2: GLenum = 0x8032;
const UNSIGNED_SHORT_4_4_4_4: GLenum = 0x8033;
const UNSIGNED_SHORT_5_5_5_1: GLenum = 0x8034;
const UNSIGNED_INT_8_8_8_8: GLenum = 0x8035;
const UNSIGNED_INT_10_10_10_2: GLenum = 0x8036;
const TEXTURE_BINDING_3D: GLenum = 0x806A;
const PACK_SKIP_IMAGES: GLenum = 0x806B;
const PACK_IMAGE_HEIGHT: GLenum = 0x806C;
const UNPACK_SKIP_IMAGES: GLenum = 0x806D;
const UNPACK_IMAGE_HEIGHT: GLenum = 0x806E;
const TEXTURE_3D: GLenum = 0x806F;
const PROXY_TEXTURE_3D: GLenum = 0x8070;
const TEXTURE_DEPTH: GLenum = 0x8071;
const TEXTURE_WRAP_R: GLenum = 0x8072;
const MAX_3D_TEXTURE_SIZE: GLenum = 0x8073;
const UNSIGNED_BYTE_2_3_3_REV: GLenum = 0x8362;
const UNSIGNED_SHORT_5_6_5: GLenum = 0x8363;
const UNSIGNED_SHORT_5_6_5_REV: GLenum = 0x8364;
const UNSIGNED_SHORT_4_4_4_4_REV: GLenum = 0x8365;
const UNSIGNED_SHORT_1_5_5_5_REV: GLenum = 0x8366;
const UNSIGNED_INT_8_8_8_8_REV: GLenum = 0x8367;
const UNSIGNED_INT_2_10_10_10_REV: GLenum = 0x8368;
const BGR: GLenum = 0x80E0;
const BGRA: GLenum = 0x80E1;
const MAX_ELEMENTS_VERTICES: GLenum = 0x80E8;
const MAX_ELEMENTS_INDICES: GLenum = 0x80E9;
const CLAMP_TO_EDGE: GLint = 0x812F;
const TEXTURE_MIN_LOD: GLenum = 0x813A;
const TEXTURE_MAX_LOD: GLenum = 0x813B;
const TEXTURE_BASE_LEVEL: GLenum = 0x813C;
const TEXTURE_MAX_LEVEL: GLenum = 0x813D;
const SMOOTH_POINT_SIZE_RANGE: GLenum = 0x0B12;
const SMOOTH_POINT_SIZE_GRANULARITY: GLenum = 0x0B13;
const SMOOTH_LINE_WIDTH_RANGE: GLenum = 0x0B22;
const SMOOTH_LINE_WIDTH_GRANULARITY: GLenum = 0x0B23;
const ALIASED_LINE_WIDTH_RANGE: GLenum = 0x846E;
const RESCALE_NORMAL: GLenum = 0x803A;
const LIGHT_MODEL_COLOR_CONTROL: GLenum = 0x81F8;
const SINGLE_COLOR: GLenum = 0x81F9;
const SEPARATE_SPECULAR_COLOR: GLenum = 0x81FA;
const ALIASED_POINT_SIZE_RANGE: GLenum = 0x846D;

pub trait GL_1_2 {
	fn glDrawRangeElements(&self, _: GLenum, _: GLuint, _: GLuint, _: GLsizei, _: GLenum, _: *const c_void) {
		panic!("OpenGL function pointer of `glDrawRangeElements` is NULL");
	}
	fn glTexImage3D(&self, _: GLenum, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLsizei, _: GLint, _: GLenum, _: GLenum, _: *const c_void) {
		panic!("OpenGL function pointer of `glTexImage3D` is NULL");
	}
	fn glTexSubImage3D(&self, _: GLenum, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLsizei, _: GLenum, _: GLenum, _: *const c_void) {
		panic!("OpenGL function pointer of `glTexSubImage3D` is NULL");
	}
	fn glCopyTexSubImage3D(&self, _: GLenum, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei) {
		panic!("OpenGL function pointer of `glCopyTexSubImage3D` is NULL");
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Version12 {
	available: bool,
	drawrangeelements: PFNGLDRAWRANGEELEMENTSPROC,
	teximage3d: PFNGLTEXIMAGE3DPROC,
	texsubimage3d: PFNGLTEXSUBIMAGE3DPROC,
	copytexsubimage3d: PFNGLCOPYTEXSUBIMAGE3DPROC,
}

impl GL_1_2 for Version12 {
	#[inline(always)]
	fn glDrawRangeElements(&self, mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const c_void) {
		(self.drawrangeelements)(mode, start, end, count, type_, indices)
	}
	#[inline(always)]
	fn glTexImage3D(&self, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void) {
		(self.teximage3d)(target, level, internalformat, width, height, depth, border, format, type_, pixels)
	}
	#[inline(always)]
	fn glTexSubImage3D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) {
		(self.texsubimage3d)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels)
	}
	#[inline(always)]
	fn glCopyTexSubImage3D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
		(self.copytexsubimage3d)(target, level, xoffset, yoffset, zoffset, x, y, width, height)
	}
}

impl Version12 {
	pub fn new(base: impl GL_1_0, get_proc_address: &impl Fn(&'static str) -> *const c_void) -> Self {
		let (spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 10200 {
			return Self::default();
		}
		Self {
			available: true,
			drawrangeelements: unsafe{transmute(get_proc_address("glDrawRangeElements"))},
			teximage3d: unsafe{transmute(get_proc_address("glTexImage3D"))},
			texsubimage3d: unsafe{transmute(get_proc_address("glTexSubImage3D"))},
			copytexsubimage3d: unsafe{transmute(get_proc_address("glCopyTexSubImage3D"))},
		}
	}
	#[inline(always)]
	pub fn get_available(&self) -> bool {
		self.available
	}
}

impl Default for Version12 {
	fn default() -> Self {
		Self {
			available: false,
			drawrangeelements: null(),
			teximage3d: null(),
			texsubimage3d: null(),
			copytexsubimage3d: null(),
		}
	}
}

type PFNGLACTIVETEXTUREPROC = extern "system" fn(GLenum);
type PFNGLSAMPLECOVERAGEPROC = extern "system" fn(GLfloat, GLboolean);
type PFNGLCOMPRESSEDTEXIMAGE3DPROC = extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLsizei, GLsizei, GLint, GLsizei, *const c_void);
type PFNGLCOMPRESSEDTEXIMAGE2DPROC = extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLsizei, GLint, GLsizei, *const c_void);
type PFNGLCOMPRESSEDTEXIMAGE1DPROC = extern "system" fn(GLenum, GLint, GLenum, GLsizei, GLint, GLsizei, *const c_void);
type PFNGLCOMPRESSEDTEXSUBIMAGE3DPROC = extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLsizei, *const c_void);
type PFNGLCOMPRESSEDTEXSUBIMAGE2DPROC = extern "system" fn(GLenum, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLsizei, *const c_void);
type PFNGLCOMPRESSEDTEXSUBIMAGE1DPROC = extern "system" fn(GLenum, GLint, GLint, GLsizei, GLenum, GLsizei, *const c_void);
type PFNGLGETCOMPRESSEDTEXIMAGEPROC = extern "system" fn(GLenum, GLint, *mut c_void);
type PFNGLCLIENTACTIVETEXTUREPROC = extern "system" fn(GLenum);
type PFNGLMULTITEXCOORD1DPROC = extern "system" fn(GLenum, GLdouble);
type PFNGLMULTITEXCOORD1DVPROC = extern "system" fn(GLenum, *const GLdouble);
type PFNGLMULTITEXCOORD1FPROC = extern "system" fn(GLenum, GLfloat);
type PFNGLMULTITEXCOORD1FVPROC = extern "system" fn(GLenum, *const GLfloat);
type PFNGLMULTITEXCOORD1IPROC = extern "system" fn(GLenum, GLint);
type PFNGLMULTITEXCOORD1IVPROC = extern "system" fn(GLenum, *const GLint);
type PFNGLMULTITEXCOORD1SPROC = extern "system" fn(GLenum, GLshort);
type PFNGLMULTITEXCOORD1SVPROC = extern "system" fn(GLenum, *const GLshort);
type PFNGLMULTITEXCOORD2DPROC = extern "system" fn(GLenum, GLdouble, GLdouble);
type PFNGLMULTITEXCOORD2DVPROC = extern "system" fn(GLenum, *const GLdouble);
type PFNGLMULTITEXCOORD2FPROC = extern "system" fn(GLenum, GLfloat, GLfloat);
type PFNGLMULTITEXCOORD2FVPROC = extern "system" fn(GLenum, *const GLfloat);
type PFNGLMULTITEXCOORD2IPROC = extern "system" fn(GLenum, GLint, GLint);
type PFNGLMULTITEXCOORD2IVPROC = extern "system" fn(GLenum, *const GLint);
type PFNGLMULTITEXCOORD2SPROC = extern "system" fn(GLenum, GLshort, GLshort);
type PFNGLMULTITEXCOORD2SVPROC = extern "system" fn(GLenum, *const GLshort);
type PFNGLMULTITEXCOORD3DPROC = extern "system" fn(GLenum, GLdouble, GLdouble, GLdouble);
type PFNGLMULTITEXCOORD3DVPROC = extern "system" fn(GLenum, *const GLdouble);
type PFNGLMULTITEXCOORD3FPROC = extern "system" fn(GLenum, GLfloat, GLfloat, GLfloat);
type PFNGLMULTITEXCOORD3FVPROC = extern "system" fn(GLenum, *const GLfloat);
type PFNGLMULTITEXCOORD3IPROC = extern "system" fn(GLenum, GLint, GLint, GLint);
type PFNGLMULTITEXCOORD3IVPROC = extern "system" fn(GLenum, *const GLint);
type PFNGLMULTITEXCOORD3SPROC = extern "system" fn(GLenum, GLshort, GLshort, GLshort);
type PFNGLMULTITEXCOORD3SVPROC = extern "system" fn(GLenum, *const GLshort);
type PFNGLMULTITEXCOORD4DPROC = extern "system" fn(GLenum, GLdouble, GLdouble, GLdouble, GLdouble);
type PFNGLMULTITEXCOORD4DVPROC = extern "system" fn(GLenum, *const GLdouble);
type PFNGLMULTITEXCOORD4FPROC = extern "system" fn(GLenum, GLfloat, GLfloat, GLfloat, GLfloat);
type PFNGLMULTITEXCOORD4FVPROC = extern "system" fn(GLenum, *const GLfloat);
type PFNGLMULTITEXCOORD4IPROC = extern "system" fn(GLenum, GLint, GLint, GLint, GLint);
type PFNGLMULTITEXCOORD4IVPROC = extern "system" fn(GLenum, *const GLint);
type PFNGLMULTITEXCOORD4SPROC = extern "system" fn(GLenum, GLshort, GLshort, GLshort, GLshort);
type PFNGLMULTITEXCOORD4SVPROC = extern "system" fn(GLenum, *const GLshort);
type PFNGLLOADTRANSPOSEMATRIXFPROC = extern "system" fn(*const GLfloat);
type PFNGLLOADTRANSPOSEMATRIXDPROC = extern "system" fn(*const GLdouble);
type PFNGLMULTTRANSPOSEMATRIXFPROC = extern "system" fn(*const GLfloat);
type PFNGLMULTTRANSPOSEMATRIXDPROC = extern "system" fn(*const GLdouble);
const TEXTURE0: GLenum = 0x84C0;
const TEXTURE1: GLenum = 0x84C1;
const TEXTURE2: GLenum = 0x84C2;
const TEXTURE3: GLenum = 0x84C3;
const TEXTURE4: GLenum = 0x84C4;
const TEXTURE5: GLenum = 0x84C5;
const TEXTURE6: GLenum = 0x84C6;
const TEXTURE7: GLenum = 0x84C7;
const TEXTURE8: GLenum = 0x84C8;
const TEXTURE9: GLenum = 0x84C9;
const TEXTURE10: GLenum = 0x84CA;
const TEXTURE11: GLenum = 0x84CB;
const TEXTURE12: GLenum = 0x84CC;
const TEXTURE13: GLenum = 0x84CD;
const TEXTURE14: GLenum = 0x84CE;
const TEXTURE15: GLenum = 0x84CF;
const TEXTURE16: GLenum = 0x84D0;
const TEXTURE17: GLenum = 0x84D1;
const TEXTURE18: GLenum = 0x84D2;
const TEXTURE19: GLenum = 0x84D3;
const TEXTURE20: GLenum = 0x84D4;
const TEXTURE21: GLenum = 0x84D5;
const TEXTURE22: GLenum = 0x84D6;
const TEXTURE23: GLenum = 0x84D7;
const TEXTURE24: GLenum = 0x84D8;
const TEXTURE25: GLenum = 0x84D9;
const TEXTURE26: GLenum = 0x84DA;
const TEXTURE27: GLenum = 0x84DB;
const TEXTURE28: GLenum = 0x84DC;
const TEXTURE29: GLenum = 0x84DD;
const TEXTURE30: GLenum = 0x84DE;
const TEXTURE31: GLenum = 0x84DF;
const ACTIVE_TEXTURE: GLenum = 0x84E0;
const MULTISAMPLE: GLenum = 0x809D;
const SAMPLE_ALPHA_TO_COVERAGE: GLenum = 0x809E;
const SAMPLE_ALPHA_TO_ONE: GLenum = 0x809F;
const SAMPLE_COVERAGE: GLenum = 0x80A0;
const SAMPLE_BUFFERS: GLenum = 0x80A8;
const SAMPLES: GLenum = 0x80A9;
const SAMPLE_COVERAGE_VALUE: GLenum = 0x80AA;
const SAMPLE_COVERAGE_INVERT: GLenum = 0x80AB;
const TEXTURE_CUBE_MAP: GLenum = 0x8513;
const TEXTURE_BINDING_CUBE_MAP: GLenum = 0x8514;
const TEXTURE_CUBE_MAP_POSITIVE_X: GLenum = 0x8515;
const TEXTURE_CUBE_MAP_NEGATIVE_X: GLenum = 0x8516;
const TEXTURE_CUBE_MAP_POSITIVE_Y: GLenum = 0x8517;
const TEXTURE_CUBE_MAP_NEGATIVE_Y: GLenum = 0x8518;
const TEXTURE_CUBE_MAP_POSITIVE_Z: GLenum = 0x8519;
const TEXTURE_CUBE_MAP_NEGATIVE_Z: GLenum = 0x851A;
const PROXY_TEXTURE_CUBE_MAP: GLenum = 0x851B;
const MAX_CUBE_MAP_TEXTURE_SIZE: GLenum = 0x851C;
const COMPRESSED_RGB: GLenum = 0x84ED;
const COMPRESSED_RGBA: GLenum = 0x84EE;
const TEXTURE_COMPRESSION_HINT: GLenum = 0x84EF;
const TEXTURE_COMPRESSED_IMAGE_SIZE: GLenum = 0x86A0;
const TEXTURE_COMPRESSED: GLenum = 0x86A1;
const NUM_COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A2;
const COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A3;
const CLAMP_TO_BORDER: GLint = 0x812D;
const CLIENT_ACTIVE_TEXTURE: GLenum = 0x84E1;
const MAX_TEXTURE_UNITS: GLenum = 0x84E2;
const TRANSPOSE_MODELVIEW_MATRIX: GLenum = 0x84E3;
const TRANSPOSE_PROJECTION_MATRIX: GLenum = 0x84E4;
const TRANSPOSE_TEXTURE_MATRIX: GLenum = 0x84E5;
const TRANSPOSE_COLOR_MATRIX: GLenum = 0x84E6;
const MULTISAMPLE_BIT: GLbitfield = 0x20000000;
const NORMAL_MAP: GLenum = 0x8511;
const REFLECTION_MAP: GLenum = 0x8512;
const COMPRESSED_ALPHA: GLenum = 0x84E9;
const COMPRESSED_LUMINANCE: GLenum = 0x84EA;
const COMPRESSED_LUMINANCE_ALPHA: GLenum = 0x84EB;
const COMPRESSED_INTENSITY: GLenum = 0x84EC;
const COMBINE: GLenum = 0x8570;
const COMBINE_RGB: GLenum = 0x8571;
const COMBINE_ALPHA: GLenum = 0x8572;
const SOURCE0_RGB: GLenum = 0x8580;
const SOURCE1_RGB: GLenum = 0x8581;
const SOURCE2_RGB: GLenum = 0x8582;
const SOURCE0_ALPHA: GLenum = 0x8588;
const SOURCE1_ALPHA: GLenum = 0x8589;
const SOURCE2_ALPHA: GLenum = 0x858A;
const OPERAND0_RGB: GLenum = 0x8590;
const OPERAND1_RGB: GLenum = 0x8591;
const OPERAND2_RGB: GLenum = 0x8592;
const OPERAND0_ALPHA: GLenum = 0x8598;
const OPERAND1_ALPHA: GLenum = 0x8599;
const OPERAND2_ALPHA: GLenum = 0x859A;
const RGB_SCALE: GLenum = 0x8573;
const ADD_SIGNED: GLenum = 0x8574;
const INTERPOLATE: GLenum = 0x8575;
const SUBTRACT: GLenum = 0x84E7;
const CONSTANT: GLenum = 0x8576;
const PRIMARY_COLOR: GLenum = 0x8577;
const PREVIOUS: GLenum = 0x8578;
const DOT3_RGB: GLenum = 0x86AE;
const DOT3_RGBA: GLenum = 0x86AF;

pub trait GL_1_3 {
	fn glActiveTexture(&self, _: GLenum) {
		panic!("OpenGL function pointer of `glActiveTexture` is NULL");
	}
	fn glSampleCoverage(&self, _: GLfloat, _: GLboolean) {
		panic!("OpenGL function pointer of `glSampleCoverage` is NULL");
	}
	fn glCompressedTexImage3D(&self, _: GLenum, _: GLint, _: GLenum, _: GLsizei, _: GLsizei, _: GLsizei, _: GLint, _: GLsizei, _: *const c_void) {
		panic!("OpenGL function pointer of `glCompressedTexImage3D` is NULL");
	}
	fn glCompressedTexImage2D(&self, _: GLenum, _: GLint, _: GLenum, _: GLsizei, _: GLsizei, _: GLint, _: GLsizei, _: *const c_void) {
		panic!("OpenGL function pointer of `glCompressedTexImage2D` is NULL");
	}
	fn glCompressedTexImage1D(&self, _: GLenum, _: GLint, _: GLenum, _: GLsizei, _: GLint, _: GLsizei, _: *const c_void) {
		panic!("OpenGL function pointer of `glCompressedTexImage1D` is NULL");
	}
	fn glCompressedTexSubImage3D(&self, _: GLenum, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLsizei, _: GLenum, _: GLsizei, _: *const c_void) {
		panic!("OpenGL function pointer of `glCompressedTexSubImage3D` is NULL");
	}
	fn glCompressedTexSubImage2D(&self, _: GLenum, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLenum, _: GLsizei, _: *const c_void) {
		panic!("OpenGL function pointer of `glCompressedTexSubImage2D` is NULL");
	}
	fn glCompressedTexSubImage1D(&self, _: GLenum, _: GLint, _: GLint, _: GLsizei, _: GLenum, _: GLsizei, _: *const c_void) {
		panic!("OpenGL function pointer of `glCompressedTexSubImage1D` is NULL");
	}
	fn glGetCompressedTexImage(&self, _: GLenum, _: GLint, _: *mut c_void) {
		panic!("OpenGL function pointer of `glGetCompressedTexImage` is NULL");
	}
	fn glClientActiveTexture(&self, _: GLenum) {
		panic!("OpenGL function pointer of `glClientActiveTexture` is NULL");
	}
	fn glMultiTexCoord1d(&self, _: GLenum, _: GLdouble) {
		panic!("OpenGL function pointer of `glMultiTexCoord1d` is NULL");
	}
	fn glMultiTexCoord1dv(&self, _: GLenum, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glMultiTexCoord1dv` is NULL");
	}
	fn glMultiTexCoord1f(&self, _: GLenum, _: GLfloat) {
		panic!("OpenGL function pointer of `glMultiTexCoord1f` is NULL");
	}
	fn glMultiTexCoord1fv(&self, _: GLenum, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glMultiTexCoord1fv` is NULL");
	}
	fn glMultiTexCoord1i(&self, _: GLenum, _: GLint) {
		panic!("OpenGL function pointer of `glMultiTexCoord1i` is NULL");
	}
	fn glMultiTexCoord1iv(&self, _: GLenum, _: *const GLint) {
		panic!("OpenGL function pointer of `glMultiTexCoord1iv` is NULL");
	}
	fn glMultiTexCoord1s(&self, _: GLenum, _: GLshort) {
		panic!("OpenGL function pointer of `glMultiTexCoord1s` is NULL");
	}
	fn glMultiTexCoord1sv(&self, _: GLenum, _: *const GLshort) {
		panic!("OpenGL function pointer of `glMultiTexCoord1sv` is NULL");
	}
	fn glMultiTexCoord2d(&self, _: GLenum, _: GLdouble, _: GLdouble) {
		panic!("OpenGL function pointer of `glMultiTexCoord2d` is NULL");
	}
	fn glMultiTexCoord2dv(&self, _: GLenum, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glMultiTexCoord2dv` is NULL");
	}
	fn glMultiTexCoord2f(&self, _: GLenum, _: GLfloat, _: GLfloat) {
		panic!("OpenGL function pointer of `glMultiTexCoord2f` is NULL");
	}
	fn glMultiTexCoord2fv(&self, _: GLenum, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glMultiTexCoord2fv` is NULL");
	}
	fn glMultiTexCoord2i(&self, _: GLenum, _: GLint, _: GLint) {
		panic!("OpenGL function pointer of `glMultiTexCoord2i` is NULL");
	}
	fn glMultiTexCoord2iv(&self, _: GLenum, _: *const GLint) {
		panic!("OpenGL function pointer of `glMultiTexCoord2iv` is NULL");
	}
	fn glMultiTexCoord2s(&self, _: GLenum, _: GLshort, _: GLshort) {
		panic!("OpenGL function pointer of `glMultiTexCoord2s` is NULL");
	}
	fn glMultiTexCoord2sv(&self, _: GLenum, _: *const GLshort) {
		panic!("OpenGL function pointer of `glMultiTexCoord2sv` is NULL");
	}
	fn glMultiTexCoord3d(&self, _: GLenum, _: GLdouble, _: GLdouble, _: GLdouble) {
		panic!("OpenGL function pointer of `glMultiTexCoord3d` is NULL");
	}
	fn glMultiTexCoord3dv(&self, _: GLenum, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glMultiTexCoord3dv` is NULL");
	}
	fn glMultiTexCoord3f(&self, _: GLenum, _: GLfloat, _: GLfloat, _: GLfloat) {
		panic!("OpenGL function pointer of `glMultiTexCoord3f` is NULL");
	}
	fn glMultiTexCoord3fv(&self, _: GLenum, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glMultiTexCoord3fv` is NULL");
	}
	fn glMultiTexCoord3i(&self, _: GLenum, _: GLint, _: GLint, _: GLint) {
		panic!("OpenGL function pointer of `glMultiTexCoord3i` is NULL");
	}
	fn glMultiTexCoord3iv(&self, _: GLenum, _: *const GLint) {
		panic!("OpenGL function pointer of `glMultiTexCoord3iv` is NULL");
	}
	fn glMultiTexCoord3s(&self, _: GLenum, _: GLshort, _: GLshort, _: GLshort) {
		panic!("OpenGL function pointer of `glMultiTexCoord3s` is NULL");
	}
	fn glMultiTexCoord3sv(&self, _: GLenum, _: *const GLshort) {
		panic!("OpenGL function pointer of `glMultiTexCoord3sv` is NULL");
	}
	fn glMultiTexCoord4d(&self, _: GLenum, _: GLdouble, _: GLdouble, _: GLdouble, _: GLdouble) {
		panic!("OpenGL function pointer of `glMultiTexCoord4d` is NULL");
	}
	fn glMultiTexCoord4dv(&self, _: GLenum, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glMultiTexCoord4dv` is NULL");
	}
	fn glMultiTexCoord4f(&self, _: GLenum, _: GLfloat, _: GLfloat, _: GLfloat, _: GLfloat) {
		panic!("OpenGL function pointer of `glMultiTexCoord4f` is NULL");
	}
	fn glMultiTexCoord4fv(&self, _: GLenum, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glMultiTexCoord4fv` is NULL");
	}
	fn glMultiTexCoord4i(&self, _: GLenum, _: GLint, _: GLint, _: GLint, _: GLint) {
		panic!("OpenGL function pointer of `glMultiTexCoord4i` is NULL");
	}
	fn glMultiTexCoord4iv(&self, _: GLenum, _: *const GLint) {
		panic!("OpenGL function pointer of `glMultiTexCoord4iv` is NULL");
	}
	fn glMultiTexCoord4s(&self, _: GLenum, _: GLshort, _: GLshort, _: GLshort, _: GLshort) {
		panic!("OpenGL function pointer of `glMultiTexCoord4s` is NULL");
	}
	fn glMultiTexCoord4sv(&self, _: GLenum, _: *const GLshort) {
		panic!("OpenGL function pointer of `glMultiTexCoord4sv` is NULL");
	}
	fn glLoadTransposeMatrixf(&self, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glLoadTransposeMatrixf` is NULL");
	}
	fn glLoadTransposeMatrixd(&self, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glLoadTransposeMatrixd` is NULL");
	}
	fn glMultTransposeMatrixf(&self, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glMultTransposeMatrixf` is NULL");
	}
	fn glMultTransposeMatrixd(&self, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glMultTransposeMatrixd` is NULL");
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Version13 {
	available: bool,
	activetexture: PFNGLACTIVETEXTUREPROC,
	samplecoverage: PFNGLSAMPLECOVERAGEPROC,
	compressedteximage3d: PFNGLCOMPRESSEDTEXIMAGE3DPROC,
	compressedteximage2d: PFNGLCOMPRESSEDTEXIMAGE2DPROC,
	compressedteximage1d: PFNGLCOMPRESSEDTEXIMAGE1DPROC,
	compressedtexsubimage3d: PFNGLCOMPRESSEDTEXSUBIMAGE3DPROC,
	compressedtexsubimage2d: PFNGLCOMPRESSEDTEXSUBIMAGE2DPROC,
	compressedtexsubimage1d: PFNGLCOMPRESSEDTEXSUBIMAGE1DPROC,
	getcompressedteximage: PFNGLGETCOMPRESSEDTEXIMAGEPROC,
	clientactivetexture: PFNGLCLIENTACTIVETEXTUREPROC,
	multitexcoord1d: PFNGLMULTITEXCOORD1DPROC,
	multitexcoord1dv: PFNGLMULTITEXCOORD1DVPROC,
	multitexcoord1f: PFNGLMULTITEXCOORD1FPROC,
	multitexcoord1fv: PFNGLMULTITEXCOORD1FVPROC,
	multitexcoord1i: PFNGLMULTITEXCOORD1IPROC,
	multitexcoord1iv: PFNGLMULTITEXCOORD1IVPROC,
	multitexcoord1s: PFNGLMULTITEXCOORD1SPROC,
	multitexcoord1sv: PFNGLMULTITEXCOORD1SVPROC,
	multitexcoord2d: PFNGLMULTITEXCOORD2DPROC,
	multitexcoord2dv: PFNGLMULTITEXCOORD2DVPROC,
	multitexcoord2f: PFNGLMULTITEXCOORD2FPROC,
	multitexcoord2fv: PFNGLMULTITEXCOORD2FVPROC,
	multitexcoord2i: PFNGLMULTITEXCOORD2IPROC,
	multitexcoord2iv: PFNGLMULTITEXCOORD2IVPROC,
	multitexcoord2s: PFNGLMULTITEXCOORD2SPROC,
	multitexcoord2sv: PFNGLMULTITEXCOORD2SVPROC,
	multitexcoord3d: PFNGLMULTITEXCOORD3DPROC,
	multitexcoord3dv: PFNGLMULTITEXCOORD3DVPROC,
	multitexcoord3f: PFNGLMULTITEXCOORD3FPROC,
	multitexcoord3fv: PFNGLMULTITEXCOORD3FVPROC,
	multitexcoord3i: PFNGLMULTITEXCOORD3IPROC,
	multitexcoord3iv: PFNGLMULTITEXCOORD3IVPROC,
	multitexcoord3s: PFNGLMULTITEXCOORD3SPROC,
	multitexcoord3sv: PFNGLMULTITEXCOORD3SVPROC,
	multitexcoord4d: PFNGLMULTITEXCOORD4DPROC,
	multitexcoord4dv: PFNGLMULTITEXCOORD4DVPROC,
	multitexcoord4f: PFNGLMULTITEXCOORD4FPROC,
	multitexcoord4fv: PFNGLMULTITEXCOORD4FVPROC,
	multitexcoord4i: PFNGLMULTITEXCOORD4IPROC,
	multitexcoord4iv: PFNGLMULTITEXCOORD4IVPROC,
	multitexcoord4s: PFNGLMULTITEXCOORD4SPROC,
	multitexcoord4sv: PFNGLMULTITEXCOORD4SVPROC,
	loadtransposematrixf: PFNGLLOADTRANSPOSEMATRIXFPROC,
	loadtransposematrixd: PFNGLLOADTRANSPOSEMATRIXDPROC,
	multtransposematrixf: PFNGLMULTTRANSPOSEMATRIXFPROC,
	multtransposematrixd: PFNGLMULTTRANSPOSEMATRIXDPROC,
}

impl GL_1_3 for Version13 {
	#[inline(always)]
	fn glActiveTexture(&self, texture: GLenum) {
		(self.activetexture)(texture)
	}
	#[inline(always)]
	fn glSampleCoverage(&self, value: GLfloat, invert: GLboolean) {
		(self.samplecoverage)(value, invert)
	}
	#[inline(always)]
	fn glCompressedTexImage3D(&self, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void) {
		(self.compressedteximage3d)(target, level, internalformat, width, height, depth, border, imageSize, data)
	}
	#[inline(always)]
	fn glCompressedTexImage2D(&self, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void) {
		(self.compressedteximage2d)(target, level, internalformat, width, height, border, imageSize, data)
	}
	#[inline(always)]
	fn glCompressedTexImage1D(&self, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void) {
		(self.compressedteximage1d)(target, level, internalformat, width, border, imageSize, data)
	}
	#[inline(always)]
	fn glCompressedTexSubImage3D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) {
		(self.compressedtexsubimage3d)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data)
	}
	#[inline(always)]
	fn glCompressedTexSubImage2D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) {
		(self.compressedtexsubimage2d)(target, level, xoffset, yoffset, width, height, format, imageSize, data)
	}
	#[inline(always)]
	fn glCompressedTexSubImage1D(&self, target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) {
		(self.compressedtexsubimage1d)(target, level, xoffset, width, format, imageSize, data)
	}
	#[inline(always)]
	fn glGetCompressedTexImage(&self, target: GLenum, level: GLint, img: *mut c_void) {
		(self.getcompressedteximage)(target, level, img)
	}
	#[inline(always)]
	fn glClientActiveTexture(&self, texture: GLenum) {
		(self.clientactivetexture)(texture)
	}
	#[inline(always)]
	fn glMultiTexCoord1d(&self, target: GLenum, s: GLdouble) {
		(self.multitexcoord1d)(target, s)
	}
	#[inline(always)]
	fn glMultiTexCoord1dv(&self, target: GLenum, v: *const GLdouble) {
		(self.multitexcoord1dv)(target, v)
	}
	#[inline(always)]
	fn glMultiTexCoord1f(&self, target: GLenum, s: GLfloat) {
		(self.multitexcoord1f)(target, s)
	}
	#[inline(always)]
	fn glMultiTexCoord1fv(&self, target: GLenum, v: *const GLfloat) {
		(self.multitexcoord1fv)(target, v)
	}
	#[inline(always)]
	fn glMultiTexCoord1i(&self, target: GLenum, s: GLint) {
		(self.multitexcoord1i)(target, s)
	}
	#[inline(always)]
	fn glMultiTexCoord1iv(&self, target: GLenum, v: *const GLint) {
		(self.multitexcoord1iv)(target, v)
	}
	#[inline(always)]
	fn glMultiTexCoord1s(&self, target: GLenum, s: GLshort) {
		(self.multitexcoord1s)(target, s)
	}
	#[inline(always)]
	fn glMultiTexCoord1sv(&self, target: GLenum, v: *const GLshort) {
		(self.multitexcoord1sv)(target, v)
	}
	#[inline(always)]
	fn glMultiTexCoord2d(&self, target: GLenum, s: GLdouble, t: GLdouble) {
		(self.multitexcoord2d)(target, s, t)
	}
	#[inline(always)]
	fn glMultiTexCoord2dv(&self, target: GLenum, v: *const GLdouble) {
		(self.multitexcoord2dv)(target, v)
	}
	#[inline(always)]
	fn glMultiTexCoord2f(&self, target: GLenum, s: GLfloat, t: GLfloat) {
		(self.multitexcoord2f)(target, s, t)
	}
	#[inline(always)]
	fn glMultiTexCoord2fv(&self, target: GLenum, v: *const GLfloat) {
		(self.multitexcoord2fv)(target, v)
	}
	#[inline(always)]
	fn glMultiTexCoord2i(&self, target: GLenum, s: GLint, t: GLint) {
		(self.multitexcoord2i)(target, s, t)
	}
	#[inline(always)]
	fn glMultiTexCoord2iv(&self, target: GLenum, v: *const GLint) {
		(self.multitexcoord2iv)(target, v)
	}
	#[inline(always)]
	fn glMultiTexCoord2s(&self, target: GLenum, s: GLshort, t: GLshort) {
		(self.multitexcoord2s)(target, s, t)
	}
	#[inline(always)]
	fn glMultiTexCoord2sv(&self, target: GLenum, v: *const GLshort) {
		(self.multitexcoord2sv)(target, v)
	}
	#[inline(always)]
	fn glMultiTexCoord3d(&self, target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble) {
		(self.multitexcoord3d)(target, s, t, r)
	}
	#[inline(always)]
	fn glMultiTexCoord3dv(&self, target: GLenum, v: *const GLdouble) {
		(self.multitexcoord3dv)(target, v)
	}
	#[inline(always)]
	fn glMultiTexCoord3f(&self, target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat) {
		(self.multitexcoord3f)(target, s, t, r)
	}
	#[inline(always)]
	fn glMultiTexCoord3fv(&self, target: GLenum, v: *const GLfloat) {
		(self.multitexcoord3fv)(target, v)
	}
	#[inline(always)]
	fn glMultiTexCoord3i(&self, target: GLenum, s: GLint, t: GLint, r: GLint) {
		(self.multitexcoord3i)(target, s, t, r)
	}
	#[inline(always)]
	fn glMultiTexCoord3iv(&self, target: GLenum, v: *const GLint) {
		(self.multitexcoord3iv)(target, v)
	}
	#[inline(always)]
	fn glMultiTexCoord3s(&self, target: GLenum, s: GLshort, t: GLshort, r: GLshort) {
		(self.multitexcoord3s)(target, s, t, r)
	}
	#[inline(always)]
	fn glMultiTexCoord3sv(&self, target: GLenum, v: *const GLshort) {
		(self.multitexcoord3sv)(target, v)
	}
	#[inline(always)]
	fn glMultiTexCoord4d(&self, target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble) {
		(self.multitexcoord4d)(target, s, t, r, q)
	}
	#[inline(always)]
	fn glMultiTexCoord4dv(&self, target: GLenum, v: *const GLdouble) {
		(self.multitexcoord4dv)(target, v)
	}
	#[inline(always)]
	fn glMultiTexCoord4f(&self, target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat) {
		(self.multitexcoord4f)(target, s, t, r, q)
	}
	#[inline(always)]
	fn glMultiTexCoord4fv(&self, target: GLenum, v: *const GLfloat) {
		(self.multitexcoord4fv)(target, v)
	}
	#[inline(always)]
	fn glMultiTexCoord4i(&self, target: GLenum, s: GLint, t: GLint, r: GLint, q: GLint) {
		(self.multitexcoord4i)(target, s, t, r, q)
	}
	#[inline(always)]
	fn glMultiTexCoord4iv(&self, target: GLenum, v: *const GLint) {
		(self.multitexcoord4iv)(target, v)
	}
	#[inline(always)]
	fn glMultiTexCoord4s(&self, target: GLenum, s: GLshort, t: GLshort, r: GLshort, q: GLshort) {
		(self.multitexcoord4s)(target, s, t, r, q)
	}
	#[inline(always)]
	fn glMultiTexCoord4sv(&self, target: GLenum, v: *const GLshort) {
		(self.multitexcoord4sv)(target, v)
	}
	#[inline(always)]
	fn glLoadTransposeMatrixf(&self, m: *const GLfloat) {
		(self.loadtransposematrixf)(m)
	}
	#[inline(always)]
	fn glLoadTransposeMatrixd(&self, m: *const GLdouble) {
		(self.loadtransposematrixd)(m)
	}
	#[inline(always)]
	fn glMultTransposeMatrixf(&self, m: *const GLfloat) {
		(self.multtransposematrixf)(m)
	}
	#[inline(always)]
	fn glMultTransposeMatrixd(&self, m: *const GLdouble) {
		(self.multtransposematrixd)(m)
	}
}

impl Version13 {
	pub fn new(base: impl GL_1_0, get_proc_address: &impl Fn(&'static str) -> *const c_void) -> Self {
		let (spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 10300 {
			return Self::default();
		}
		Self {
			available: true,
			activetexture: unsafe{transmute(get_proc_address("glActiveTexture"))},
			samplecoverage: unsafe{transmute(get_proc_address("glSampleCoverage"))},
			compressedteximage3d: unsafe{transmute(get_proc_address("glCompressedTexImage3D"))},
			compressedteximage2d: unsafe{transmute(get_proc_address("glCompressedTexImage2D"))},
			compressedteximage1d: unsafe{transmute(get_proc_address("glCompressedTexImage1D"))},
			compressedtexsubimage3d: unsafe{transmute(get_proc_address("glCompressedTexSubImage3D"))},
			compressedtexsubimage2d: unsafe{transmute(get_proc_address("glCompressedTexSubImage2D"))},
			compressedtexsubimage1d: unsafe{transmute(get_proc_address("glCompressedTexSubImage1D"))},
			getcompressedteximage: unsafe{transmute(get_proc_address("glGetCompressedTexImage"))},
			clientactivetexture: unsafe{transmute(get_proc_address("glClientActiveTexture"))},
			multitexcoord1d: unsafe{transmute(get_proc_address("glMultiTexCoord1d"))},
			multitexcoord1dv: unsafe{transmute(get_proc_address("glMultiTexCoord1dv"))},
			multitexcoord1f: unsafe{transmute(get_proc_address("glMultiTexCoord1f"))},
			multitexcoord1fv: unsafe{transmute(get_proc_address("glMultiTexCoord1fv"))},
			multitexcoord1i: unsafe{transmute(get_proc_address("glMultiTexCoord1i"))},
			multitexcoord1iv: unsafe{transmute(get_proc_address("glMultiTexCoord1iv"))},
			multitexcoord1s: unsafe{transmute(get_proc_address("glMultiTexCoord1s"))},
			multitexcoord1sv: unsafe{transmute(get_proc_address("glMultiTexCoord1sv"))},
			multitexcoord2d: unsafe{transmute(get_proc_address("glMultiTexCoord2d"))},
			multitexcoord2dv: unsafe{transmute(get_proc_address("glMultiTexCoord2dv"))},
			multitexcoord2f: unsafe{transmute(get_proc_address("glMultiTexCoord2f"))},
			multitexcoord2fv: unsafe{transmute(get_proc_address("glMultiTexCoord2fv"))},
			multitexcoord2i: unsafe{transmute(get_proc_address("glMultiTexCoord2i"))},
			multitexcoord2iv: unsafe{transmute(get_proc_address("glMultiTexCoord2iv"))},
			multitexcoord2s: unsafe{transmute(get_proc_address("glMultiTexCoord2s"))},
			multitexcoord2sv: unsafe{transmute(get_proc_address("glMultiTexCoord2sv"))},
			multitexcoord3d: unsafe{transmute(get_proc_address("glMultiTexCoord3d"))},
			multitexcoord3dv: unsafe{transmute(get_proc_address("glMultiTexCoord3dv"))},
			multitexcoord3f: unsafe{transmute(get_proc_address("glMultiTexCoord3f"))},
			multitexcoord3fv: unsafe{transmute(get_proc_address("glMultiTexCoord3fv"))},
			multitexcoord3i: unsafe{transmute(get_proc_address("glMultiTexCoord3i"))},
			multitexcoord3iv: unsafe{transmute(get_proc_address("glMultiTexCoord3iv"))},
			multitexcoord3s: unsafe{transmute(get_proc_address("glMultiTexCoord3s"))},
			multitexcoord3sv: unsafe{transmute(get_proc_address("glMultiTexCoord3sv"))},
			multitexcoord4d: unsafe{transmute(get_proc_address("glMultiTexCoord4d"))},
			multitexcoord4dv: unsafe{transmute(get_proc_address("glMultiTexCoord4dv"))},
			multitexcoord4f: unsafe{transmute(get_proc_address("glMultiTexCoord4f"))},
			multitexcoord4fv: unsafe{transmute(get_proc_address("glMultiTexCoord4fv"))},
			multitexcoord4i: unsafe{transmute(get_proc_address("glMultiTexCoord4i"))},
			multitexcoord4iv: unsafe{transmute(get_proc_address("glMultiTexCoord4iv"))},
			multitexcoord4s: unsafe{transmute(get_proc_address("glMultiTexCoord4s"))},
			multitexcoord4sv: unsafe{transmute(get_proc_address("glMultiTexCoord4sv"))},
			loadtransposematrixf: unsafe{transmute(get_proc_address("glLoadTransposeMatrixf"))},
			loadtransposematrixd: unsafe{transmute(get_proc_address("glLoadTransposeMatrixd"))},
			multtransposematrixf: unsafe{transmute(get_proc_address("glMultTransposeMatrixf"))},
			multtransposematrixd: unsafe{transmute(get_proc_address("glMultTransposeMatrixd"))},
		}
	}
	#[inline(always)]
	pub fn get_available(&self) -> bool {
		self.available
	}
}

impl Default for Version13 {
	fn default() -> Self {
		Self {
			available: false,
			activetexture: null(),
			samplecoverage: null(),
			compressedteximage3d: null(),
			compressedteximage2d: null(),
			compressedteximage1d: null(),
			compressedtexsubimage3d: null(),
			compressedtexsubimage2d: null(),
			compressedtexsubimage1d: null(),
			getcompressedteximage: null(),
			clientactivetexture: null(),
			multitexcoord1d: null(),
			multitexcoord1dv: null(),
			multitexcoord1f: null(),
			multitexcoord1fv: null(),
			multitexcoord1i: null(),
			multitexcoord1iv: null(),
			multitexcoord1s: null(),
			multitexcoord1sv: null(),
			multitexcoord2d: null(),
			multitexcoord2dv: null(),
			multitexcoord2f: null(),
			multitexcoord2fv: null(),
			multitexcoord2i: null(),
			multitexcoord2iv: null(),
			multitexcoord2s: null(),
			multitexcoord2sv: null(),
			multitexcoord3d: null(),
			multitexcoord3dv: null(),
			multitexcoord3f: null(),
			multitexcoord3fv: null(),
			multitexcoord3i: null(),
			multitexcoord3iv: null(),
			multitexcoord3s: null(),
			multitexcoord3sv: null(),
			multitexcoord4d: null(),
			multitexcoord4dv: null(),
			multitexcoord4f: null(),
			multitexcoord4fv: null(),
			multitexcoord4i: null(),
			multitexcoord4iv: null(),
			multitexcoord4s: null(),
			multitexcoord4sv: null(),
			loadtransposematrixf: null(),
			loadtransposematrixd: null(),
			multtransposematrixf: null(),
			multtransposematrixd: null(),
		}
	}
}

type PFNGLBLENDFUNCSEPARATEPROC = extern "system" fn(GLenum, GLenum, GLenum, GLenum);
type PFNGLMULTIDRAWARRAYSPROC = extern "system" fn(GLenum, *const GLint, *const GLsizei, GLsizei);
type PFNGLMULTIDRAWELEMENTSPROC = extern "system" fn(GLenum, *const GLsizei, GLenum, *const *const c_void, GLsizei);
type PFNGLPOINTPARAMETERFPROC = extern "system" fn(GLenum, GLfloat);
type PFNGLPOINTPARAMETERFVPROC = extern "system" fn(GLenum, *const GLfloat);
type PFNGLPOINTPARAMETERIPROC = extern "system" fn(GLenum, GLint);
type PFNGLPOINTPARAMETERIVPROC = extern "system" fn(GLenum, *const GLint);
type PFNGLFOGCOORDFPROC = extern "system" fn(GLfloat);
type PFNGLFOGCOORDFVPROC = extern "system" fn(*const GLfloat);
type PFNGLFOGCOORDDPROC = extern "system" fn(GLdouble);
type PFNGLFOGCOORDDVPROC = extern "system" fn(*const GLdouble);
type PFNGLFOGCOORDPOINTERPROC = extern "system" fn(GLenum, GLsizei, *const c_void);
type PFNGLSECONDARYCOLOR3BPROC = extern "system" fn(GLbyte, GLbyte, GLbyte);
type PFNGLSECONDARYCOLOR3BVPROC = extern "system" fn(*const GLbyte);
type PFNGLSECONDARYCOLOR3DPROC = extern "system" fn(GLdouble, GLdouble, GLdouble);
type PFNGLSECONDARYCOLOR3DVPROC = extern "system" fn(*const GLdouble);
type PFNGLSECONDARYCOLOR3FPROC = extern "system" fn(GLfloat, GLfloat, GLfloat);
type PFNGLSECONDARYCOLOR3FVPROC = extern "system" fn(*const GLfloat);
type PFNGLSECONDARYCOLOR3IPROC = extern "system" fn(GLint, GLint, GLint);
type PFNGLSECONDARYCOLOR3IVPROC = extern "system" fn(*const GLint);
type PFNGLSECONDARYCOLOR3SPROC = extern "system" fn(GLshort, GLshort, GLshort);
type PFNGLSECONDARYCOLOR3SVPROC = extern "system" fn(*const GLshort);
type PFNGLSECONDARYCOLOR3UBPROC = extern "system" fn(GLubyte, GLubyte, GLubyte);
type PFNGLSECONDARYCOLOR3UBVPROC = extern "system" fn(*const GLubyte);
type PFNGLSECONDARYCOLOR3UIPROC = extern "system" fn(GLuint, GLuint, GLuint);
type PFNGLSECONDARYCOLOR3UIVPROC = extern "system" fn(*const GLuint);
type PFNGLSECONDARYCOLOR3USPROC = extern "system" fn(GLushort, GLushort, GLushort);
type PFNGLSECONDARYCOLOR3USVPROC = extern "system" fn(*const GLushort);
type PFNGLSECONDARYCOLORPOINTERPROC = extern "system" fn(GLint, GLenum, GLsizei, *const c_void);
type PFNGLWINDOWPOS2DPROC = extern "system" fn(GLdouble, GLdouble);
type PFNGLWINDOWPOS2DVPROC = extern "system" fn(*const GLdouble);
type PFNGLWINDOWPOS2FPROC = extern "system" fn(GLfloat, GLfloat);
type PFNGLWINDOWPOS2FVPROC = extern "system" fn(*const GLfloat);
type PFNGLWINDOWPOS2IPROC = extern "system" fn(GLint, GLint);
type PFNGLWINDOWPOS2IVPROC = extern "system" fn(*const GLint);
type PFNGLWINDOWPOS2SPROC = extern "system" fn(GLshort, GLshort);
type PFNGLWINDOWPOS2SVPROC = extern "system" fn(*const GLshort);
type PFNGLWINDOWPOS3DPROC = extern "system" fn(GLdouble, GLdouble, GLdouble);
type PFNGLWINDOWPOS3DVPROC = extern "system" fn(*const GLdouble);
type PFNGLWINDOWPOS3FPROC = extern "system" fn(GLfloat, GLfloat, GLfloat);
type PFNGLWINDOWPOS3FVPROC = extern "system" fn(*const GLfloat);
type PFNGLWINDOWPOS3IPROC = extern "system" fn(GLint, GLint, GLint);
type PFNGLWINDOWPOS3IVPROC = extern "system" fn(*const GLint);
type PFNGLWINDOWPOS3SPROC = extern "system" fn(GLshort, GLshort, GLshort);
type PFNGLWINDOWPOS3SVPROC = extern "system" fn(*const GLshort);
type PFNGLBLENDCOLORPROC = extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat);
type PFNGLBLENDEQUATIONPROC = extern "system" fn(GLenum);
const BLEND_DST_RGB: GLenum = 0x80C8;
const BLEND_SRC_RGB: GLenum = 0x80C9;
const BLEND_DST_ALPHA: GLenum = 0x80CA;
const BLEND_SRC_ALPHA: GLenum = 0x80CB;
const POINT_FADE_THRESHOLD_SIZE: GLenum = 0x8128;
const DEPTH_COMPONENT16: GLenum = 0x81A5;
const DEPTH_COMPONENT24: GLenum = 0x81A6;
const DEPTH_COMPONENT32: GLenum = 0x81A7;
const MIRRORED_REPEAT: GLint = 0x8370;
const MAX_TEXTURE_LOD_BIAS: GLenum = 0x84FD;
const TEXTURE_LOD_BIAS: GLenum = 0x8501;
const INCR_WRAP: GLenum = 0x8507;
const DECR_WRAP: GLenum = 0x8508;
const TEXTURE_DEPTH_SIZE: GLenum = 0x884A;
const TEXTURE_COMPARE_MODE: GLenum = 0x884C;
const TEXTURE_COMPARE_FUNC: GLenum = 0x884D;
const POINT_SIZE_MIN: GLenum = 0x8126;
const POINT_SIZE_MAX: GLenum = 0x8127;
const POINT_DISTANCE_ATTENUATION: GLenum = 0x8129;
const GENERATE_MIPMAP: GLenum = 0x8191;
const GENERATE_MIPMAP_HINT: GLenum = 0x8192;
const FOG_COORDINATE_SOURCE: GLenum = 0x8450;
const FOG_COORDINATE: GLenum = 0x8451;
const FRAGMENT_DEPTH: GLenum = 0x8452;
const CURRENT_FOG_COORDINATE: GLenum = 0x8453;
const FOG_COORDINATE_ARRAY_TYPE: GLenum = 0x8454;
const FOG_COORDINATE_ARRAY_STRIDE: GLenum = 0x8455;
const FOG_COORDINATE_ARRAY_POINTER: GLenum = 0x8456;
const FOG_COORDINATE_ARRAY: GLenum = 0x8457;
const COLOR_SUM: GLenum = 0x8458;
const CURRENT_SECONDARY_COLOR: GLenum = 0x8459;
const SECONDARY_COLOR_ARRAY_SIZE: GLenum = 0x845A;
const SECONDARY_COLOR_ARRAY_TYPE: GLenum = 0x845B;
const SECONDARY_COLOR_ARRAY_STRIDE: GLenum = 0x845C;
const SECONDARY_COLOR_ARRAY_POINTER: GLenum = 0x845D;
const SECONDARY_COLOR_ARRAY: GLenum = 0x845E;
const TEXTURE_FILTER_CONTROL: GLenum = 0x8500;
const DEPTH_TEXTURE_MODE: GLenum = 0x884B;
const COMPARE_R_TO_TEXTURE: GLenum = 0x884E;
const BLEND_COLOR: GLenum = 0x8005;
const BLEND_EQUATION: GLenum = 0x8009;
const CONSTANT_COLOR: GLenum = 0x8001;
const ONE_MINUS_CONSTANT_COLOR: GLenum = 0x8002;
const CONSTANT_ALPHA: GLenum = 0x8003;
const ONE_MINUS_CONSTANT_ALPHA: GLenum = 0x8004;
const FUNC_ADD: GLenum = 0x8006;
const FUNC_REVERSE_SUBTRACT: GLenum = 0x800B;
const FUNC_SUBTRACT: GLenum = 0x800A;
const MIN: GLenum = 0x8007;
const MAX: GLenum = 0x8008;

pub trait GL_1_4 {
	fn glBlendFuncSeparate(&self, _: GLenum, _: GLenum, _: GLenum, _: GLenum) {
		panic!("OpenGL function pointer of `glBlendFuncSeparate` is NULL");
	}
	fn glMultiDrawArrays(&self, _: GLenum, _: *const GLint, _: *const GLsizei, _: GLsizei) {
		panic!("OpenGL function pointer of `glMultiDrawArrays` is NULL");
	}
	fn glMultiDrawElements(&self, _: GLenum, _: *const GLsizei, _: GLenum, _: *const *const c_void, _: GLsizei) {
		panic!("OpenGL function pointer of `glMultiDrawElements` is NULL");
	}
	fn glPointParameterf(&self, _: GLenum, _: GLfloat) {
		panic!("OpenGL function pointer of `glPointParameterf` is NULL");
	}
	fn glPointParameterfv(&self, _: GLenum, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glPointParameterfv` is NULL");
	}
	fn glPointParameteri(&self, _: GLenum, _: GLint) {
		panic!("OpenGL function pointer of `glPointParameteri` is NULL");
	}
	fn glPointParameteriv(&self, _: GLenum, _: *const GLint) {
		panic!("OpenGL function pointer of `glPointParameteriv` is NULL");
	}
	fn glFogCoordf(&self, _: GLfloat) {
		panic!("OpenGL function pointer of `glFogCoordf` is NULL");
	}
	fn glFogCoordfv(&self, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glFogCoordfv` is NULL");
	}
	fn glFogCoordd(&self, _: GLdouble) {
		panic!("OpenGL function pointer of `glFogCoordd` is NULL");
	}
	fn glFogCoorddv(&self, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glFogCoorddv` is NULL");
	}
	fn glFogCoordPointer(&self, _: GLenum, _: GLsizei, _: *const c_void) {
		panic!("OpenGL function pointer of `glFogCoordPointer` is NULL");
	}
	fn glSecondaryColor3b(&self, _: GLbyte, _: GLbyte, _: GLbyte) {
		panic!("OpenGL function pointer of `glSecondaryColor3b` is NULL");
	}
	fn glSecondaryColor3bv(&self, _: *const GLbyte) {
		panic!("OpenGL function pointer of `glSecondaryColor3bv` is NULL");
	}
	fn glSecondaryColor3d(&self, _: GLdouble, _: GLdouble, _: GLdouble) {
		panic!("OpenGL function pointer of `glSecondaryColor3d` is NULL");
	}
	fn glSecondaryColor3dv(&self, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glSecondaryColor3dv` is NULL");
	}
	fn glSecondaryColor3f(&self, _: GLfloat, _: GLfloat, _: GLfloat) {
		panic!("OpenGL function pointer of `glSecondaryColor3f` is NULL");
	}
	fn glSecondaryColor3fv(&self, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glSecondaryColor3fv` is NULL");
	}
	fn glSecondaryColor3i(&self, _: GLint, _: GLint, _: GLint) {
		panic!("OpenGL function pointer of `glSecondaryColor3i` is NULL");
	}
	fn glSecondaryColor3iv(&self, _: *const GLint) {
		panic!("OpenGL function pointer of `glSecondaryColor3iv` is NULL");
	}
	fn glSecondaryColor3s(&self, _: GLshort, _: GLshort, _: GLshort) {
		panic!("OpenGL function pointer of `glSecondaryColor3s` is NULL");
	}
	fn glSecondaryColor3sv(&self, _: *const GLshort) {
		panic!("OpenGL function pointer of `glSecondaryColor3sv` is NULL");
	}
	fn glSecondaryColor3ub(&self, _: GLubyte, _: GLubyte, _: GLubyte) {
		panic!("OpenGL function pointer of `glSecondaryColor3ub` is NULL");
	}
	fn glSecondaryColor3ubv(&self, _: *const GLubyte) {
		panic!("OpenGL function pointer of `glSecondaryColor3ubv` is NULL");
	}
	fn glSecondaryColor3ui(&self, _: GLuint, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glSecondaryColor3ui` is NULL");
	}
	fn glSecondaryColor3uiv(&self, _: *const GLuint) {
		panic!("OpenGL function pointer of `glSecondaryColor3uiv` is NULL");
	}
	fn glSecondaryColor3us(&self, _: GLushort, _: GLushort, _: GLushort) {
		panic!("OpenGL function pointer of `glSecondaryColor3us` is NULL");
	}
	fn glSecondaryColor3usv(&self, _: *const GLushort) {
		panic!("OpenGL function pointer of `glSecondaryColor3usv` is NULL");
	}
	fn glSecondaryColorPointer(&self, _: GLint, _: GLenum, _: GLsizei, _: *const c_void) {
		panic!("OpenGL function pointer of `glSecondaryColorPointer` is NULL");
	}
	fn glWindowPos2d(&self, _: GLdouble, _: GLdouble) {
		panic!("OpenGL function pointer of `glWindowPos2d` is NULL");
	}
	fn glWindowPos2dv(&self, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glWindowPos2dv` is NULL");
	}
	fn glWindowPos2f(&self, _: GLfloat, _: GLfloat) {
		panic!("OpenGL function pointer of `glWindowPos2f` is NULL");
	}
	fn glWindowPos2fv(&self, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glWindowPos2fv` is NULL");
	}
	fn glWindowPos2i(&self, _: GLint, _: GLint) {
		panic!("OpenGL function pointer of `glWindowPos2i` is NULL");
	}
	fn glWindowPos2iv(&self, _: *const GLint) {
		panic!("OpenGL function pointer of `glWindowPos2iv` is NULL");
	}
	fn glWindowPos2s(&self, _: GLshort, _: GLshort) {
		panic!("OpenGL function pointer of `glWindowPos2s` is NULL");
	}
	fn glWindowPos2sv(&self, _: *const GLshort) {
		panic!("OpenGL function pointer of `glWindowPos2sv` is NULL");
	}
	fn glWindowPos3d(&self, _: GLdouble, _: GLdouble, _: GLdouble) {
		panic!("OpenGL function pointer of `glWindowPos3d` is NULL");
	}
	fn glWindowPos3dv(&self, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glWindowPos3dv` is NULL");
	}
	fn glWindowPos3f(&self, _: GLfloat, _: GLfloat, _: GLfloat) {
		panic!("OpenGL function pointer of `glWindowPos3f` is NULL");
	}
	fn glWindowPos3fv(&self, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glWindowPos3fv` is NULL");
	}
	fn glWindowPos3i(&self, _: GLint, _: GLint, _: GLint) {
		panic!("OpenGL function pointer of `glWindowPos3i` is NULL");
	}
	fn glWindowPos3iv(&self, _: *const GLint) {
		panic!("OpenGL function pointer of `glWindowPos3iv` is NULL");
	}
	fn glWindowPos3s(&self, _: GLshort, _: GLshort, _: GLshort) {
		panic!("OpenGL function pointer of `glWindowPos3s` is NULL");
	}
	fn glWindowPos3sv(&self, _: *const GLshort) {
		panic!("OpenGL function pointer of `glWindowPos3sv` is NULL");
	}
	fn glBlendColor(&self, _: GLfloat, _: GLfloat, _: GLfloat, _: GLfloat) {
		panic!("OpenGL function pointer of `glBlendColor` is NULL");
	}
	fn glBlendEquation(&self, _: GLenum) {
		panic!("OpenGL function pointer of `glBlendEquation` is NULL");
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Version14 {
	available: bool,
	blendfuncseparate: PFNGLBLENDFUNCSEPARATEPROC,
	multidrawarrays: PFNGLMULTIDRAWARRAYSPROC,
	multidrawelements: PFNGLMULTIDRAWELEMENTSPROC,
	pointparameterf: PFNGLPOINTPARAMETERFPROC,
	pointparameterfv: PFNGLPOINTPARAMETERFVPROC,
	pointparameteri: PFNGLPOINTPARAMETERIPROC,
	pointparameteriv: PFNGLPOINTPARAMETERIVPROC,
	fogcoordf: PFNGLFOGCOORDFPROC,
	fogcoordfv: PFNGLFOGCOORDFVPROC,
	fogcoordd: PFNGLFOGCOORDDPROC,
	fogcoorddv: PFNGLFOGCOORDDVPROC,
	fogcoordpointer: PFNGLFOGCOORDPOINTERPROC,
	secondarycolor3b: PFNGLSECONDARYCOLOR3BPROC,
	secondarycolor3bv: PFNGLSECONDARYCOLOR3BVPROC,
	secondarycolor3d: PFNGLSECONDARYCOLOR3DPROC,
	secondarycolor3dv: PFNGLSECONDARYCOLOR3DVPROC,
	secondarycolor3f: PFNGLSECONDARYCOLOR3FPROC,
	secondarycolor3fv: PFNGLSECONDARYCOLOR3FVPROC,
	secondarycolor3i: PFNGLSECONDARYCOLOR3IPROC,
	secondarycolor3iv: PFNGLSECONDARYCOLOR3IVPROC,
	secondarycolor3s: PFNGLSECONDARYCOLOR3SPROC,
	secondarycolor3sv: PFNGLSECONDARYCOLOR3SVPROC,
	secondarycolor3ub: PFNGLSECONDARYCOLOR3UBPROC,
	secondarycolor3ubv: PFNGLSECONDARYCOLOR3UBVPROC,
	secondarycolor3ui: PFNGLSECONDARYCOLOR3UIPROC,
	secondarycolor3uiv: PFNGLSECONDARYCOLOR3UIVPROC,
	secondarycolor3us: PFNGLSECONDARYCOLOR3USPROC,
	secondarycolor3usv: PFNGLSECONDARYCOLOR3USVPROC,
	secondarycolorpointer: PFNGLSECONDARYCOLORPOINTERPROC,
	windowpos2d: PFNGLWINDOWPOS2DPROC,
	windowpos2dv: PFNGLWINDOWPOS2DVPROC,
	windowpos2f: PFNGLWINDOWPOS2FPROC,
	windowpos2fv: PFNGLWINDOWPOS2FVPROC,
	windowpos2i: PFNGLWINDOWPOS2IPROC,
	windowpos2iv: PFNGLWINDOWPOS2IVPROC,
	windowpos2s: PFNGLWINDOWPOS2SPROC,
	windowpos2sv: PFNGLWINDOWPOS2SVPROC,
	windowpos3d: PFNGLWINDOWPOS3DPROC,
	windowpos3dv: PFNGLWINDOWPOS3DVPROC,
	windowpos3f: PFNGLWINDOWPOS3FPROC,
	windowpos3fv: PFNGLWINDOWPOS3FVPROC,
	windowpos3i: PFNGLWINDOWPOS3IPROC,
	windowpos3iv: PFNGLWINDOWPOS3IVPROC,
	windowpos3s: PFNGLWINDOWPOS3SPROC,
	windowpos3sv: PFNGLWINDOWPOS3SVPROC,
	blendcolor: PFNGLBLENDCOLORPROC,
	blendequation: PFNGLBLENDEQUATIONPROC,
}

impl GL_1_4 for Version14 {
	#[inline(always)]
	fn glBlendFuncSeparate(&self, sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum) {
		(self.blendfuncseparate)(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha)
	}
	#[inline(always)]
	fn glMultiDrawArrays(&self, mode: GLenum, first: *const GLint, count: *const GLsizei, drawcount: GLsizei) {
		(self.multidrawarrays)(mode, first, count, drawcount)
	}
	#[inline(always)]
	fn glMultiDrawElements(&self, mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const c_void, drawcount: GLsizei) {
		(self.multidrawelements)(mode, count, type_, indices, drawcount)
	}
	#[inline(always)]
	fn glPointParameterf(&self, pname: GLenum, param: GLfloat) {
		(self.pointparameterf)(pname, param)
	}
	#[inline(always)]
	fn glPointParameterfv(&self, pname: GLenum, params: *const GLfloat) {
		(self.pointparameterfv)(pname, params)
	}
	#[inline(always)]
	fn glPointParameteri(&self, pname: GLenum, param: GLint) {
		(self.pointparameteri)(pname, param)
	}
	#[inline(always)]
	fn glPointParameteriv(&self, pname: GLenum, params: *const GLint) {
		(self.pointparameteriv)(pname, params)
	}
	#[inline(always)]
	fn glFogCoordf(&self, coord: GLfloat) {
		(self.fogcoordf)(coord)
	}
	#[inline(always)]
	fn glFogCoordfv(&self, coord: *const GLfloat) {
		(self.fogcoordfv)(coord)
	}
	#[inline(always)]
	fn glFogCoordd(&self, coord: GLdouble) {
		(self.fogcoordd)(coord)
	}
	#[inline(always)]
	fn glFogCoorddv(&self, coord: *const GLdouble) {
		(self.fogcoorddv)(coord)
	}
	#[inline(always)]
	fn glFogCoordPointer(&self, type_: GLenum, stride: GLsizei, pointer: *const c_void) {
		(self.fogcoordpointer)(type_, stride, pointer)
	}
	#[inline(always)]
	fn glSecondaryColor3b(&self, red: GLbyte, green: GLbyte, blue: GLbyte) {
		(self.secondarycolor3b)(red, green, blue)
	}
	#[inline(always)]
	fn glSecondaryColor3bv(&self, v: *const GLbyte) {
		(self.secondarycolor3bv)(v)
	}
	#[inline(always)]
	fn glSecondaryColor3d(&self, red: GLdouble, green: GLdouble, blue: GLdouble) {
		(self.secondarycolor3d)(red, green, blue)
	}
	#[inline(always)]
	fn glSecondaryColor3dv(&self, v: *const GLdouble) {
		(self.secondarycolor3dv)(v)
	}
	#[inline(always)]
	fn glSecondaryColor3f(&self, red: GLfloat, green: GLfloat, blue: GLfloat) {
		(self.secondarycolor3f)(red, green, blue)
	}
	#[inline(always)]
	fn glSecondaryColor3fv(&self, v: *const GLfloat) {
		(self.secondarycolor3fv)(v)
	}
	#[inline(always)]
	fn glSecondaryColor3i(&self, red: GLint, green: GLint, blue: GLint) {
		(self.secondarycolor3i)(red, green, blue)
	}
	#[inline(always)]
	fn glSecondaryColor3iv(&self, v: *const GLint) {
		(self.secondarycolor3iv)(v)
	}
	#[inline(always)]
	fn glSecondaryColor3s(&self, red: GLshort, green: GLshort, blue: GLshort) {
		(self.secondarycolor3s)(red, green, blue)
	}
	#[inline(always)]
	fn glSecondaryColor3sv(&self, v: *const GLshort) {
		(self.secondarycolor3sv)(v)
	}
	#[inline(always)]
	fn glSecondaryColor3ub(&self, red: GLubyte, green: GLubyte, blue: GLubyte) {
		(self.secondarycolor3ub)(red, green, blue)
	}
	#[inline(always)]
	fn glSecondaryColor3ubv(&self, v: *const GLubyte) {
		(self.secondarycolor3ubv)(v)
	}
	#[inline(always)]
	fn glSecondaryColor3ui(&self, red: GLuint, green: GLuint, blue: GLuint) {
		(self.secondarycolor3ui)(red, green, blue)
	}
	#[inline(always)]
	fn glSecondaryColor3uiv(&self, v: *const GLuint) {
		(self.secondarycolor3uiv)(v)
	}
	#[inline(always)]
	fn glSecondaryColor3us(&self, red: GLushort, green: GLushort, blue: GLushort) {
		(self.secondarycolor3us)(red, green, blue)
	}
	#[inline(always)]
	fn glSecondaryColor3usv(&self, v: *const GLushort) {
		(self.secondarycolor3usv)(v)
	}
	#[inline(always)]
	fn glSecondaryColorPointer(&self, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void) {
		(self.secondarycolorpointer)(size, type_, stride, pointer)
	}
	#[inline(always)]
	fn glWindowPos2d(&self, x: GLdouble, y: GLdouble) {
		(self.windowpos2d)(x, y)
	}
	#[inline(always)]
	fn glWindowPos2dv(&self, v: *const GLdouble) {
		(self.windowpos2dv)(v)
	}
	#[inline(always)]
	fn glWindowPos2f(&self, x: GLfloat, y: GLfloat) {
		(self.windowpos2f)(x, y)
	}
	#[inline(always)]
	fn glWindowPos2fv(&self, v: *const GLfloat) {
		(self.windowpos2fv)(v)
	}
	#[inline(always)]
	fn glWindowPos2i(&self, x: GLint, y: GLint) {
		(self.windowpos2i)(x, y)
	}
	#[inline(always)]
	fn glWindowPos2iv(&self, v: *const GLint) {
		(self.windowpos2iv)(v)
	}
	#[inline(always)]
	fn glWindowPos2s(&self, x: GLshort, y: GLshort) {
		(self.windowpos2s)(x, y)
	}
	#[inline(always)]
	fn glWindowPos2sv(&self, v: *const GLshort) {
		(self.windowpos2sv)(v)
	}
	#[inline(always)]
	fn glWindowPos3d(&self, x: GLdouble, y: GLdouble, z: GLdouble) {
		(self.windowpos3d)(x, y, z)
	}
	#[inline(always)]
	fn glWindowPos3dv(&self, v: *const GLdouble) {
		(self.windowpos3dv)(v)
	}
	#[inline(always)]
	fn glWindowPos3f(&self, x: GLfloat, y: GLfloat, z: GLfloat) {
		(self.windowpos3f)(x, y, z)
	}
	#[inline(always)]
	fn glWindowPos3fv(&self, v: *const GLfloat) {
		(self.windowpos3fv)(v)
	}
	#[inline(always)]
	fn glWindowPos3i(&self, x: GLint, y: GLint, z: GLint) {
		(self.windowpos3i)(x, y, z)
	}
	#[inline(always)]
	fn glWindowPos3iv(&self, v: *const GLint) {
		(self.windowpos3iv)(v)
	}
	#[inline(always)]
	fn glWindowPos3s(&self, x: GLshort, y: GLshort, z: GLshort) {
		(self.windowpos3s)(x, y, z)
	}
	#[inline(always)]
	fn glWindowPos3sv(&self, v: *const GLshort) {
		(self.windowpos3sv)(v)
	}
	#[inline(always)]
	fn glBlendColor(&self, red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
		(self.blendcolor)(red, green, blue, alpha)
	}
	#[inline(always)]
	fn glBlendEquation(&self, mode: GLenum) {
		(self.blendequation)(mode)
	}
}

impl Version14 {
	pub fn new(base: impl GL_1_0, get_proc_address: &impl Fn(&'static str) -> *const c_void) -> Self {
		let (spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 10400 {
			return Self::default();
		}
		Self {
			available: true,
			blendfuncseparate: unsafe{transmute(get_proc_address("glBlendFuncSeparate"))},
			multidrawarrays: unsafe{transmute(get_proc_address("glMultiDrawArrays"))},
			multidrawelements: unsafe{transmute(get_proc_address("glMultiDrawElements"))},
			pointparameterf: unsafe{transmute(get_proc_address("glPointParameterf"))},
			pointparameterfv: unsafe{transmute(get_proc_address("glPointParameterfv"))},
			pointparameteri: unsafe{transmute(get_proc_address("glPointParameteri"))},
			pointparameteriv: unsafe{transmute(get_proc_address("glPointParameteriv"))},
			fogcoordf: unsafe{transmute(get_proc_address("glFogCoordf"))},
			fogcoordfv: unsafe{transmute(get_proc_address("glFogCoordfv"))},
			fogcoordd: unsafe{transmute(get_proc_address("glFogCoordd"))},
			fogcoorddv: unsafe{transmute(get_proc_address("glFogCoorddv"))},
			fogcoordpointer: unsafe{transmute(get_proc_address("glFogCoordPointer"))},
			secondarycolor3b: unsafe{transmute(get_proc_address("glSecondaryColor3b"))},
			secondarycolor3bv: unsafe{transmute(get_proc_address("glSecondaryColor3bv"))},
			secondarycolor3d: unsafe{transmute(get_proc_address("glSecondaryColor3d"))},
			secondarycolor3dv: unsafe{transmute(get_proc_address("glSecondaryColor3dv"))},
			secondarycolor3f: unsafe{transmute(get_proc_address("glSecondaryColor3f"))},
			secondarycolor3fv: unsafe{transmute(get_proc_address("glSecondaryColor3fv"))},
			secondarycolor3i: unsafe{transmute(get_proc_address("glSecondaryColor3i"))},
			secondarycolor3iv: unsafe{transmute(get_proc_address("glSecondaryColor3iv"))},
			secondarycolor3s: unsafe{transmute(get_proc_address("glSecondaryColor3s"))},
			secondarycolor3sv: unsafe{transmute(get_proc_address("glSecondaryColor3sv"))},
			secondarycolor3ub: unsafe{transmute(get_proc_address("glSecondaryColor3ub"))},
			secondarycolor3ubv: unsafe{transmute(get_proc_address("glSecondaryColor3ubv"))},
			secondarycolor3ui: unsafe{transmute(get_proc_address("glSecondaryColor3ui"))},
			secondarycolor3uiv: unsafe{transmute(get_proc_address("glSecondaryColor3uiv"))},
			secondarycolor3us: unsafe{transmute(get_proc_address("glSecondaryColor3us"))},
			secondarycolor3usv: unsafe{transmute(get_proc_address("glSecondaryColor3usv"))},
			secondarycolorpointer: unsafe{transmute(get_proc_address("glSecondaryColorPointer"))},
			windowpos2d: unsafe{transmute(get_proc_address("glWindowPos2d"))},
			windowpos2dv: unsafe{transmute(get_proc_address("glWindowPos2dv"))},
			windowpos2f: unsafe{transmute(get_proc_address("glWindowPos2f"))},
			windowpos2fv: unsafe{transmute(get_proc_address("glWindowPos2fv"))},
			windowpos2i: unsafe{transmute(get_proc_address("glWindowPos2i"))},
			windowpos2iv: unsafe{transmute(get_proc_address("glWindowPos2iv"))},
			windowpos2s: unsafe{transmute(get_proc_address("glWindowPos2s"))},
			windowpos2sv: unsafe{transmute(get_proc_address("glWindowPos2sv"))},
			windowpos3d: unsafe{transmute(get_proc_address("glWindowPos3d"))},
			windowpos3dv: unsafe{transmute(get_proc_address("glWindowPos3dv"))},
			windowpos3f: unsafe{transmute(get_proc_address("glWindowPos3f"))},
			windowpos3fv: unsafe{transmute(get_proc_address("glWindowPos3fv"))},
			windowpos3i: unsafe{transmute(get_proc_address("glWindowPos3i"))},
			windowpos3iv: unsafe{transmute(get_proc_address("glWindowPos3iv"))},
			windowpos3s: unsafe{transmute(get_proc_address("glWindowPos3s"))},
			windowpos3sv: unsafe{transmute(get_proc_address("glWindowPos3sv"))},
			blendcolor: unsafe{transmute(get_proc_address("glBlendColor"))},
			blendequation: unsafe{transmute(get_proc_address("glBlendEquation"))},
		}
	}
	#[inline(always)]
	pub fn get_available(&self) -> bool {
		self.available
	}
}

impl Default for Version14 {
	fn default() -> Self {
		Self {
			available: false,
			blendfuncseparate: null(),
			multidrawarrays: null(),
			multidrawelements: null(),
			pointparameterf: null(),
			pointparameterfv: null(),
			pointparameteri: null(),
			pointparameteriv: null(),
			fogcoordf: null(),
			fogcoordfv: null(),
			fogcoordd: null(),
			fogcoorddv: null(),
			fogcoordpointer: null(),
			secondarycolor3b: null(),
			secondarycolor3bv: null(),
			secondarycolor3d: null(),
			secondarycolor3dv: null(),
			secondarycolor3f: null(),
			secondarycolor3fv: null(),
			secondarycolor3i: null(),
			secondarycolor3iv: null(),
			secondarycolor3s: null(),
			secondarycolor3sv: null(),
			secondarycolor3ub: null(),
			secondarycolor3ubv: null(),
			secondarycolor3ui: null(),
			secondarycolor3uiv: null(),
			secondarycolor3us: null(),
			secondarycolor3usv: null(),
			secondarycolorpointer: null(),
			windowpos2d: null(),
			windowpos2dv: null(),
			windowpos2f: null(),
			windowpos2fv: null(),
			windowpos2i: null(),
			windowpos2iv: null(),
			windowpos2s: null(),
			windowpos2sv: null(),
			windowpos3d: null(),
			windowpos3dv: null(),
			windowpos3f: null(),
			windowpos3fv: null(),
			windowpos3i: null(),
			windowpos3iv: null(),
			windowpos3s: null(),
			windowpos3sv: null(),
			blendcolor: null(),
			blendequation: null(),
		}
	}
}

type GLsizeiptr = khronos_ssize_t;
type GLintptr = khronos_intptr_t;
type PFNGLGENQUERIESPROC = extern "system" fn(GLsizei, *mut GLuint);
type PFNGLDELETEQUERIESPROC = extern "system" fn(GLsizei, *const GLuint);
type PFNGLISQUERYPROC = extern "system" fn(GLuint) -> GLboolean;
type PFNGLBEGINQUERYPROC = extern "system" fn(GLenum, GLuint);
type PFNGLENDQUERYPROC = extern "system" fn(GLenum);
type PFNGLGETQUERYIVPROC = extern "system" fn(GLenum, GLenum, *mut GLint);
type PFNGLGETQUERYOBJECTIVPROC = extern "system" fn(GLuint, GLenum, *mut GLint);
type PFNGLGETQUERYOBJECTUIVPROC = extern "system" fn(GLuint, GLenum, *mut GLuint);
type PFNGLBINDBUFFERPROC = extern "system" fn(GLenum, GLuint);
type PFNGLDELETEBUFFERSPROC = extern "system" fn(GLsizei, *const GLuint);
type PFNGLGENBUFFERSPROC = extern "system" fn(GLsizei, *mut GLuint);
type PFNGLISBUFFERPROC = extern "system" fn(GLuint) -> GLboolean;
type PFNGLBUFFERDATAPROC = extern "system" fn(GLenum, GLsizeiptr, *const c_void, GLenum);
type PFNGLBUFFERSUBDATAPROC = extern "system" fn(GLenum, GLintptr, GLsizeiptr, *const c_void);
type PFNGLGETBUFFERSUBDATAPROC = extern "system" fn(GLenum, GLintptr, GLsizeiptr, *mut c_void);
type PFNGLMAPBUFFERPROC = extern "system" fn(GLenum, GLenum) -> *mut c_void;
type PFNGLUNMAPBUFFERPROC = extern "system" fn(GLenum) -> GLboolean;
type PFNGLGETBUFFERPARAMETERIVPROC = extern "system" fn(GLenum, GLenum, *mut GLint);
type PFNGLGETBUFFERPOINTERVPROC = extern "system" fn(GLenum, GLenum, *mut *mut c_void);
const BUFFER_SIZE: GLenum = 0x8764;
const BUFFER_USAGE: GLenum = 0x8765;
const QUERY_COUNTER_BITS: GLenum = 0x8864;
const CURRENT_QUERY: GLenum = 0x8865;
const QUERY_RESULT: GLenum = 0x8866;
const QUERY_RESULT_AVAILABLE: GLenum = 0x8867;
const ARRAY_BUFFER: GLenum = 0x8892;
const ELEMENT_ARRAY_BUFFER: GLenum = 0x8893;
const ARRAY_BUFFER_BINDING: GLenum = 0x8894;
const ELEMENT_ARRAY_BUFFER_BINDING: GLenum = 0x8895;
const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLenum = 0x889F;
const READ_ONLY: GLenum = 0x88B8;
const WRITE_ONLY: GLenum = 0x88B9;
const READ_WRITE: GLenum = 0x88BA;
const BUFFER_ACCESS: GLenum = 0x88BB;
const BUFFER_MAPPED: GLenum = 0x88BC;
const BUFFER_MAP_POINTER: GLenum = 0x88BD;
const STREAM_DRAW: GLenum = 0x88E0;
const STREAM_READ: GLenum = 0x88E1;
const STREAM_COPY: GLenum = 0x88E2;
const STATIC_DRAW: GLenum = 0x88E4;
const STATIC_READ: GLenum = 0x88E5;
const STATIC_COPY: GLenum = 0x88E6;
const DYNAMIC_DRAW: GLenum = 0x88E8;
const DYNAMIC_READ: GLenum = 0x88E9;
const DYNAMIC_COPY: GLenum = 0x88EA;
const SAMPLES_PASSED: GLenum = 0x8914;
const SRC1_ALPHA: GLenum = 0x8589;
const VERTEX_ARRAY_BUFFER_BINDING: GLenum = 0x8896;
const NORMAL_ARRAY_BUFFER_BINDING: GLenum = 0x8897;
const COLOR_ARRAY_BUFFER_BINDING: GLenum = 0x8898;
const INDEX_ARRAY_BUFFER_BINDING: GLenum = 0x8899;
const TEXTURE_COORD_ARRAY_BUFFER_BINDING: GLenum = 0x889A;
const EDGE_FLAG_ARRAY_BUFFER_BINDING: GLenum = 0x889B;
const SECONDARY_COLOR_ARRAY_BUFFER_BINDING: GLenum = 0x889C;
const FOG_COORDINATE_ARRAY_BUFFER_BINDING: GLenum = 0x889D;
const WEIGHT_ARRAY_BUFFER_BINDING: GLenum = 0x889E;
const FOG_COORD_SRC: GLenum = 0x8450;
const FOG_COORD: GLenum = 0x8451;
const CURRENT_FOG_COORD: GLenum = 0x8453;
const FOG_COORD_ARRAY_TYPE: GLenum = 0x8454;
const FOG_COORD_ARRAY_STRIDE: GLenum = 0x8455;
const FOG_COORD_ARRAY_POINTER: GLenum = 0x8456;
const FOG_COORD_ARRAY: GLenum = 0x8457;
const FOG_COORD_ARRAY_BUFFER_BINDING: GLenum = 0x889D;
const SRC0_RGB: GLenum = 0x8580;
const SRC1_RGB: GLenum = 0x8581;
const SRC2_RGB: GLenum = 0x8582;
const SRC0_ALPHA: GLenum = 0x8588;
const SRC2_ALPHA: GLenum = 0x858A;

pub trait GL_1_5 {
	fn glGenQueries(&self, _: GLsizei, _: *mut GLuint) {
		panic!("OpenGL function pointer of `glGenQueries` is NULL");
	}
	fn glDeleteQueries(&self, _: GLsizei, _: *const GLuint) {
		panic!("OpenGL function pointer of `glDeleteQueries` is NULL");
	}
	fn glIsQuery(&self, _: GLuint) -> GLboolean {
		panic!("OpenGL function pointer of `glIsQuery` is NULL");
	}
	fn glBeginQuery(&self, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glBeginQuery` is NULL");
	}
	fn glEndQuery(&self, _: GLenum) {
		panic!("OpenGL function pointer of `glEndQuery` is NULL");
	}
	fn glGetQueryiv(&self, _: GLenum, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetQueryiv` is NULL");
	}
	fn glGetQueryObjectiv(&self, _: GLuint, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetQueryObjectiv` is NULL");
	}
	fn glGetQueryObjectuiv(&self, _: GLuint, _: GLenum, _: *mut GLuint) {
		panic!("OpenGL function pointer of `glGetQueryObjectuiv` is NULL");
	}
	fn glBindBuffer(&self, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glBindBuffer` is NULL");
	}
	fn glDeleteBuffers(&self, _: GLsizei, _: *const GLuint) {
		panic!("OpenGL function pointer of `glDeleteBuffers` is NULL");
	}
	fn glGenBuffers(&self, _: GLsizei, _: *mut GLuint) {
		panic!("OpenGL function pointer of `glGenBuffers` is NULL");
	}
	fn glIsBuffer(&self, _: GLuint) -> GLboolean {
		panic!("OpenGL function pointer of `glIsBuffer` is NULL");
	}
	fn glBufferData(&self, _: GLenum, _: GLsizeiptr, _: *const c_void, _: GLenum) {
		panic!("OpenGL function pointer of `glBufferData` is NULL");
	}
	fn glBufferSubData(&self, _: GLenum, _: GLintptr, _: GLsizeiptr, _: *const c_void) {
		panic!("OpenGL function pointer of `glBufferSubData` is NULL");
	}
	fn glGetBufferSubData(&self, _: GLenum, _: GLintptr, _: GLsizeiptr, _: *mut c_void) {
		panic!("OpenGL function pointer of `glGetBufferSubData` is NULL");
	}
	fn glMapBuffer(&self, _: GLenum, _: GLenum) -> *mut c_void {
		panic!("OpenGL function pointer of `glMapBuffer` is NULL");
	}
	fn glUnmapBuffer(&self, _: GLenum) -> GLboolean {
		panic!("OpenGL function pointer of `glUnmapBuffer` is NULL");
	}
	fn glGetBufferParameteriv(&self, _: GLenum, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetBufferParameteriv` is NULL");
	}
	fn glGetBufferPointerv(&self, _: GLenum, _: GLenum, _: *mut *mut c_void) {
		panic!("OpenGL function pointer of `glGetBufferPointerv` is NULL");
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Version15 {
	available: bool,
	genqueries: PFNGLGENQUERIESPROC,
	deletequeries: PFNGLDELETEQUERIESPROC,
	isquery: PFNGLISQUERYPROC,
	beginquery: PFNGLBEGINQUERYPROC,
	endquery: PFNGLENDQUERYPROC,
	getqueryiv: PFNGLGETQUERYIVPROC,
	getqueryobjectiv: PFNGLGETQUERYOBJECTIVPROC,
	getqueryobjectuiv: PFNGLGETQUERYOBJECTUIVPROC,
	bindbuffer: PFNGLBINDBUFFERPROC,
	deletebuffers: PFNGLDELETEBUFFERSPROC,
	genbuffers: PFNGLGENBUFFERSPROC,
	isbuffer: PFNGLISBUFFERPROC,
	bufferdata: PFNGLBUFFERDATAPROC,
	buffersubdata: PFNGLBUFFERSUBDATAPROC,
	getbuffersubdata: PFNGLGETBUFFERSUBDATAPROC,
	mapbuffer: PFNGLMAPBUFFERPROC,
	unmapbuffer: PFNGLUNMAPBUFFERPROC,
	getbufferparameteriv: PFNGLGETBUFFERPARAMETERIVPROC,
	getbufferpointerv: PFNGLGETBUFFERPOINTERVPROC,
}

impl GL_1_5 for Version15 {
	#[inline(always)]
	fn glGenQueries(&self, n: GLsizei, ids: *mut GLuint) {
		(self.genqueries)(n, ids)
	}
	#[inline(always)]
	fn glDeleteQueries(&self, n: GLsizei, ids: *const GLuint) {
		(self.deletequeries)(n, ids)
	}
	#[inline(always)]
	fn glIsQuery(&self, id: GLuint) -> GLboolean {
		(self.isquery)(id)
	}
	#[inline(always)]
	fn glBeginQuery(&self, target: GLenum, id: GLuint) {
		(self.beginquery)(target, id)
	}
	#[inline(always)]
	fn glEndQuery(&self, target: GLenum) {
		(self.endquery)(target)
	}
	#[inline(always)]
	fn glGetQueryiv(&self, target: GLenum, pname: GLenum, params: *mut GLint) {
		(self.getqueryiv)(target, pname, params)
	}
	#[inline(always)]
	fn glGetQueryObjectiv(&self, id: GLuint, pname: GLenum, params: *mut GLint) {
		(self.getqueryobjectiv)(id, pname, params)
	}
	#[inline(always)]
	fn glGetQueryObjectuiv(&self, id: GLuint, pname: GLenum, params: *mut GLuint) {
		(self.getqueryobjectuiv)(id, pname, params)
	}
	#[inline(always)]
	fn glBindBuffer(&self, target: GLenum, buffer: GLuint) {
		(self.bindbuffer)(target, buffer)
	}
	#[inline(always)]
	fn glDeleteBuffers(&self, n: GLsizei, buffers: *const GLuint) {
		(self.deletebuffers)(n, buffers)
	}
	#[inline(always)]
	fn glGenBuffers(&self, n: GLsizei, buffers: *mut GLuint) {
		(self.genbuffers)(n, buffers)
	}
	#[inline(always)]
	fn glIsBuffer(&self, buffer: GLuint) -> GLboolean {
		(self.isbuffer)(buffer)
	}
	#[inline(always)]
	fn glBufferData(&self, target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum) {
		(self.bufferdata)(target, size, data, usage)
	}
	#[inline(always)]
	fn glBufferSubData(&self, target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const c_void) {
		(self.buffersubdata)(target, offset, size, data)
	}
	#[inline(always)]
	fn glGetBufferSubData(&self, target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut c_void) {
		(self.getbuffersubdata)(target, offset, size, data)
	}
	#[inline(always)]
	fn glMapBuffer(&self, target: GLenum, access: GLenum) -> *mut c_void {
		(self.mapbuffer)(target, access)
	}
	#[inline(always)]
	fn glUnmapBuffer(&self, target: GLenum) -> GLboolean {
		(self.unmapbuffer)(target)
	}
	#[inline(always)]
	fn glGetBufferParameteriv(&self, target: GLenum, pname: GLenum, params: *mut GLint) {
		(self.getbufferparameteriv)(target, pname, params)
	}
	#[inline(always)]
	fn glGetBufferPointerv(&self, target: GLenum, pname: GLenum, params: *mut *mut c_void) {
		(self.getbufferpointerv)(target, pname, params)
	}
}

impl Version15 {
	pub fn new(base: impl GL_1_0, get_proc_address: &impl Fn(&'static str) -> *const c_void) -> Self {
		let (spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 10500 {
			return Self::default();
		}
		Self {
			available: true,
			genqueries: unsafe{transmute(get_proc_address("glGenQueries"))},
			deletequeries: unsafe{transmute(get_proc_address("glDeleteQueries"))},
			isquery: unsafe{transmute(get_proc_address("glIsQuery"))},
			beginquery: unsafe{transmute(get_proc_address("glBeginQuery"))},
			endquery: unsafe{transmute(get_proc_address("glEndQuery"))},
			getqueryiv: unsafe{transmute(get_proc_address("glGetQueryiv"))},
			getqueryobjectiv: unsafe{transmute(get_proc_address("glGetQueryObjectiv"))},
			getqueryobjectuiv: unsafe{transmute(get_proc_address("glGetQueryObjectuiv"))},
			bindbuffer: unsafe{transmute(get_proc_address("glBindBuffer"))},
			deletebuffers: unsafe{transmute(get_proc_address("glDeleteBuffers"))},
			genbuffers: unsafe{transmute(get_proc_address("glGenBuffers"))},
			isbuffer: unsafe{transmute(get_proc_address("glIsBuffer"))},
			bufferdata: unsafe{transmute(get_proc_address("glBufferData"))},
			buffersubdata: unsafe{transmute(get_proc_address("glBufferSubData"))},
			getbuffersubdata: unsafe{transmute(get_proc_address("glGetBufferSubData"))},
			mapbuffer: unsafe{transmute(get_proc_address("glMapBuffer"))},
			unmapbuffer: unsafe{transmute(get_proc_address("glUnmapBuffer"))},
			getbufferparameteriv: unsafe{transmute(get_proc_address("glGetBufferParameteriv"))},
			getbufferpointerv: unsafe{transmute(get_proc_address("glGetBufferPointerv"))},
		}
	}
	#[inline(always)]
	pub fn get_available(&self) -> bool {
		self.available
	}
}

impl Default for Version15 {
	fn default() -> Self {
		Self {
			available: false,
			genqueries: null(),
			deletequeries: null(),
			isquery: null(),
			beginquery: null(),
			endquery: null(),
			getqueryiv: null(),
			getqueryobjectiv: null(),
			getqueryobjectuiv: null(),
			bindbuffer: null(),
			deletebuffers: null(),
			genbuffers: null(),
			isbuffer: null(),
			bufferdata: null(),
			buffersubdata: null(),
			getbuffersubdata: null(),
			mapbuffer: null(),
			unmapbuffer: null(),
			getbufferparameteriv: null(),
			getbufferpointerv: null(),
		}
	}
}

type GLchar = i8;
type PFNGLBLENDEQUATIONSEPARATEPROC = extern "system" fn(GLenum, GLenum);
type PFNGLDRAWBUFFERSPROC = extern "system" fn(GLsizei, *const GLenum);
type PFNGLSTENCILOPSEPARATEPROC = extern "system" fn(GLenum, GLenum, GLenum, GLenum);
type PFNGLSTENCILFUNCSEPARATEPROC = extern "system" fn(GLenum, GLenum, GLint, GLuint);
type PFNGLSTENCILMASKSEPARATEPROC = extern "system" fn(GLenum, GLuint);
type PFNGLATTACHSHADERPROC = extern "system" fn(GLuint, GLuint);
type PFNGLBINDATTRIBLOCATIONPROC = extern "system" fn(GLuint, GLuint, *const GLchar);
type PFNGLCOMPILESHADERPROC = extern "system" fn(GLuint);
type PFNGLCREATEPROGRAMPROC = extern "system" fn() -> GLuint;
type PFNGLCREATESHADERPROC = extern "system" fn(GLenum) -> GLuint;
type PFNGLDELETEPROGRAMPROC = extern "system" fn(GLuint);
type PFNGLDELETESHADERPROC = extern "system" fn(GLuint);
type PFNGLDETACHSHADERPROC = extern "system" fn(GLuint, GLuint);
type PFNGLDISABLEVERTEXATTRIBARRAYPROC = extern "system" fn(GLuint);
type PFNGLENABLEVERTEXATTRIBARRAYPROC = extern "system" fn(GLuint);
type PFNGLGETACTIVEATTRIBPROC = extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLint, *mut GLenum, *mut GLchar);
type PFNGLGETACTIVEUNIFORMPROC = extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLint, *mut GLenum, *mut GLchar);
type PFNGLGETATTACHEDSHADERSPROC = extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLuint);
type PFNGLGETATTRIBLOCATIONPROC = extern "system" fn(GLuint, *const GLchar) -> GLint;
type PFNGLGETPROGRAMIVPROC = extern "system" fn(GLuint, GLenum, *mut GLint);
type PFNGLGETPROGRAMINFOLOGPROC = extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar);
type PFNGLGETSHADERIVPROC = extern "system" fn(GLuint, GLenum, *mut GLint);
type PFNGLGETSHADERINFOLOGPROC = extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar);
type PFNGLGETSHADERSOURCEPROC = extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar);
type PFNGLGETUNIFORMLOCATIONPROC = extern "system" fn(GLuint, *const GLchar) -> GLint;
type PFNGLGETUNIFORMFVPROC = extern "system" fn(GLuint, GLint, *mut GLfloat);
type PFNGLGETUNIFORMIVPROC = extern "system" fn(GLuint, GLint, *mut GLint);
type PFNGLGETVERTEXATTRIBDVPROC = extern "system" fn(GLuint, GLenum, *mut GLdouble);
type PFNGLGETVERTEXATTRIBFVPROC = extern "system" fn(GLuint, GLenum, *mut GLfloat);
type PFNGLGETVERTEXATTRIBIVPROC = extern "system" fn(GLuint, GLenum, *mut GLint);
type PFNGLGETVERTEXATTRIBPOINTERVPROC = extern "system" fn(GLuint, GLenum, *mut *mut c_void);
type PFNGLISPROGRAMPROC = extern "system" fn(GLuint) -> GLboolean;
type PFNGLISSHADERPROC = extern "system" fn(GLuint) -> GLboolean;
type PFNGLLINKPROGRAMPROC = extern "system" fn(GLuint);
type PFNGLSHADERSOURCEPROC = extern "system" fn(GLuint, GLsizei, *const *const GLchar, *const GLint);
type PFNGLUSEPROGRAMPROC = extern "system" fn(GLuint);
type PFNGLUNIFORM1FPROC = extern "system" fn(GLint, GLfloat);
type PFNGLUNIFORM2FPROC = extern "system" fn(GLint, GLfloat, GLfloat);
type PFNGLUNIFORM3FPROC = extern "system" fn(GLint, GLfloat, GLfloat, GLfloat);
type PFNGLUNIFORM4FPROC = extern "system" fn(GLint, GLfloat, GLfloat, GLfloat, GLfloat);
type PFNGLUNIFORM1IPROC = extern "system" fn(GLint, GLint);
type PFNGLUNIFORM2IPROC = extern "system" fn(GLint, GLint, GLint);
type PFNGLUNIFORM3IPROC = extern "system" fn(GLint, GLint, GLint, GLint);
type PFNGLUNIFORM4IPROC = extern "system" fn(GLint, GLint, GLint, GLint, GLint);
type PFNGLUNIFORM1FVPROC = extern "system" fn(GLint, GLsizei, *const GLfloat);
type PFNGLUNIFORM2FVPROC = extern "system" fn(GLint, GLsizei, *const GLfloat);
type PFNGLUNIFORM3FVPROC = extern "system" fn(GLint, GLsizei, *const GLfloat);
type PFNGLUNIFORM4FVPROC = extern "system" fn(GLint, GLsizei, *const GLfloat);
type PFNGLUNIFORM1IVPROC = extern "system" fn(GLint, GLsizei, *const GLint);
type PFNGLUNIFORM2IVPROC = extern "system" fn(GLint, GLsizei, *const GLint);
type PFNGLUNIFORM3IVPROC = extern "system" fn(GLint, GLsizei, *const GLint);
type PFNGLUNIFORM4IVPROC = extern "system" fn(GLint, GLsizei, *const GLint);
type PFNGLUNIFORMMATRIX2FVPROC = extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat);
type PFNGLUNIFORMMATRIX3FVPROC = extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat);
type PFNGLUNIFORMMATRIX4FVPROC = extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat);
type PFNGLVALIDATEPROGRAMPROC = extern "system" fn(GLuint);
type PFNGLVERTEXATTRIB1DPROC = extern "system" fn(GLuint, GLdouble);
type PFNGLVERTEXATTRIB1DVPROC = extern "system" fn(GLuint, *const GLdouble);
type PFNGLVERTEXATTRIB1FPROC = extern "system" fn(GLuint, GLfloat);
type PFNGLVERTEXATTRIB1FVPROC = extern "system" fn(GLuint, *const GLfloat);
type PFNGLVERTEXATTRIB1SPROC = extern "system" fn(GLuint, GLshort);
type PFNGLVERTEXATTRIB1SVPROC = extern "system" fn(GLuint, *const GLshort);
type PFNGLVERTEXATTRIB2DPROC = extern "system" fn(GLuint, GLdouble, GLdouble);
type PFNGLVERTEXATTRIB2DVPROC = extern "system" fn(GLuint, *const GLdouble);
type PFNGLVERTEXATTRIB2FPROC = extern "system" fn(GLuint, GLfloat, GLfloat);
type PFNGLVERTEXATTRIB2FVPROC = extern "system" fn(GLuint, *const GLfloat);
type PFNGLVERTEXATTRIB2SPROC = extern "system" fn(GLuint, GLshort, GLshort);
type PFNGLVERTEXATTRIB2SVPROC = extern "system" fn(GLuint, *const GLshort);
type PFNGLVERTEXATTRIB3DPROC = extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble);
type PFNGLVERTEXATTRIB3DVPROC = extern "system" fn(GLuint, *const GLdouble);
type PFNGLVERTEXATTRIB3FPROC = extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat);
type PFNGLVERTEXATTRIB3FVPROC = extern "system" fn(GLuint, *const GLfloat);
type PFNGLVERTEXATTRIB3SPROC = extern "system" fn(GLuint, GLshort, GLshort, GLshort);
type PFNGLVERTEXATTRIB3SVPROC = extern "system" fn(GLuint, *const GLshort);
type PFNGLVERTEXATTRIB4NBVPROC = extern "system" fn(GLuint, *const GLbyte);
type PFNGLVERTEXATTRIB4NIVPROC = extern "system" fn(GLuint, *const GLint);
type PFNGLVERTEXATTRIB4NSVPROC = extern "system" fn(GLuint, *const GLshort);
type PFNGLVERTEXATTRIB4NUBPROC = extern "system" fn(GLuint, GLubyte, GLubyte, GLubyte, GLubyte);
type PFNGLVERTEXATTRIB4NUBVPROC = extern "system" fn(GLuint, *const GLubyte);
type PFNGLVERTEXATTRIB4NUIVPROC = extern "system" fn(GLuint, *const GLuint);
type PFNGLVERTEXATTRIB4NUSVPROC = extern "system" fn(GLuint, *const GLushort);
type PFNGLVERTEXATTRIB4BVPROC = extern "system" fn(GLuint, *const GLbyte);
type PFNGLVERTEXATTRIB4DPROC = extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble, GLdouble);
type PFNGLVERTEXATTRIB4DVPROC = extern "system" fn(GLuint, *const GLdouble);
type PFNGLVERTEXATTRIB4FPROC = extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat, GLfloat);
type PFNGLVERTEXATTRIB4FVPROC = extern "system" fn(GLuint, *const GLfloat);
type PFNGLVERTEXATTRIB4IVPROC = extern "system" fn(GLuint, *const GLint);
type PFNGLVERTEXATTRIB4SPROC = extern "system" fn(GLuint, GLshort, GLshort, GLshort, GLshort);
type PFNGLVERTEXATTRIB4SVPROC = extern "system" fn(GLuint, *const GLshort);
type PFNGLVERTEXATTRIB4UBVPROC = extern "system" fn(GLuint, *const GLubyte);
type PFNGLVERTEXATTRIB4UIVPROC = extern "system" fn(GLuint, *const GLuint);
type PFNGLVERTEXATTRIB4USVPROC = extern "system" fn(GLuint, *const GLushort);
type PFNGLVERTEXATTRIBPOINTERPROC = extern "system" fn(GLuint, GLint, GLenum, GLboolean, GLsizei, *const c_void);
const BLEND_EQUATION_RGB: GLenum = 0x8009;
const VERTEX_ATTRIB_ARRAY_ENABLED: GLenum = 0x8622;
const VERTEX_ATTRIB_ARRAY_SIZE: GLenum = 0x8623;
const VERTEX_ATTRIB_ARRAY_STRIDE: GLenum = 0x8624;
const VERTEX_ATTRIB_ARRAY_TYPE: GLenum = 0x8625;
const CURRENT_VERTEX_ATTRIB: GLenum = 0x8626;
const VERTEX_PROGRAM_POINT_SIZE: GLenum = 0x8642;
const VERTEX_ATTRIB_ARRAY_POINTER: GLenum = 0x8645;
const STENCIL_BACK_FUNC: GLenum = 0x8800;
const STENCIL_BACK_FAIL: GLenum = 0x8801;
const STENCIL_BACK_PASS_DEPTH_FAIL: GLenum = 0x8802;
const STENCIL_BACK_PASS_DEPTH_PASS: GLenum = 0x8803;
const MAX_DRAW_BUFFERS: GLenum = 0x8824;
const DRAW_BUFFER0: GLenum = 0x8825;
const DRAW_BUFFER1: GLenum = 0x8826;
const DRAW_BUFFER2: GLenum = 0x8827;
const DRAW_BUFFER3: GLenum = 0x8828;
const DRAW_BUFFER4: GLenum = 0x8829;
const DRAW_BUFFER5: GLenum = 0x882A;
const DRAW_BUFFER6: GLenum = 0x882B;
const DRAW_BUFFER7: GLenum = 0x882C;
const DRAW_BUFFER8: GLenum = 0x882D;
const DRAW_BUFFER9: GLenum = 0x882E;
const DRAW_BUFFER10: GLenum = 0x882F;
const DRAW_BUFFER11: GLenum = 0x8830;
const DRAW_BUFFER12: GLenum = 0x8831;
const DRAW_BUFFER13: GLenum = 0x8832;
const DRAW_BUFFER14: GLenum = 0x8833;
const DRAW_BUFFER15: GLenum = 0x8834;
const BLEND_EQUATION_ALPHA: GLenum = 0x883D;
const MAX_VERTEX_ATTRIBS: GLenum = 0x8869;
const VERTEX_ATTRIB_ARRAY_NORMALIZED: GLenum = 0x886A;
const MAX_TEXTURE_IMAGE_UNITS: GLenum = 0x8872;
const FRAGMENT_SHADER: GLenum = 0x8B30;
const VERTEX_SHADER: GLenum = 0x8B31;
const MAX_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8B49;
const MAX_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8B4A;
const MAX_VARYING_FLOATS: GLenum = 0x8B4B;
const MAX_VERTEX_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4C;
const MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4D;
const SHADER_TYPE: GLenum = 0x8B4F;
const FLOAT_VEC2: GLenum = 0x8B50;
const FLOAT_VEC3: GLenum = 0x8B51;
const FLOAT_VEC4: GLenum = 0x8B52;
const INT_VEC2: GLenum = 0x8B53;
const INT_VEC3: GLenum = 0x8B54;
const INT_VEC4: GLenum = 0x8B55;
const BOOL: GLenum = 0x8B56;
const BOOL_VEC2: GLenum = 0x8B57;
const BOOL_VEC3: GLenum = 0x8B58;
const BOOL_VEC4: GLenum = 0x8B59;
const FLOAT_MAT2: GLenum = 0x8B5A;
const FLOAT_MAT3: GLenum = 0x8B5B;
const FLOAT_MAT4: GLenum = 0x8B5C;
const SAMPLER_1D: GLenum = 0x8B5D;
const SAMPLER_2D: GLenum = 0x8B5E;
const SAMPLER_3D: GLenum = 0x8B5F;
const SAMPLER_CUBE: GLenum = 0x8B60;
const SAMPLER_1D_SHADOW: GLenum = 0x8B61;
const SAMPLER_2D_SHADOW: GLenum = 0x8B62;
const DELETE_STATUS: GLenum = 0x8B80;
const COMPILE_STATUS: GLenum = 0x8B81;
const LINK_STATUS: GLenum = 0x8B82;
const VALIDATE_STATUS: GLenum = 0x8B83;
const INFO_LOG_LENGTH: GLenum = 0x8B84;
const ATTACHED_SHADERS: GLenum = 0x8B85;
const ACTIVE_UNIFORMS: GLenum = 0x8B86;
const ACTIVE_UNIFORM_MAX_LENGTH: GLenum = 0x8B87;
const SHADER_SOURCE_LENGTH: GLenum = 0x8B88;
const ACTIVE_ATTRIBUTES: GLenum = 0x8B89;
const ACTIVE_ATTRIBUTE_MAX_LENGTH: GLenum = 0x8B8A;
const FRAGMENT_SHADER_DERIVATIVE_HINT: GLenum = 0x8B8B;
const SHADING_LANGUAGE_VERSION: GLenum = 0x8B8C;
const CURRENT_PROGRAM: GLenum = 0x8B8D;
const POINT_SPRITE_COORD_ORIGIN: GLenum = 0x8CA0;
const LOWER_LEFT: GLenum = 0x8CA1;
const UPPER_LEFT: GLenum = 0x8CA2;
const STENCIL_BACK_REF: GLenum = 0x8CA3;
const STENCIL_BACK_VALUE_MASK: GLenum = 0x8CA4;
const STENCIL_BACK_WRITEMASK: GLenum = 0x8CA5;
const VERTEX_PROGRAM_TWO_SIDE: GLenum = 0x8643;
const POINT_SPRITE: GLenum = 0x8861;
const COORD_REPLACE: GLenum = 0x8862;
const MAX_TEXTURE_COORDS: GLenum = 0x8871;

pub trait GL_2_0 {
	fn glBlendEquationSeparate(&self, _: GLenum, _: GLenum) {
		panic!("OpenGL function pointer of `glBlendEquationSeparate` is NULL");
	}
	fn glDrawBuffers(&self, _: GLsizei, _: *const GLenum) {
		panic!("OpenGL function pointer of `glDrawBuffers` is NULL");
	}
	fn glStencilOpSeparate(&self, _: GLenum, _: GLenum, _: GLenum, _: GLenum) {
		panic!("OpenGL function pointer of `glStencilOpSeparate` is NULL");
	}
	fn glStencilFuncSeparate(&self, _: GLenum, _: GLenum, _: GLint, _: GLuint) {
		panic!("OpenGL function pointer of `glStencilFuncSeparate` is NULL");
	}
	fn glStencilMaskSeparate(&self, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glStencilMaskSeparate` is NULL");
	}
	fn glAttachShader(&self, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glAttachShader` is NULL");
	}
	fn glBindAttribLocation(&self, _: GLuint, _: GLuint, _: *const GLchar) {
		panic!("OpenGL function pointer of `glBindAttribLocation` is NULL");
	}
	fn glCompileShader(&self, _: GLuint) {
		panic!("OpenGL function pointer of `glCompileShader` is NULL");
	}
	fn glCreateProgram(&self) -> GLuint {
		panic!("OpenGL function pointer of `glCreateProgram` is NULL");
	}
	fn glCreateShader(&self, _: GLenum) -> GLuint {
		panic!("OpenGL function pointer of `glCreateShader` is NULL");
	}
	fn glDeleteProgram(&self, _: GLuint) {
		panic!("OpenGL function pointer of `glDeleteProgram` is NULL");
	}
	fn glDeleteShader(&self, _: GLuint) {
		panic!("OpenGL function pointer of `glDeleteShader` is NULL");
	}
	fn glDetachShader(&self, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glDetachShader` is NULL");
	}
	fn glDisableVertexAttribArray(&self, _: GLuint) {
		panic!("OpenGL function pointer of `glDisableVertexAttribArray` is NULL");
	}
	fn glEnableVertexAttribArray(&self, _: GLuint) {
		panic!("OpenGL function pointer of `glEnableVertexAttribArray` is NULL");
	}
	fn glGetActiveAttrib(&self, _: GLuint, _: GLuint, _: GLsizei, _: *mut GLsizei, _: *mut GLint, _: *mut GLenum, _: *mut GLchar) {
		panic!("OpenGL function pointer of `glGetActiveAttrib` is NULL");
	}
	fn glGetActiveUniform(&self, _: GLuint, _: GLuint, _: GLsizei, _: *mut GLsizei, _: *mut GLint, _: *mut GLenum, _: *mut GLchar) {
		panic!("OpenGL function pointer of `glGetActiveUniform` is NULL");
	}
	fn glGetAttachedShaders(&self, _: GLuint, _: GLsizei, _: *mut GLsizei, _: *mut GLuint) {
		panic!("OpenGL function pointer of `glGetAttachedShaders` is NULL");
	}
	fn glGetAttribLocation(&self, _: GLuint, _: *const GLchar) -> GLint {
		panic!("OpenGL function pointer of `glGetAttribLocation` is NULL");
	}
	fn glGetProgramiv(&self, _: GLuint, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetProgramiv` is NULL");
	}
	fn glGetProgramInfoLog(&self, _: GLuint, _: GLsizei, _: *mut GLsizei, _: *mut GLchar) {
		panic!("OpenGL function pointer of `glGetProgramInfoLog` is NULL");
	}
	fn glGetShaderiv(&self, _: GLuint, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetShaderiv` is NULL");
	}
	fn glGetShaderInfoLog(&self, _: GLuint, _: GLsizei, _: *mut GLsizei, _: *mut GLchar) {
		panic!("OpenGL function pointer of `glGetShaderInfoLog` is NULL");
	}
	fn glGetShaderSource(&self, _: GLuint, _: GLsizei, _: *mut GLsizei, _: *mut GLchar) {
		panic!("OpenGL function pointer of `glGetShaderSource` is NULL");
	}
	fn glGetUniformLocation(&self, _: GLuint, _: *const GLchar) -> GLint {
		panic!("OpenGL function pointer of `glGetUniformLocation` is NULL");
	}
	fn glGetUniformfv(&self, _: GLuint, _: GLint, _: *mut GLfloat) {
		panic!("OpenGL function pointer of `glGetUniformfv` is NULL");
	}
	fn glGetUniformiv(&self, _: GLuint, _: GLint, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetUniformiv` is NULL");
	}
	fn glGetVertexAttribdv(&self, _: GLuint, _: GLenum, _: *mut GLdouble) {
		panic!("OpenGL function pointer of `glGetVertexAttribdv` is NULL");
	}
	fn glGetVertexAttribfv(&self, _: GLuint, _: GLenum, _: *mut GLfloat) {
		panic!("OpenGL function pointer of `glGetVertexAttribfv` is NULL");
	}
	fn glGetVertexAttribiv(&self, _: GLuint, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetVertexAttribiv` is NULL");
	}
	fn glGetVertexAttribPointerv(&self, _: GLuint, _: GLenum, _: *mut *mut c_void) {
		panic!("OpenGL function pointer of `glGetVertexAttribPointerv` is NULL");
	}
	fn glIsProgram(&self, _: GLuint) -> GLboolean {
		panic!("OpenGL function pointer of `glIsProgram` is NULL");
	}
	fn glIsShader(&self, _: GLuint) -> GLboolean {
		panic!("OpenGL function pointer of `glIsShader` is NULL");
	}
	fn glLinkProgram(&self, _: GLuint) {
		panic!("OpenGL function pointer of `glLinkProgram` is NULL");
	}
	fn glShaderSource(&self, _: GLuint, _: GLsizei, _: *const *const GLchar, _: *const GLint) {
		panic!("OpenGL function pointer of `glShaderSource` is NULL");
	}
	fn glUseProgram(&self, _: GLuint) {
		panic!("OpenGL function pointer of `glUseProgram` is NULL");
	}
	fn glUniform1f(&self, _: GLint, _: GLfloat) {
		panic!("OpenGL function pointer of `glUniform1f` is NULL");
	}
	fn glUniform2f(&self, _: GLint, _: GLfloat, _: GLfloat) {
		panic!("OpenGL function pointer of `glUniform2f` is NULL");
	}
	fn glUniform3f(&self, _: GLint, _: GLfloat, _: GLfloat, _: GLfloat) {
		panic!("OpenGL function pointer of `glUniform3f` is NULL");
	}
	fn glUniform4f(&self, _: GLint, _: GLfloat, _: GLfloat, _: GLfloat, _: GLfloat) {
		panic!("OpenGL function pointer of `glUniform4f` is NULL");
	}
	fn glUniform1i(&self, _: GLint, _: GLint) {
		panic!("OpenGL function pointer of `glUniform1i` is NULL");
	}
	fn glUniform2i(&self, _: GLint, _: GLint, _: GLint) {
		panic!("OpenGL function pointer of `glUniform2i` is NULL");
	}
	fn glUniform3i(&self, _: GLint, _: GLint, _: GLint, _: GLint) {
		panic!("OpenGL function pointer of `glUniform3i` is NULL");
	}
	fn glUniform4i(&self, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint) {
		panic!("OpenGL function pointer of `glUniform4i` is NULL");
	}
	fn glUniform1fv(&self, _: GLint, _: GLsizei, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glUniform1fv` is NULL");
	}
	fn glUniform2fv(&self, _: GLint, _: GLsizei, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glUniform2fv` is NULL");
	}
	fn glUniform3fv(&self, _: GLint, _: GLsizei, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glUniform3fv` is NULL");
	}
	fn glUniform4fv(&self, _: GLint, _: GLsizei, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glUniform4fv` is NULL");
	}
	fn glUniform1iv(&self, _: GLint, _: GLsizei, _: *const GLint) {
		panic!("OpenGL function pointer of `glUniform1iv` is NULL");
	}
	fn glUniform2iv(&self, _: GLint, _: GLsizei, _: *const GLint) {
		panic!("OpenGL function pointer of `glUniform2iv` is NULL");
	}
	fn glUniform3iv(&self, _: GLint, _: GLsizei, _: *const GLint) {
		panic!("OpenGL function pointer of `glUniform3iv` is NULL");
	}
	fn glUniform4iv(&self, _: GLint, _: GLsizei, _: *const GLint) {
		panic!("OpenGL function pointer of `glUniform4iv` is NULL");
	}
	fn glUniformMatrix2fv(&self, _: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glUniformMatrix2fv` is NULL");
	}
	fn glUniformMatrix3fv(&self, _: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glUniformMatrix3fv` is NULL");
	}
	fn glUniformMatrix4fv(&self, _: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glUniformMatrix4fv` is NULL");
	}
	fn glValidateProgram(&self, _: GLuint) {
		panic!("OpenGL function pointer of `glValidateProgram` is NULL");
	}
	fn glVertexAttrib1d(&self, _: GLuint, _: GLdouble) {
		panic!("OpenGL function pointer of `glVertexAttrib1d` is NULL");
	}
	fn glVertexAttrib1dv(&self, _: GLuint, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glVertexAttrib1dv` is NULL");
	}
	fn glVertexAttrib1f(&self, _: GLuint, _: GLfloat) {
		panic!("OpenGL function pointer of `glVertexAttrib1f` is NULL");
	}
	fn glVertexAttrib1fv(&self, _: GLuint, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glVertexAttrib1fv` is NULL");
	}
	fn glVertexAttrib1s(&self, _: GLuint, _: GLshort) {
		panic!("OpenGL function pointer of `glVertexAttrib1s` is NULL");
	}
	fn glVertexAttrib1sv(&self, _: GLuint, _: *const GLshort) {
		panic!("OpenGL function pointer of `glVertexAttrib1sv` is NULL");
	}
	fn glVertexAttrib2d(&self, _: GLuint, _: GLdouble, _: GLdouble) {
		panic!("OpenGL function pointer of `glVertexAttrib2d` is NULL");
	}
	fn glVertexAttrib2dv(&self, _: GLuint, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glVertexAttrib2dv` is NULL");
	}
	fn glVertexAttrib2f(&self, _: GLuint, _: GLfloat, _: GLfloat) {
		panic!("OpenGL function pointer of `glVertexAttrib2f` is NULL");
	}
	fn glVertexAttrib2fv(&self, _: GLuint, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glVertexAttrib2fv` is NULL");
	}
	fn glVertexAttrib2s(&self, _: GLuint, _: GLshort, _: GLshort) {
		panic!("OpenGL function pointer of `glVertexAttrib2s` is NULL");
	}
	fn glVertexAttrib2sv(&self, _: GLuint, _: *const GLshort) {
		panic!("OpenGL function pointer of `glVertexAttrib2sv` is NULL");
	}
	fn glVertexAttrib3d(&self, _: GLuint, _: GLdouble, _: GLdouble, _: GLdouble) {
		panic!("OpenGL function pointer of `glVertexAttrib3d` is NULL");
	}
	fn glVertexAttrib3dv(&self, _: GLuint, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glVertexAttrib3dv` is NULL");
	}
	fn glVertexAttrib3f(&self, _: GLuint, _: GLfloat, _: GLfloat, _: GLfloat) {
		panic!("OpenGL function pointer of `glVertexAttrib3f` is NULL");
	}
	fn glVertexAttrib3fv(&self, _: GLuint, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glVertexAttrib3fv` is NULL");
	}
	fn glVertexAttrib3s(&self, _: GLuint, _: GLshort, _: GLshort, _: GLshort) {
		panic!("OpenGL function pointer of `glVertexAttrib3s` is NULL");
	}
	fn glVertexAttrib3sv(&self, _: GLuint, _: *const GLshort) {
		panic!("OpenGL function pointer of `glVertexAttrib3sv` is NULL");
	}
	fn glVertexAttrib4Nbv(&self, _: GLuint, _: *const GLbyte) {
		panic!("OpenGL function pointer of `glVertexAttrib4Nbv` is NULL");
	}
	fn glVertexAttrib4Niv(&self, _: GLuint, _: *const GLint) {
		panic!("OpenGL function pointer of `glVertexAttrib4Niv` is NULL");
	}
	fn glVertexAttrib4Nsv(&self, _: GLuint, _: *const GLshort) {
		panic!("OpenGL function pointer of `glVertexAttrib4Nsv` is NULL");
	}
	fn glVertexAttrib4Nub(&self, _: GLuint, _: GLubyte, _: GLubyte, _: GLubyte, _: GLubyte) {
		panic!("OpenGL function pointer of `glVertexAttrib4Nub` is NULL");
	}
	fn glVertexAttrib4Nubv(&self, _: GLuint, _: *const GLubyte) {
		panic!("OpenGL function pointer of `glVertexAttrib4Nubv` is NULL");
	}
	fn glVertexAttrib4Nuiv(&self, _: GLuint, _: *const GLuint) {
		panic!("OpenGL function pointer of `glVertexAttrib4Nuiv` is NULL");
	}
	fn glVertexAttrib4Nusv(&self, _: GLuint, _: *const GLushort) {
		panic!("OpenGL function pointer of `glVertexAttrib4Nusv` is NULL");
	}
	fn glVertexAttrib4bv(&self, _: GLuint, _: *const GLbyte) {
		panic!("OpenGL function pointer of `glVertexAttrib4bv` is NULL");
	}
	fn glVertexAttrib4d(&self, _: GLuint, _: GLdouble, _: GLdouble, _: GLdouble, _: GLdouble) {
		panic!("OpenGL function pointer of `glVertexAttrib4d` is NULL");
	}
	fn glVertexAttrib4dv(&self, _: GLuint, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glVertexAttrib4dv` is NULL");
	}
	fn glVertexAttrib4f(&self, _: GLuint, _: GLfloat, _: GLfloat, _: GLfloat, _: GLfloat) {
		panic!("OpenGL function pointer of `glVertexAttrib4f` is NULL");
	}
	fn glVertexAttrib4fv(&self, _: GLuint, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glVertexAttrib4fv` is NULL");
	}
	fn glVertexAttrib4iv(&self, _: GLuint, _: *const GLint) {
		panic!("OpenGL function pointer of `glVertexAttrib4iv` is NULL");
	}
	fn glVertexAttrib4s(&self, _: GLuint, _: GLshort, _: GLshort, _: GLshort, _: GLshort) {
		panic!("OpenGL function pointer of `glVertexAttrib4s` is NULL");
	}
	fn glVertexAttrib4sv(&self, _: GLuint, _: *const GLshort) {
		panic!("OpenGL function pointer of `glVertexAttrib4sv` is NULL");
	}
	fn glVertexAttrib4ubv(&self, _: GLuint, _: *const GLubyte) {
		panic!("OpenGL function pointer of `glVertexAttrib4ubv` is NULL");
	}
	fn glVertexAttrib4uiv(&self, _: GLuint, _: *const GLuint) {
		panic!("OpenGL function pointer of `glVertexAttrib4uiv` is NULL");
	}
	fn glVertexAttrib4usv(&self, _: GLuint, _: *const GLushort) {
		panic!("OpenGL function pointer of `glVertexAttrib4usv` is NULL");
	}
	fn glVertexAttribPointer(&self, _: GLuint, _: GLint, _: GLenum, _: GLboolean, _: GLsizei, _: *const c_void) {
		panic!("OpenGL function pointer of `glVertexAttribPointer` is NULL");
	}
	fn get_shading_language_version(&self) -> &'static str;
}

#[derive(Debug, Clone, Copy)]
pub struct Version20 {
	shading_language_version: &'static str,
	available: bool,
	blendequationseparate: PFNGLBLENDEQUATIONSEPARATEPROC,
	drawbuffers: PFNGLDRAWBUFFERSPROC,
	stencilopseparate: PFNGLSTENCILOPSEPARATEPROC,
	stencilfuncseparate: PFNGLSTENCILFUNCSEPARATEPROC,
	stencilmaskseparate: PFNGLSTENCILMASKSEPARATEPROC,
	attachshader: PFNGLATTACHSHADERPROC,
	bindattriblocation: PFNGLBINDATTRIBLOCATIONPROC,
	compileshader: PFNGLCOMPILESHADERPROC,
	createprogram: PFNGLCREATEPROGRAMPROC,
	createshader: PFNGLCREATESHADERPROC,
	deleteprogram: PFNGLDELETEPROGRAMPROC,
	deleteshader: PFNGLDELETESHADERPROC,
	detachshader: PFNGLDETACHSHADERPROC,
	disablevertexattribarray: PFNGLDISABLEVERTEXATTRIBARRAYPROC,
	enablevertexattribarray: PFNGLENABLEVERTEXATTRIBARRAYPROC,
	getactiveattrib: PFNGLGETACTIVEATTRIBPROC,
	getactiveuniform: PFNGLGETACTIVEUNIFORMPROC,
	getattachedshaders: PFNGLGETATTACHEDSHADERSPROC,
	getattriblocation: PFNGLGETATTRIBLOCATIONPROC,
	getprogramiv: PFNGLGETPROGRAMIVPROC,
	getprograminfolog: PFNGLGETPROGRAMINFOLOGPROC,
	getshaderiv: PFNGLGETSHADERIVPROC,
	getshaderinfolog: PFNGLGETSHADERINFOLOGPROC,
	getshadersource: PFNGLGETSHADERSOURCEPROC,
	getuniformlocation: PFNGLGETUNIFORMLOCATIONPROC,
	getuniformfv: PFNGLGETUNIFORMFVPROC,
	getuniformiv: PFNGLGETUNIFORMIVPROC,
	getvertexattribdv: PFNGLGETVERTEXATTRIBDVPROC,
	getvertexattribfv: PFNGLGETVERTEXATTRIBFVPROC,
	getvertexattribiv: PFNGLGETVERTEXATTRIBIVPROC,
	getvertexattribpointerv: PFNGLGETVERTEXATTRIBPOINTERVPROC,
	isprogram: PFNGLISPROGRAMPROC,
	isshader: PFNGLISSHADERPROC,
	linkprogram: PFNGLLINKPROGRAMPROC,
	shadersource: PFNGLSHADERSOURCEPROC,
	useprogram: PFNGLUSEPROGRAMPROC,
	uniform1f: PFNGLUNIFORM1FPROC,
	uniform2f: PFNGLUNIFORM2FPROC,
	uniform3f: PFNGLUNIFORM3FPROC,
	uniform4f: PFNGLUNIFORM4FPROC,
	uniform1i: PFNGLUNIFORM1IPROC,
	uniform2i: PFNGLUNIFORM2IPROC,
	uniform3i: PFNGLUNIFORM3IPROC,
	uniform4i: PFNGLUNIFORM4IPROC,
	uniform1fv: PFNGLUNIFORM1FVPROC,
	uniform2fv: PFNGLUNIFORM2FVPROC,
	uniform3fv: PFNGLUNIFORM3FVPROC,
	uniform4fv: PFNGLUNIFORM4FVPROC,
	uniform1iv: PFNGLUNIFORM1IVPROC,
	uniform2iv: PFNGLUNIFORM2IVPROC,
	uniform3iv: PFNGLUNIFORM3IVPROC,
	uniform4iv: PFNGLUNIFORM4IVPROC,
	uniformmatrix2fv: PFNGLUNIFORMMATRIX2FVPROC,
	uniformmatrix3fv: PFNGLUNIFORMMATRIX3FVPROC,
	uniformmatrix4fv: PFNGLUNIFORMMATRIX4FVPROC,
	validateprogram: PFNGLVALIDATEPROGRAMPROC,
	vertexattrib1d: PFNGLVERTEXATTRIB1DPROC,
	vertexattrib1dv: PFNGLVERTEXATTRIB1DVPROC,
	vertexattrib1f: PFNGLVERTEXATTRIB1FPROC,
	vertexattrib1fv: PFNGLVERTEXATTRIB1FVPROC,
	vertexattrib1s: PFNGLVERTEXATTRIB1SPROC,
	vertexattrib1sv: PFNGLVERTEXATTRIB1SVPROC,
	vertexattrib2d: PFNGLVERTEXATTRIB2DPROC,
	vertexattrib2dv: PFNGLVERTEXATTRIB2DVPROC,
	vertexattrib2f: PFNGLVERTEXATTRIB2FPROC,
	vertexattrib2fv: PFNGLVERTEXATTRIB2FVPROC,
	vertexattrib2s: PFNGLVERTEXATTRIB2SPROC,
	vertexattrib2sv: PFNGLVERTEXATTRIB2SVPROC,
	vertexattrib3d: PFNGLVERTEXATTRIB3DPROC,
	vertexattrib3dv: PFNGLVERTEXATTRIB3DVPROC,
	vertexattrib3f: PFNGLVERTEXATTRIB3FPROC,
	vertexattrib3fv: PFNGLVERTEXATTRIB3FVPROC,
	vertexattrib3s: PFNGLVERTEXATTRIB3SPROC,
	vertexattrib3sv: PFNGLVERTEXATTRIB3SVPROC,
	vertexattrib4nbv: PFNGLVERTEXATTRIB4NBVPROC,
	vertexattrib4niv: PFNGLVERTEXATTRIB4NIVPROC,
	vertexattrib4nsv: PFNGLVERTEXATTRIB4NSVPROC,
	vertexattrib4nub: PFNGLVERTEXATTRIB4NUBPROC,
	vertexattrib4nubv: PFNGLVERTEXATTRIB4NUBVPROC,
	vertexattrib4nuiv: PFNGLVERTEXATTRIB4NUIVPROC,
	vertexattrib4nusv: PFNGLVERTEXATTRIB4NUSVPROC,
	vertexattrib4bv: PFNGLVERTEXATTRIB4BVPROC,
	vertexattrib4d: PFNGLVERTEXATTRIB4DPROC,
	vertexattrib4dv: PFNGLVERTEXATTRIB4DVPROC,
	vertexattrib4f: PFNGLVERTEXATTRIB4FPROC,
	vertexattrib4fv: PFNGLVERTEXATTRIB4FVPROC,
	vertexattrib4iv: PFNGLVERTEXATTRIB4IVPROC,
	vertexattrib4s: PFNGLVERTEXATTRIB4SPROC,
	vertexattrib4sv: PFNGLVERTEXATTRIB4SVPROC,
	vertexattrib4ubv: PFNGLVERTEXATTRIB4UBVPROC,
	vertexattrib4uiv: PFNGLVERTEXATTRIB4UIVPROC,
	vertexattrib4usv: PFNGLVERTEXATTRIB4USVPROC,
	vertexattribpointer: PFNGLVERTEXATTRIBPOINTERPROC,
}

impl GL_2_0 for Version20 {
	#[inline(always)]
	fn glBlendEquationSeparate(&self, modeRGB: GLenum, modeAlpha: GLenum) {
		(self.blendequationseparate)(modeRGB, modeAlpha)
	}
	#[inline(always)]
	fn glDrawBuffers(&self, n: GLsizei, bufs: *const GLenum) {
		(self.drawbuffers)(n, bufs)
	}
	#[inline(always)]
	fn glStencilOpSeparate(&self, face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum) {
		(self.stencilopseparate)(face, sfail, dpfail, dppass)
	}
	#[inline(always)]
	fn glStencilFuncSeparate(&self, face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) {
		(self.stencilfuncseparate)(face, func, ref_, mask)
	}
	#[inline(always)]
	fn glStencilMaskSeparate(&self, face: GLenum, mask: GLuint) {
		(self.stencilmaskseparate)(face, mask)
	}
	#[inline(always)]
	fn glAttachShader(&self, program: GLuint, shader: GLuint) {
		(self.attachshader)(program, shader)
	}
	#[inline(always)]
	fn glBindAttribLocation(&self, program: GLuint, index: GLuint, name: *const GLchar) {
		(self.bindattriblocation)(program, index, name)
	}
	#[inline(always)]
	fn glCompileShader(&self, shader: GLuint) {
		(self.compileshader)(shader)
	}
	#[inline(always)]
	fn glCreateProgram(&self) -> GLuint {
		(self.createprogram)()
	}
	#[inline(always)]
	fn glCreateShader(&self, type_: GLenum) -> GLuint {
		(self.createshader)(type_)
	}
	#[inline(always)]
	fn glDeleteProgram(&self, program: GLuint) {
		(self.deleteprogram)(program)
	}
	#[inline(always)]
	fn glDeleteShader(&self, shader: GLuint) {
		(self.deleteshader)(shader)
	}
	#[inline(always)]
	fn glDetachShader(&self, program: GLuint, shader: GLuint) {
		(self.detachshader)(program, shader)
	}
	#[inline(always)]
	fn glDisableVertexAttribArray(&self, index: GLuint) {
		(self.disablevertexattribarray)(index)
	}
	#[inline(always)]
	fn glEnableVertexAttribArray(&self, index: GLuint) {
		(self.enablevertexattribarray)(index)
	}
	#[inline(always)]
	fn glGetActiveAttrib(&self, program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) {
		(self.getactiveattrib)(program, index, bufSize, length, size, type_, name)
	}
	#[inline(always)]
	fn glGetActiveUniform(&self, program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) {
		(self.getactiveuniform)(program, index, bufSize, length, size, type_, name)
	}
	#[inline(always)]
	fn glGetAttachedShaders(&self, program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint) {
		(self.getattachedshaders)(program, maxCount, count, shaders)
	}
	#[inline(always)]
	fn glGetAttribLocation(&self, program: GLuint, name: *const GLchar) -> GLint {
		(self.getattriblocation)(program, name)
	}
	#[inline(always)]
	fn glGetProgramiv(&self, program: GLuint, pname: GLenum, params: *mut GLint) {
		(self.getprogramiv)(program, pname, params)
	}
	#[inline(always)]
	fn glGetProgramInfoLog(&self, program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) {
		(self.getprograminfolog)(program, bufSize, length, infoLog)
	}
	#[inline(always)]
	fn glGetShaderiv(&self, shader: GLuint, pname: GLenum, params: *mut GLint) {
		(self.getshaderiv)(shader, pname, params)
	}
	#[inline(always)]
	fn glGetShaderInfoLog(&self, shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) {
		(self.getshaderinfolog)(shader, bufSize, length, infoLog)
	}
	#[inline(always)]
	fn glGetShaderSource(&self, shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar) {
		(self.getshadersource)(shader, bufSize, length, source)
	}
	#[inline(always)]
	fn glGetUniformLocation(&self, program: GLuint, name: *const GLchar) -> GLint {
		(self.getuniformlocation)(program, name)
	}
	#[inline(always)]
	fn glGetUniformfv(&self, program: GLuint, location: GLint, params: *mut GLfloat) {
		(self.getuniformfv)(program, location, params)
	}
	#[inline(always)]
	fn glGetUniformiv(&self, program: GLuint, location: GLint, params: *mut GLint) {
		(self.getuniformiv)(program, location, params)
	}
	#[inline(always)]
	fn glGetVertexAttribdv(&self, index: GLuint, pname: GLenum, params: *mut GLdouble) {
		(self.getvertexattribdv)(index, pname, params)
	}
	#[inline(always)]
	fn glGetVertexAttribfv(&self, index: GLuint, pname: GLenum, params: *mut GLfloat) {
		(self.getvertexattribfv)(index, pname, params)
	}
	#[inline(always)]
	fn glGetVertexAttribiv(&self, index: GLuint, pname: GLenum, params: *mut GLint) {
		(self.getvertexattribiv)(index, pname, params)
	}
	#[inline(always)]
	fn glGetVertexAttribPointerv(&self, index: GLuint, pname: GLenum, pointer: *mut *mut c_void) {
		(self.getvertexattribpointerv)(index, pname, pointer)
	}
	#[inline(always)]
	fn glIsProgram(&self, program: GLuint) -> GLboolean {
		(self.isprogram)(program)
	}
	#[inline(always)]
	fn glIsShader(&self, shader: GLuint) -> GLboolean {
		(self.isshader)(shader)
	}
	#[inline(always)]
	fn glLinkProgram(&self, program: GLuint) {
		(self.linkprogram)(program)
	}
	#[inline(always)]
	fn glShaderSource(&self, shader: GLuint, count: GLsizei, string_: *const *const GLchar, length: *const GLint) {
		(self.shadersource)(shader, count, string_, length)
	}
	#[inline(always)]
	fn glUseProgram(&self, program: GLuint) {
		(self.useprogram)(program)
	}
	#[inline(always)]
	fn glUniform1f(&self, location: GLint, v0: GLfloat) {
		(self.uniform1f)(location, v0)
	}
	#[inline(always)]
	fn glUniform2f(&self, location: GLint, v0: GLfloat, v1: GLfloat) {
		(self.uniform2f)(location, v0, v1)
	}
	#[inline(always)]
	fn glUniform3f(&self, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) {
		(self.uniform3f)(location, v0, v1, v2)
	}
	#[inline(always)]
	fn glUniform4f(&self, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) {
		(self.uniform4f)(location, v0, v1, v2, v3)
	}
	#[inline(always)]
	fn glUniform1i(&self, location: GLint, v0: GLint) {
		(self.uniform1i)(location, v0)
	}
	#[inline(always)]
	fn glUniform2i(&self, location: GLint, v0: GLint, v1: GLint) {
		(self.uniform2i)(location, v0, v1)
	}
	#[inline(always)]
	fn glUniform3i(&self, location: GLint, v0: GLint, v1: GLint, v2: GLint) {
		(self.uniform3i)(location, v0, v1, v2)
	}
	#[inline(always)]
	fn glUniform4i(&self, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) {
		(self.uniform4i)(location, v0, v1, v2, v3)
	}
	#[inline(always)]
	fn glUniform1fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
		(self.uniform1fv)(location, count, value)
	}
	#[inline(always)]
	fn glUniform2fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
		(self.uniform2fv)(location, count, value)
	}
	#[inline(always)]
	fn glUniform3fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
		(self.uniform3fv)(location, count, value)
	}
	#[inline(always)]
	fn glUniform4fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
		(self.uniform4fv)(location, count, value)
	}
	#[inline(always)]
	fn glUniform1iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
		(self.uniform1iv)(location, count, value)
	}
	#[inline(always)]
	fn glUniform2iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
		(self.uniform2iv)(location, count, value)
	}
	#[inline(always)]
	fn glUniform3iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
		(self.uniform3iv)(location, count, value)
	}
	#[inline(always)]
	fn glUniform4iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
		(self.uniform4iv)(location, count, value)
	}
	#[inline(always)]
	fn glUniformMatrix2fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.uniformmatrix2fv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glUniformMatrix3fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.uniformmatrix3fv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glUniformMatrix4fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.uniformmatrix4fv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glValidateProgram(&self, program: GLuint) {
		(self.validateprogram)(program)
	}
	#[inline(always)]
	fn glVertexAttrib1d(&self, index: GLuint, x: GLdouble) {
		(self.vertexattrib1d)(index, x)
	}
	#[inline(always)]
	fn glVertexAttrib1dv(&self, index: GLuint, v: *const GLdouble) {
		(self.vertexattrib1dv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib1f(&self, index: GLuint, x: GLfloat) {
		(self.vertexattrib1f)(index, x)
	}
	#[inline(always)]
	fn glVertexAttrib1fv(&self, index: GLuint, v: *const GLfloat) {
		(self.vertexattrib1fv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib1s(&self, index: GLuint, x: GLshort) {
		(self.vertexattrib1s)(index, x)
	}
	#[inline(always)]
	fn glVertexAttrib1sv(&self, index: GLuint, v: *const GLshort) {
		(self.vertexattrib1sv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib2d(&self, index: GLuint, x: GLdouble, y: GLdouble) {
		(self.vertexattrib2d)(index, x, y)
	}
	#[inline(always)]
	fn glVertexAttrib2dv(&self, index: GLuint, v: *const GLdouble) {
		(self.vertexattrib2dv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib2f(&self, index: GLuint, x: GLfloat, y: GLfloat) {
		(self.vertexattrib2f)(index, x, y)
	}
	#[inline(always)]
	fn glVertexAttrib2fv(&self, index: GLuint, v: *const GLfloat) {
		(self.vertexattrib2fv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib2s(&self, index: GLuint, x: GLshort, y: GLshort) {
		(self.vertexattrib2s)(index, x, y)
	}
	#[inline(always)]
	fn glVertexAttrib2sv(&self, index: GLuint, v: *const GLshort) {
		(self.vertexattrib2sv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib3d(&self, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) {
		(self.vertexattrib3d)(index, x, y, z)
	}
	#[inline(always)]
	fn glVertexAttrib3dv(&self, index: GLuint, v: *const GLdouble) {
		(self.vertexattrib3dv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib3f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) {
		(self.vertexattrib3f)(index, x, y, z)
	}
	#[inline(always)]
	fn glVertexAttrib3fv(&self, index: GLuint, v: *const GLfloat) {
		(self.vertexattrib3fv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib3s(&self, index: GLuint, x: GLshort, y: GLshort, z: GLshort) {
		(self.vertexattrib3s)(index, x, y, z)
	}
	#[inline(always)]
	fn glVertexAttrib3sv(&self, index: GLuint, v: *const GLshort) {
		(self.vertexattrib3sv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib4Nbv(&self, index: GLuint, v: *const GLbyte) {
		(self.vertexattrib4nbv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib4Niv(&self, index: GLuint, v: *const GLint) {
		(self.vertexattrib4niv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib4Nsv(&self, index: GLuint, v: *const GLshort) {
		(self.vertexattrib4nsv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib4Nub(&self, index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte) {
		(self.vertexattrib4nub)(index, x, y, z, w)
	}
	#[inline(always)]
	fn glVertexAttrib4Nubv(&self, index: GLuint, v: *const GLubyte) {
		(self.vertexattrib4nubv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib4Nuiv(&self, index: GLuint, v: *const GLuint) {
		(self.vertexattrib4nuiv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib4Nusv(&self, index: GLuint, v: *const GLushort) {
		(self.vertexattrib4nusv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib4bv(&self, index: GLuint, v: *const GLbyte) {
		(self.vertexattrib4bv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib4d(&self, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) {
		(self.vertexattrib4d)(index, x, y, z, w)
	}
	#[inline(always)]
	fn glVertexAttrib4dv(&self, index: GLuint, v: *const GLdouble) {
		(self.vertexattrib4dv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib4f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
		(self.vertexattrib4f)(index, x, y, z, w)
	}
	#[inline(always)]
	fn glVertexAttrib4fv(&self, index: GLuint, v: *const GLfloat) {
		(self.vertexattrib4fv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib4iv(&self, index: GLuint, v: *const GLint) {
		(self.vertexattrib4iv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib4s(&self, index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort) {
		(self.vertexattrib4s)(index, x, y, z, w)
	}
	#[inline(always)]
	fn glVertexAttrib4sv(&self, index: GLuint, v: *const GLshort) {
		(self.vertexattrib4sv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib4ubv(&self, index: GLuint, v: *const GLubyte) {
		(self.vertexattrib4ubv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib4uiv(&self, index: GLuint, v: *const GLuint) {
		(self.vertexattrib4uiv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib4usv(&self, index: GLuint, v: *const GLushort) {
		(self.vertexattrib4usv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribPointer(&self, index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void) {
		(self.vertexattribpointer)(index, size, type_, normalized, stride, pointer)
	}
	#[inline(always)]
	fn get_shading_language_version(&self) -> &'static str {
		self.shading_language_version
	}
}

impl Version20 {
	pub fn new(base: impl GL_1_0, get_proc_address: &impl Fn(&'static str) -> *const c_void) -> Self {
		let (spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 20000 {
			return Self::default();
		}
		Self {
			available: true,
			blendequationseparate: unsafe{transmute(get_proc_address("glBlendEquationSeparate"))},
			drawbuffers: unsafe{transmute(get_proc_address("glDrawBuffers"))},
			stencilopseparate: unsafe{transmute(get_proc_address("glStencilOpSeparate"))},
			stencilfuncseparate: unsafe{transmute(get_proc_address("glStencilFuncSeparate"))},
			stencilmaskseparate: unsafe{transmute(get_proc_address("glStencilMaskSeparate"))},
			attachshader: unsafe{transmute(get_proc_address("glAttachShader"))},
			bindattriblocation: unsafe{transmute(get_proc_address("glBindAttribLocation"))},
			compileshader: unsafe{transmute(get_proc_address("glCompileShader"))},
			createprogram: unsafe{transmute(get_proc_address("glCreateProgram"))},
			createshader: unsafe{transmute(get_proc_address("glCreateShader"))},
			deleteprogram: unsafe{transmute(get_proc_address("glDeleteProgram"))},
			deleteshader: unsafe{transmute(get_proc_address("glDeleteShader"))},
			detachshader: unsafe{transmute(get_proc_address("glDetachShader"))},
			disablevertexattribarray: unsafe{transmute(get_proc_address("glDisableVertexAttribArray"))},
			enablevertexattribarray: unsafe{transmute(get_proc_address("glEnableVertexAttribArray"))},
			getactiveattrib: unsafe{transmute(get_proc_address("glGetActiveAttrib"))},
			getactiveuniform: unsafe{transmute(get_proc_address("glGetActiveUniform"))},
			getattachedshaders: unsafe{transmute(get_proc_address("glGetAttachedShaders"))},
			getattriblocation: unsafe{transmute(get_proc_address("glGetAttribLocation"))},
			getprogramiv: unsafe{transmute(get_proc_address("glGetProgramiv"))},
			getprograminfolog: unsafe{transmute(get_proc_address("glGetProgramInfoLog"))},
			getshaderiv: unsafe{transmute(get_proc_address("glGetShaderiv"))},
			getshaderinfolog: unsafe{transmute(get_proc_address("glGetShaderInfoLog"))},
			getshadersource: unsafe{transmute(get_proc_address("glGetShaderSource"))},
			getuniformlocation: unsafe{transmute(get_proc_address("glGetUniformLocation"))},
			getuniformfv: unsafe{transmute(get_proc_address("glGetUniformfv"))},
			getuniformiv: unsafe{transmute(get_proc_address("glGetUniformiv"))},
			getvertexattribdv: unsafe{transmute(get_proc_address("glGetVertexAttribdv"))},
			getvertexattribfv: unsafe{transmute(get_proc_address("glGetVertexAttribfv"))},
			getvertexattribiv: unsafe{transmute(get_proc_address("glGetVertexAttribiv"))},
			getvertexattribpointerv: unsafe{transmute(get_proc_address("glGetVertexAttribPointerv"))},
			isprogram: unsafe{transmute(get_proc_address("glIsProgram"))},
			isshader: unsafe{transmute(get_proc_address("glIsShader"))},
			linkprogram: unsafe{transmute(get_proc_address("glLinkProgram"))},
			shadersource: unsafe{transmute(get_proc_address("glShaderSource"))},
			useprogram: unsafe{transmute(get_proc_address("glUseProgram"))},
			uniform1f: unsafe{transmute(get_proc_address("glUniform1f"))},
			uniform2f: unsafe{transmute(get_proc_address("glUniform2f"))},
			uniform3f: unsafe{transmute(get_proc_address("glUniform3f"))},
			uniform4f: unsafe{transmute(get_proc_address("glUniform4f"))},
			uniform1i: unsafe{transmute(get_proc_address("glUniform1i"))},
			uniform2i: unsafe{transmute(get_proc_address("glUniform2i"))},
			uniform3i: unsafe{transmute(get_proc_address("glUniform3i"))},
			uniform4i: unsafe{transmute(get_proc_address("glUniform4i"))},
			uniform1fv: unsafe{transmute(get_proc_address("glUniform1fv"))},
			uniform2fv: unsafe{transmute(get_proc_address("glUniform2fv"))},
			uniform3fv: unsafe{transmute(get_proc_address("glUniform3fv"))},
			uniform4fv: unsafe{transmute(get_proc_address("glUniform4fv"))},
			uniform1iv: unsafe{transmute(get_proc_address("glUniform1iv"))},
			uniform2iv: unsafe{transmute(get_proc_address("glUniform2iv"))},
			uniform3iv: unsafe{transmute(get_proc_address("glUniform3iv"))},
			uniform4iv: unsafe{transmute(get_proc_address("glUniform4iv"))},
			uniformmatrix2fv: unsafe{transmute(get_proc_address("glUniformMatrix2fv"))},
			uniformmatrix3fv: unsafe{transmute(get_proc_address("glUniformMatrix3fv"))},
			uniformmatrix4fv: unsafe{transmute(get_proc_address("glUniformMatrix4fv"))},
			validateprogram: unsafe{transmute(get_proc_address("glValidateProgram"))},
			vertexattrib1d: unsafe{transmute(get_proc_address("glVertexAttrib1d"))},
			vertexattrib1dv: unsafe{transmute(get_proc_address("glVertexAttrib1dv"))},
			vertexattrib1f: unsafe{transmute(get_proc_address("glVertexAttrib1f"))},
			vertexattrib1fv: unsafe{transmute(get_proc_address("glVertexAttrib1fv"))},
			vertexattrib1s: unsafe{transmute(get_proc_address("glVertexAttrib1s"))},
			vertexattrib1sv: unsafe{transmute(get_proc_address("glVertexAttrib1sv"))},
			vertexattrib2d: unsafe{transmute(get_proc_address("glVertexAttrib2d"))},
			vertexattrib2dv: unsafe{transmute(get_proc_address("glVertexAttrib2dv"))},
			vertexattrib2f: unsafe{transmute(get_proc_address("glVertexAttrib2f"))},
			vertexattrib2fv: unsafe{transmute(get_proc_address("glVertexAttrib2fv"))},
			vertexattrib2s: unsafe{transmute(get_proc_address("glVertexAttrib2s"))},
			vertexattrib2sv: unsafe{transmute(get_proc_address("glVertexAttrib2sv"))},
			vertexattrib3d: unsafe{transmute(get_proc_address("glVertexAttrib3d"))},
			vertexattrib3dv: unsafe{transmute(get_proc_address("glVertexAttrib3dv"))},
			vertexattrib3f: unsafe{transmute(get_proc_address("glVertexAttrib3f"))},
			vertexattrib3fv: unsafe{transmute(get_proc_address("glVertexAttrib3fv"))},
			vertexattrib3s: unsafe{transmute(get_proc_address("glVertexAttrib3s"))},
			vertexattrib3sv: unsafe{transmute(get_proc_address("glVertexAttrib3sv"))},
			vertexattrib4nbv: unsafe{transmute(get_proc_address("glVertexAttrib4Nbv"))},
			vertexattrib4niv: unsafe{transmute(get_proc_address("glVertexAttrib4Niv"))},
			vertexattrib4nsv: unsafe{transmute(get_proc_address("glVertexAttrib4Nsv"))},
			vertexattrib4nub: unsafe{transmute(get_proc_address("glVertexAttrib4Nub"))},
			vertexattrib4nubv: unsafe{transmute(get_proc_address("glVertexAttrib4Nubv"))},
			vertexattrib4nuiv: unsafe{transmute(get_proc_address("glVertexAttrib4Nuiv"))},
			vertexattrib4nusv: unsafe{transmute(get_proc_address("glVertexAttrib4Nusv"))},
			vertexattrib4bv: unsafe{transmute(get_proc_address("glVertexAttrib4bv"))},
			vertexattrib4d: unsafe{transmute(get_proc_address("glVertexAttrib4d"))},
			vertexattrib4dv: unsafe{transmute(get_proc_address("glVertexAttrib4dv"))},
			vertexattrib4f: unsafe{transmute(get_proc_address("glVertexAttrib4f"))},
			vertexattrib4fv: unsafe{transmute(get_proc_address("glVertexAttrib4fv"))},
			vertexattrib4iv: unsafe{transmute(get_proc_address("glVertexAttrib4iv"))},
			vertexattrib4s: unsafe{transmute(get_proc_address("glVertexAttrib4s"))},
			vertexattrib4sv: unsafe{transmute(get_proc_address("glVertexAttrib4sv"))},
			vertexattrib4ubv: unsafe{transmute(get_proc_address("glVertexAttrib4ubv"))},
			vertexattrib4uiv: unsafe{transmute(get_proc_address("glVertexAttrib4uiv"))},
			vertexattrib4usv: unsafe{transmute(get_proc_address("glVertexAttrib4usv"))},
			vertexattribpointer: unsafe{transmute(get_proc_address("glVertexAttribPointer"))},
			shading_language_version: unsafe{CStr::from_ptr(base.glGetString(SHADING_LANGUAGE_VERSION) as *const i8)}.to_str().unwrap(),
		}
	}
	#[inline(always)]
	pub fn get_available(&self) -> bool {
		self.available
	}
}

impl Default for Version20 {
	fn default() -> Self {
		Self {
			available: false,
			blendequationseparate: null(),
			drawbuffers: null(),
			stencilopseparate: null(),
			stencilfuncseparate: null(),
			stencilmaskseparate: null(),
			attachshader: null(),
			bindattriblocation: null(),
			compileshader: null(),
			createprogram: null(),
			createshader: null(),
			deleteprogram: null(),
			deleteshader: null(),
			detachshader: null(),
			disablevertexattribarray: null(),
			enablevertexattribarray: null(),
			getactiveattrib: null(),
			getactiveuniform: null(),
			getattachedshaders: null(),
			getattriblocation: null(),
			getprogramiv: null(),
			getprograminfolog: null(),
			getshaderiv: null(),
			getshaderinfolog: null(),
			getshadersource: null(),
			getuniformlocation: null(),
			getuniformfv: null(),
			getuniformiv: null(),
			getvertexattribdv: null(),
			getvertexattribfv: null(),
			getvertexattribiv: null(),
			getvertexattribpointerv: null(),
			isprogram: null(),
			isshader: null(),
			linkprogram: null(),
			shadersource: null(),
			useprogram: null(),
			uniform1f: null(),
			uniform2f: null(),
			uniform3f: null(),
			uniform4f: null(),
			uniform1i: null(),
			uniform2i: null(),
			uniform3i: null(),
			uniform4i: null(),
			uniform1fv: null(),
			uniform2fv: null(),
			uniform3fv: null(),
			uniform4fv: null(),
			uniform1iv: null(),
			uniform2iv: null(),
			uniform3iv: null(),
			uniform4iv: null(),
			uniformmatrix2fv: null(),
			uniformmatrix3fv: null(),
			uniformmatrix4fv: null(),
			validateprogram: null(),
			vertexattrib1d: null(),
			vertexattrib1dv: null(),
			vertexattrib1f: null(),
			vertexattrib1fv: null(),
			vertexattrib1s: null(),
			vertexattrib1sv: null(),
			vertexattrib2d: null(),
			vertexattrib2dv: null(),
			vertexattrib2f: null(),
			vertexattrib2fv: null(),
			vertexattrib2s: null(),
			vertexattrib2sv: null(),
			vertexattrib3d: null(),
			vertexattrib3dv: null(),
			vertexattrib3f: null(),
			vertexattrib3fv: null(),
			vertexattrib3s: null(),
			vertexattrib3sv: null(),
			vertexattrib4nbv: null(),
			vertexattrib4niv: null(),
			vertexattrib4nsv: null(),
			vertexattrib4nub: null(),
			vertexattrib4nubv: null(),
			vertexattrib4nuiv: null(),
			vertexattrib4nusv: null(),
			vertexattrib4bv: null(),
			vertexattrib4d: null(),
			vertexattrib4dv: null(),
			vertexattrib4f: null(),
			vertexattrib4fv: null(),
			vertexattrib4iv: null(),
			vertexattrib4s: null(),
			vertexattrib4sv: null(),
			vertexattrib4ubv: null(),
			vertexattrib4uiv: null(),
			vertexattrib4usv: null(),
			vertexattribpointer: null(),
			shading_language_version: "unknown",
		}
	}
}

type PFNGLUNIFORMMATRIX2X3FVPROC = extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat);
type PFNGLUNIFORMMATRIX3X2FVPROC = extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat);
type PFNGLUNIFORMMATRIX2X4FVPROC = extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat);
type PFNGLUNIFORMMATRIX4X2FVPROC = extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat);
type PFNGLUNIFORMMATRIX3X4FVPROC = extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat);
type PFNGLUNIFORMMATRIX4X3FVPROC = extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat);
const PIXEL_PACK_BUFFER: GLenum = 0x88EB;
const PIXEL_UNPACK_BUFFER: GLenum = 0x88EC;
const PIXEL_PACK_BUFFER_BINDING: GLenum = 0x88ED;
const PIXEL_UNPACK_BUFFER_BINDING: GLenum = 0x88EF;
const FLOAT_MAT2x3: GLenum = 0x8B65;
const FLOAT_MAT2x4: GLenum = 0x8B66;
const FLOAT_MAT3x2: GLenum = 0x8B67;
const FLOAT_MAT3x4: GLenum = 0x8B68;
const FLOAT_MAT4x2: GLenum = 0x8B69;
const FLOAT_MAT4x3: GLenum = 0x8B6A;
const SRGB: GLenum = 0x8C40;
const SRGB8: GLenum = 0x8C41;
const SRGB_ALPHA: GLenum = 0x8C42;
const SRGB8_ALPHA8: GLenum = 0x8C43;
const COMPRESSED_SRGB: GLenum = 0x8C48;
const COMPRESSED_SRGB_ALPHA: GLenum = 0x8C49;
const CURRENT_RASTER_SECONDARY_COLOR: GLenum = 0x845F;
const SLUMINANCE_ALPHA: GLenum = 0x8C44;
const SLUMINANCE8_ALPHA8: GLenum = 0x8C45;
const SLUMINANCE: GLenum = 0x8C46;
const SLUMINANCE8: GLenum = 0x8C47;
const COMPRESSED_SLUMINANCE: GLenum = 0x8C4A;
const COMPRESSED_SLUMINANCE_ALPHA: GLenum = 0x8C4B;

pub trait GL_2_1 {
	fn glUniformMatrix2x3fv(&self, _: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glUniformMatrix2x3fv` is NULL");
	}
	fn glUniformMatrix3x2fv(&self, _: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glUniformMatrix3x2fv` is NULL");
	}
	fn glUniformMatrix2x4fv(&self, _: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glUniformMatrix2x4fv` is NULL");
	}
	fn glUniformMatrix4x2fv(&self, _: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glUniformMatrix4x2fv` is NULL");
	}
	fn glUniformMatrix3x4fv(&self, _: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glUniformMatrix3x4fv` is NULL");
	}
	fn glUniformMatrix4x3fv(&self, _: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glUniformMatrix4x3fv` is NULL");
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Version21 {
	available: bool,
	uniformmatrix2x3fv: PFNGLUNIFORMMATRIX2X3FVPROC,
	uniformmatrix3x2fv: PFNGLUNIFORMMATRIX3X2FVPROC,
	uniformmatrix2x4fv: PFNGLUNIFORMMATRIX2X4FVPROC,
	uniformmatrix4x2fv: PFNGLUNIFORMMATRIX4X2FVPROC,
	uniformmatrix3x4fv: PFNGLUNIFORMMATRIX3X4FVPROC,
	uniformmatrix4x3fv: PFNGLUNIFORMMATRIX4X3FVPROC,
}

impl GL_2_1 for Version21 {
	#[inline(always)]
	fn glUniformMatrix2x3fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.uniformmatrix2x3fv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glUniformMatrix3x2fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.uniformmatrix3x2fv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glUniformMatrix2x4fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.uniformmatrix2x4fv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glUniformMatrix4x2fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.uniformmatrix4x2fv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glUniformMatrix3x4fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.uniformmatrix3x4fv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glUniformMatrix4x3fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.uniformmatrix4x3fv)(location, count, transpose, value)
	}
}

impl Version21 {
	pub fn new(base: impl GL_1_0, get_proc_address: &impl Fn(&'static str) -> *const c_void) -> Self {
		let (spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 20100 {
			return Self::default();
		}
		Self {
			available: true,
			uniformmatrix2x3fv: unsafe{transmute(get_proc_address("glUniformMatrix2x3fv"))},
			uniformmatrix3x2fv: unsafe{transmute(get_proc_address("glUniformMatrix3x2fv"))},
			uniformmatrix2x4fv: unsafe{transmute(get_proc_address("glUniformMatrix2x4fv"))},
			uniformmatrix4x2fv: unsafe{transmute(get_proc_address("glUniformMatrix4x2fv"))},
			uniformmatrix3x4fv: unsafe{transmute(get_proc_address("glUniformMatrix3x4fv"))},
			uniformmatrix4x3fv: unsafe{transmute(get_proc_address("glUniformMatrix4x3fv"))},
		}
	}
	#[inline(always)]
	pub fn get_available(&self) -> bool {
		self.available
	}
}

impl Default for Version21 {
	fn default() -> Self {
		Self {
			available: false,
			uniformmatrix2x3fv: null(),
			uniformmatrix3x2fv: null(),
			uniformmatrix2x4fv: null(),
			uniformmatrix4x2fv: null(),
			uniformmatrix3x4fv: null(),
			uniformmatrix4x3fv: null(),
		}
	}
}

type GLhalf = khronos_uint16_t;
type PFNGLCOLORMASKIPROC = extern "system" fn(GLuint, GLboolean, GLboolean, GLboolean, GLboolean);
type PFNGLGETBOOLEANI_VPROC = extern "system" fn(GLenum, GLuint, *mut GLboolean);
type PFNGLGETINTEGERI_VPROC = extern "system" fn(GLenum, GLuint, *mut GLint);
type PFNGLENABLEIPROC = extern "system" fn(GLenum, GLuint);
type PFNGLDISABLEIPROC = extern "system" fn(GLenum, GLuint);
type PFNGLISENABLEDIPROC = extern "system" fn(GLenum, GLuint) -> GLboolean;
type PFNGLBEGINTRANSFORMFEEDBACKPROC = extern "system" fn(GLenum);
type PFNGLENDTRANSFORMFEEDBACKPROC = extern "system" fn();
type PFNGLBINDBUFFERRANGEPROC = extern "system" fn(GLenum, GLuint, GLuint, GLintptr, GLsizeiptr);
type PFNGLBINDBUFFERBASEPROC = extern "system" fn(GLenum, GLuint, GLuint);
type PFNGLTRANSFORMFEEDBACKVARYINGSPROC = extern "system" fn(GLuint, GLsizei, *const *const GLchar, GLenum);
type PFNGLGETTRANSFORMFEEDBACKVARYINGPROC = extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLsizei, *mut GLenum, *mut GLchar);
type PFNGLCLAMPCOLORPROC = extern "system" fn(GLenum, GLenum);
type PFNGLBEGINCONDITIONALRENDERPROC = extern "system" fn(GLuint, GLenum);
type PFNGLENDCONDITIONALRENDERPROC = extern "system" fn();
type PFNGLVERTEXATTRIBIPOINTERPROC = extern "system" fn(GLuint, GLint, GLenum, GLsizei, *const c_void);
type PFNGLGETVERTEXATTRIBIIVPROC = extern "system" fn(GLuint, GLenum, *mut GLint);
type PFNGLGETVERTEXATTRIBIUIVPROC = extern "system" fn(GLuint, GLenum, *mut GLuint);
type PFNGLVERTEXATTRIBI1IPROC = extern "system" fn(GLuint, GLint);
type PFNGLVERTEXATTRIBI2IPROC = extern "system" fn(GLuint, GLint, GLint);
type PFNGLVERTEXATTRIBI3IPROC = extern "system" fn(GLuint, GLint, GLint, GLint);
type PFNGLVERTEXATTRIBI4IPROC = extern "system" fn(GLuint, GLint, GLint, GLint, GLint);
type PFNGLVERTEXATTRIBI1UIPROC = extern "system" fn(GLuint, GLuint);
type PFNGLVERTEXATTRIBI2UIPROC = extern "system" fn(GLuint, GLuint, GLuint);
type PFNGLVERTEXATTRIBI3UIPROC = extern "system" fn(GLuint, GLuint, GLuint, GLuint);
type PFNGLVERTEXATTRIBI4UIPROC = extern "system" fn(GLuint, GLuint, GLuint, GLuint, GLuint);
type PFNGLVERTEXATTRIBI1IVPROC = extern "system" fn(GLuint, *const GLint);
type PFNGLVERTEXATTRIBI2IVPROC = extern "system" fn(GLuint, *const GLint);
type PFNGLVERTEXATTRIBI3IVPROC = extern "system" fn(GLuint, *const GLint);
type PFNGLVERTEXATTRIBI4IVPROC = extern "system" fn(GLuint, *const GLint);
type PFNGLVERTEXATTRIBI1UIVPROC = extern "system" fn(GLuint, *const GLuint);
type PFNGLVERTEXATTRIBI2UIVPROC = extern "system" fn(GLuint, *const GLuint);
type PFNGLVERTEXATTRIBI3UIVPROC = extern "system" fn(GLuint, *const GLuint);
type PFNGLVERTEXATTRIBI4UIVPROC = extern "system" fn(GLuint, *const GLuint);
type PFNGLVERTEXATTRIBI4BVPROC = extern "system" fn(GLuint, *const GLbyte);
type PFNGLVERTEXATTRIBI4SVPROC = extern "system" fn(GLuint, *const GLshort);
type PFNGLVERTEXATTRIBI4UBVPROC = extern "system" fn(GLuint, *const GLubyte);
type PFNGLVERTEXATTRIBI4USVPROC = extern "system" fn(GLuint, *const GLushort);
type PFNGLGETUNIFORMUIVPROC = extern "system" fn(GLuint, GLint, *mut GLuint);
type PFNGLBINDFRAGDATALOCATIONPROC = extern "system" fn(GLuint, GLuint, *const GLchar);
type PFNGLGETFRAGDATALOCATIONPROC = extern "system" fn(GLuint, *const GLchar) -> GLint;
type PFNGLUNIFORM1UIPROC = extern "system" fn(GLint, GLuint);
type PFNGLUNIFORM2UIPROC = extern "system" fn(GLint, GLuint, GLuint);
type PFNGLUNIFORM3UIPROC = extern "system" fn(GLint, GLuint, GLuint, GLuint);
type PFNGLUNIFORM4UIPROC = extern "system" fn(GLint, GLuint, GLuint, GLuint, GLuint);
type PFNGLUNIFORM1UIVPROC = extern "system" fn(GLint, GLsizei, *const GLuint);
type PFNGLUNIFORM2UIVPROC = extern "system" fn(GLint, GLsizei, *const GLuint);
type PFNGLUNIFORM3UIVPROC = extern "system" fn(GLint, GLsizei, *const GLuint);
type PFNGLUNIFORM4UIVPROC = extern "system" fn(GLint, GLsizei, *const GLuint);
type PFNGLTEXPARAMETERIIVPROC = extern "system" fn(GLenum, GLenum, *const GLint);
type PFNGLTEXPARAMETERIUIVPROC = extern "system" fn(GLenum, GLenum, *const GLuint);
type PFNGLGETTEXPARAMETERIIVPROC = extern "system" fn(GLenum, GLenum, *mut GLint);
type PFNGLGETTEXPARAMETERIUIVPROC = extern "system" fn(GLenum, GLenum, *mut GLuint);
type PFNGLCLEARBUFFERIVPROC = extern "system" fn(GLenum, GLint, *const GLint);
type PFNGLCLEARBUFFERUIVPROC = extern "system" fn(GLenum, GLint, *const GLuint);
type PFNGLCLEARBUFFERFVPROC = extern "system" fn(GLenum, GLint, *const GLfloat);
type PFNGLCLEARBUFFERFIPROC = extern "system" fn(GLenum, GLint, GLfloat, GLint);
type PFNGLGETSTRINGIPROC = extern "system" fn(GLenum, GLuint) -> *const GLubyte;
type PFNGLISRENDERBUFFERPROC = extern "system" fn(GLuint) -> GLboolean;
type PFNGLBINDRENDERBUFFERPROC = extern "system" fn(GLenum, GLuint);
type PFNGLDELETERENDERBUFFERSPROC = extern "system" fn(GLsizei, *const GLuint);
type PFNGLGENRENDERBUFFERSPROC = extern "system" fn(GLsizei, *mut GLuint);
type PFNGLRENDERBUFFERSTORAGEPROC = extern "system" fn(GLenum, GLenum, GLsizei, GLsizei);
type PFNGLGETRENDERBUFFERPARAMETERIVPROC = extern "system" fn(GLenum, GLenum, *mut GLint);
type PFNGLISFRAMEBUFFERPROC = extern "system" fn(GLuint) -> GLboolean;
type PFNGLBINDFRAMEBUFFERPROC = extern "system" fn(GLenum, GLuint);
type PFNGLDELETEFRAMEBUFFERSPROC = extern "system" fn(GLsizei, *const GLuint);
type PFNGLGENFRAMEBUFFERSPROC = extern "system" fn(GLsizei, *mut GLuint);
type PFNGLCHECKFRAMEBUFFERSTATUSPROC = extern "system" fn(GLenum) -> GLenum;
type PFNGLFRAMEBUFFERTEXTURE1DPROC = extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint);
type PFNGLFRAMEBUFFERTEXTURE2DPROC = extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint);
type PFNGLFRAMEBUFFERTEXTURE3DPROC = extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint, GLint);
type PFNGLFRAMEBUFFERRENDERBUFFERPROC = extern "system" fn(GLenum, GLenum, GLenum, GLuint);
type PFNGLGETFRAMEBUFFERATTACHMENTPARAMETERIVPROC = extern "system" fn(GLenum, GLenum, GLenum, *mut GLint);
type PFNGLGENERATEMIPMAPPROC = extern "system" fn(GLenum);
type PFNGLBLITFRAMEBUFFERPROC = extern "system" fn(GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLbitfield, GLenum);
type PFNGLRENDERBUFFERSTORAGEMULTISAMPLEPROC = extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei);
type PFNGLFRAMEBUFFERTEXTURELAYERPROC = extern "system" fn(GLenum, GLenum, GLuint, GLint, GLint);
type PFNGLMAPBUFFERRANGEPROC = extern "system" fn(GLenum, GLintptr, GLsizeiptr, GLbitfield) -> *mut c_void;
type PFNGLFLUSHMAPPEDBUFFERRANGEPROC = extern "system" fn(GLenum, GLintptr, GLsizeiptr);
type PFNGLBINDVERTEXARRAYPROC = extern "system" fn(GLuint);
type PFNGLDELETEVERTEXARRAYSPROC = extern "system" fn(GLsizei, *const GLuint);
type PFNGLGENVERTEXARRAYSPROC = extern "system" fn(GLsizei, *mut GLuint);
type PFNGLISVERTEXARRAYPROC = extern "system" fn(GLuint) -> GLboolean;
const COMPARE_REF_TO_TEXTURE: GLenum = 0x884E;
const CLIP_DISTANCE0: GLenum = 0x3000;
const CLIP_DISTANCE1: GLenum = 0x3001;
const CLIP_DISTANCE2: GLenum = 0x3002;
const CLIP_DISTANCE3: GLenum = 0x3003;
const CLIP_DISTANCE4: GLenum = 0x3004;
const CLIP_DISTANCE5: GLenum = 0x3005;
const CLIP_DISTANCE6: GLenum = 0x3006;
const CLIP_DISTANCE7: GLenum = 0x3007;
const MAX_CLIP_DISTANCES: GLenum = 0x0D32;
const MAJOR_VERSION: GLenum = 0x821B;
const MINOR_VERSION: GLenum = 0x821C;
const NUM_EXTENSIONS: GLenum = 0x821D;
const CONTEXT_FLAGS: GLenum = 0x821E;
const COMPRESSED_RED: GLenum = 0x8225;
const COMPRESSED_RG: GLenum = 0x8226;
const CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: GLbitfield = 0x00000001;
const RGBA32F: GLenum = 0x8814;
const RGB32F: GLenum = 0x8815;
const RGBA16F: GLenum = 0x881A;
const RGB16F: GLenum = 0x881B;
const VERTEX_ATTRIB_ARRAY_INTEGER: GLenum = 0x88FD;
const MAX_ARRAY_TEXTURE_LAYERS: GLenum = 0x88FF;
const MIN_PROGRAM_TEXEL_OFFSET: GLenum = 0x8904;
const MAX_PROGRAM_TEXEL_OFFSET: GLenum = 0x8905;
const CLAMP_READ_COLOR: GLenum = 0x891C;
const FIXED_ONLY: GLenum = 0x891D;
const MAX_VARYING_COMPONENTS: GLenum = 0x8B4B;
const TEXTURE_1D_ARRAY: GLenum = 0x8C18;
const PROXY_TEXTURE_1D_ARRAY: GLenum = 0x8C19;
const TEXTURE_2D_ARRAY: GLenum = 0x8C1A;
const PROXY_TEXTURE_2D_ARRAY: GLenum = 0x8C1B;
const TEXTURE_BINDING_1D_ARRAY: GLenum = 0x8C1C;
const TEXTURE_BINDING_2D_ARRAY: GLenum = 0x8C1D;
const R11F_G11F_B10F: GLenum = 0x8C3A;
const UNSIGNED_INT_10F_11F_11F_REV: GLenum = 0x8C3B;
const RGB9_E5: GLenum = 0x8C3D;
const UNSIGNED_INT_5_9_9_9_REV: GLenum = 0x8C3E;
const TEXTURE_SHARED_SIZE: GLenum = 0x8C3F;
const TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: GLenum = 0x8C76;
const TRANSFORM_FEEDBACK_BUFFER_MODE: GLenum = 0x8C7F;
const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: GLenum = 0x8C80;
const TRANSFORM_FEEDBACK_VARYINGS: GLenum = 0x8C83;
const TRANSFORM_FEEDBACK_BUFFER_START: GLenum = 0x8C84;
const TRANSFORM_FEEDBACK_BUFFER_SIZE: GLenum = 0x8C85;
const PRIMITIVES_GENERATED: GLenum = 0x8C87;
const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: GLenum = 0x8C88;
const RASTERIZER_DISCARD: GLenum = 0x8C89;
const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: GLenum = 0x8C8A;
const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: GLenum = 0x8C8B;
const INTERLEAVED_ATTRIBS: GLenum = 0x8C8C;
const SEPARATE_ATTRIBS: GLenum = 0x8C8D;
const TRANSFORM_FEEDBACK_BUFFER: GLenum = 0x8C8E;
const TRANSFORM_FEEDBACK_BUFFER_BINDING: GLenum = 0x8C8F;
const RGBA32UI: GLenum = 0x8D70;
const RGB32UI: GLenum = 0x8D71;
const RGBA16UI: GLenum = 0x8D76;
const RGB16UI: GLenum = 0x8D77;
const RGBA8UI: GLenum = 0x8D7C;
const RGB8UI: GLenum = 0x8D7D;
const RGBA32I: GLenum = 0x8D82;
const RGB32I: GLenum = 0x8D83;
const RGBA16I: GLenum = 0x8D88;
const RGB16I: GLenum = 0x8D89;
const RGBA8I: GLenum = 0x8D8E;
const RGB8I: GLenum = 0x8D8F;
const RED_INTEGER: GLenum = 0x8D94;
const GREEN_INTEGER: GLenum = 0x8D95;
const BLUE_INTEGER: GLenum = 0x8D96;
const RGB_INTEGER: GLenum = 0x8D98;
const RGBA_INTEGER: GLenum = 0x8D99;
const BGR_INTEGER: GLenum = 0x8D9A;
const BGRA_INTEGER: GLenum = 0x8D9B;
const SAMPLER_1D_ARRAY: GLenum = 0x8DC0;
const SAMPLER_2D_ARRAY: GLenum = 0x8DC1;
const SAMPLER_1D_ARRAY_SHADOW: GLenum = 0x8DC3;
const SAMPLER_2D_ARRAY_SHADOW: GLenum = 0x8DC4;
const SAMPLER_CUBE_SHADOW: GLenum = 0x8DC5;
const UNSIGNED_INT_VEC2: GLenum = 0x8DC6;
const UNSIGNED_INT_VEC3: GLenum = 0x8DC7;
const UNSIGNED_INT_VEC4: GLenum = 0x8DC8;
const INT_SAMPLER_1D: GLenum = 0x8DC9;
const INT_SAMPLER_2D: GLenum = 0x8DCA;
const INT_SAMPLER_3D: GLenum = 0x8DCB;
const INT_SAMPLER_CUBE: GLenum = 0x8DCC;
const INT_SAMPLER_1D_ARRAY: GLenum = 0x8DCE;
const INT_SAMPLER_2D_ARRAY: GLenum = 0x8DCF;
const UNSIGNED_INT_SAMPLER_1D: GLenum = 0x8DD1;
const UNSIGNED_INT_SAMPLER_2D: GLenum = 0x8DD2;
const UNSIGNED_INT_SAMPLER_3D: GLenum = 0x8DD3;
const UNSIGNED_INT_SAMPLER_CUBE: GLenum = 0x8DD4;
const UNSIGNED_INT_SAMPLER_1D_ARRAY: GLenum = 0x8DD6;
const UNSIGNED_INT_SAMPLER_2D_ARRAY: GLenum = 0x8DD7;
const QUERY_WAIT: GLenum = 0x8E13;
const QUERY_NO_WAIT: GLenum = 0x8E14;
const QUERY_BY_REGION_WAIT: GLenum = 0x8E15;
const QUERY_BY_REGION_NO_WAIT: GLenum = 0x8E16;
const BUFFER_ACCESS_FLAGS: GLenum = 0x911F;
const BUFFER_MAP_LENGTH: GLenum = 0x9120;
const BUFFER_MAP_OFFSET: GLenum = 0x9121;
const DEPTH_COMPONENT32F: GLenum = 0x8CAC;
const DEPTH32F_STENCIL8: GLenum = 0x8CAD;
const FLOAT_32_UNSIGNED_INT_24_8_REV: GLenum = 0x8DAD;
const INVALID_FRAMEBUFFER_OPERATION: GLenum = 0x0506;
const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: GLenum = 0x8210;
const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: GLenum = 0x8211;
const FRAMEBUFFER_ATTACHMENT_RED_SIZE: GLenum = 0x8212;
const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: GLenum = 0x8213;
const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: GLenum = 0x8214;
const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: GLenum = 0x8215;
const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: GLenum = 0x8216;
const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: GLenum = 0x8217;
const FRAMEBUFFER_DEFAULT: GLenum = 0x8218;
const FRAMEBUFFER_UNDEFINED: GLenum = 0x8219;
const DEPTH_STENCIL_ATTACHMENT: GLenum = 0x821A;
const MAX_RENDERBUFFER_SIZE: GLenum = 0x84E8;
const DEPTH_STENCIL: GLenum = 0x84F9;
const UNSIGNED_INT_24_8: GLenum = 0x84FA;
const DEPTH24_STENCIL8: GLenum = 0x88F0;
const TEXTURE_STENCIL_SIZE: GLenum = 0x88F1;
const TEXTURE_RED_TYPE: GLenum = 0x8C10;
const TEXTURE_GREEN_TYPE: GLenum = 0x8C11;
const TEXTURE_BLUE_TYPE: GLenum = 0x8C12;
const TEXTURE_ALPHA_TYPE: GLenum = 0x8C13;
const TEXTURE_DEPTH_TYPE: GLenum = 0x8C16;
const UNSIGNED_NORMALIZED: GLenum = 0x8C17;
const FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
const DRAW_FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
const RENDERBUFFER_BINDING: GLenum = 0x8CA7;
const READ_FRAMEBUFFER: GLenum = 0x8CA8;
const DRAW_FRAMEBUFFER: GLenum = 0x8CA9;
const READ_FRAMEBUFFER_BINDING: GLenum = 0x8CAA;
const RENDERBUFFER_SAMPLES: GLenum = 0x8CAB;
const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: GLenum = 0x8CD0;
const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: GLenum = 0x8CD1;
const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: GLenum = 0x8CD2;
const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLenum = 0x8CD3;
const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: GLenum = 0x8CD4;
const FRAMEBUFFER_COMPLETE: GLenum = 0x8CD5;
const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GLenum = 0x8CD6;
const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLenum = 0x8CD7;
const FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: GLenum = 0x8CDB;
const FRAMEBUFFER_INCOMPLETE_READ_BUFFER: GLenum = 0x8CDC;
const FRAMEBUFFER_UNSUPPORTED: GLenum = 0x8CDD;
const MAX_COLOR_ATTACHMENTS: GLenum = 0x8CDF;
const COLOR_ATTACHMENT0: GLenum = 0x8CE0;
const COLOR_ATTACHMENT1: GLenum = 0x8CE1;
const COLOR_ATTACHMENT2: GLenum = 0x8CE2;
const COLOR_ATTACHMENT3: GLenum = 0x8CE3;
const COLOR_ATTACHMENT4: GLenum = 0x8CE4;
const COLOR_ATTACHMENT5: GLenum = 0x8CE5;
const COLOR_ATTACHMENT6: GLenum = 0x8CE6;
const COLOR_ATTACHMENT7: GLenum = 0x8CE7;
const COLOR_ATTACHMENT8: GLenum = 0x8CE8;
const COLOR_ATTACHMENT9: GLenum = 0x8CE9;
const COLOR_ATTACHMENT10: GLenum = 0x8CEA;
const COLOR_ATTACHMENT11: GLenum = 0x8CEB;
const COLOR_ATTACHMENT12: GLenum = 0x8CEC;
const COLOR_ATTACHMENT13: GLenum = 0x8CED;
const COLOR_ATTACHMENT14: GLenum = 0x8CEE;
const COLOR_ATTACHMENT15: GLenum = 0x8CEF;
const COLOR_ATTACHMENT16: GLenum = 0x8CF0;
const COLOR_ATTACHMENT17: GLenum = 0x8CF1;
const COLOR_ATTACHMENT18: GLenum = 0x8CF2;
const COLOR_ATTACHMENT19: GLenum = 0x8CF3;
const COLOR_ATTACHMENT20: GLenum = 0x8CF4;
const COLOR_ATTACHMENT21: GLenum = 0x8CF5;
const COLOR_ATTACHMENT22: GLenum = 0x8CF6;
const COLOR_ATTACHMENT23: GLenum = 0x8CF7;
const COLOR_ATTACHMENT24: GLenum = 0x8CF8;
const COLOR_ATTACHMENT25: GLenum = 0x8CF9;
const COLOR_ATTACHMENT26: GLenum = 0x8CFA;
const COLOR_ATTACHMENT27: GLenum = 0x8CFB;
const COLOR_ATTACHMENT28: GLenum = 0x8CFC;
const COLOR_ATTACHMENT29: GLenum = 0x8CFD;
const COLOR_ATTACHMENT30: GLenum = 0x8CFE;
const COLOR_ATTACHMENT31: GLenum = 0x8CFF;
const DEPTH_ATTACHMENT: GLenum = 0x8D00;
const STENCIL_ATTACHMENT: GLenum = 0x8D20;
const FRAMEBUFFER: GLenum = 0x8D40;
const RENDERBUFFER: GLenum = 0x8D41;
const RENDERBUFFER_WIDTH: GLenum = 0x8D42;
const RENDERBUFFER_HEIGHT: GLenum = 0x8D43;
const RENDERBUFFER_INTERNAL_FORMAT: GLenum = 0x8D44;
const STENCIL_INDEX1: GLenum = 0x8D46;
const STENCIL_INDEX4: GLenum = 0x8D47;
const STENCIL_INDEX8: GLenum = 0x8D48;
const STENCIL_INDEX16: GLenum = 0x8D49;
const RENDERBUFFER_RED_SIZE: GLenum = 0x8D50;
const RENDERBUFFER_GREEN_SIZE: GLenum = 0x8D51;
const RENDERBUFFER_BLUE_SIZE: GLenum = 0x8D52;
const RENDERBUFFER_ALPHA_SIZE: GLenum = 0x8D53;
const RENDERBUFFER_DEPTH_SIZE: GLenum = 0x8D54;
const RENDERBUFFER_STENCIL_SIZE: GLenum = 0x8D55;
const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: GLenum = 0x8D56;
const MAX_SAMPLES: GLenum = 0x8D57;
const INDEX: GLenum = 0x8222;
const TEXTURE_LUMINANCE_TYPE: GLenum = 0x8C14;
const TEXTURE_INTENSITY_TYPE: GLenum = 0x8C15;
const FRAMEBUFFER_SRGB: GLenum = 0x8DB9;
const HALF_FLOAT: GLenum = 0x140B;
const MAP_READ_BIT: GLbitfield = 0x0001;
const MAP_WRITE_BIT: GLbitfield = 0x0002;
const MAP_INVALIDATE_RANGE_BIT: GLbitfield = 0x0004;
const MAP_INVALIDATE_BUFFER_BIT: GLbitfield = 0x0008;
const MAP_FLUSH_EXPLICIT_BIT: GLbitfield = 0x0010;
const MAP_UNSYNCHRONIZED_BIT: GLbitfield = 0x0020;
const COMPRESSED_RED_RGTC1: GLenum = 0x8DBB;
const COMPRESSED_SIGNED_RED_RGTC1: GLenum = 0x8DBC;
const COMPRESSED_RG_RGTC2: GLenum = 0x8DBD;
const COMPRESSED_SIGNED_RG_RGTC2: GLenum = 0x8DBE;
const RG: GLenum = 0x8227;
const RG_INTEGER: GLenum = 0x8228;
const R8: GLenum = 0x8229;
const R16: GLenum = 0x822A;
const RG8: GLenum = 0x822B;
const RG16: GLenum = 0x822C;
const R16F: GLenum = 0x822D;
const R32F: GLenum = 0x822E;
const RG16F: GLenum = 0x822F;
const RG32F: GLenum = 0x8230;
const R8I: GLenum = 0x8231;
const R8UI: GLenum = 0x8232;
const R16I: GLenum = 0x8233;
const R16UI: GLenum = 0x8234;
const R32I: GLenum = 0x8235;
const R32UI: GLenum = 0x8236;
const RG8I: GLenum = 0x8237;
const RG8UI: GLenum = 0x8238;
const RG16I: GLenum = 0x8239;
const RG16UI: GLenum = 0x823A;
const RG32I: GLenum = 0x823B;
const RG32UI: GLenum = 0x823C;
const VERTEX_ARRAY_BINDING: GLenum = 0x85B5;
const CLAMP_VERTEX_COLOR: GLenum = 0x891A;
const CLAMP_FRAGMENT_COLOR: GLenum = 0x891B;
const ALPHA_INTEGER: GLenum = 0x8D97;

pub trait GL_3_0 {
	fn glColorMaski(&self, _: GLuint, _: GLboolean, _: GLboolean, _: GLboolean, _: GLboolean) {
		panic!("OpenGL function pointer of `glColorMaski` is NULL");
	}
	fn glGetBooleani_v(&self, _: GLenum, _: GLuint, _: *mut GLboolean) {
		panic!("OpenGL function pointer of `glGetBooleani_v` is NULL");
	}
	fn glGetIntegeri_v(&self, _: GLenum, _: GLuint, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetIntegeri_v` is NULL");
	}
	fn glEnablei(&self, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glEnablei` is NULL");
	}
	fn glDisablei(&self, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glDisablei` is NULL");
	}
	fn glIsEnabledi(&self, _: GLenum, _: GLuint) -> GLboolean {
		panic!("OpenGL function pointer of `glIsEnabledi` is NULL");
	}
	fn glBeginTransformFeedback(&self, _: GLenum) {
		panic!("OpenGL function pointer of `glBeginTransformFeedback` is NULL");
	}
	fn glEndTransformFeedback(&self) {
		panic!("OpenGL function pointer of `glEndTransformFeedback` is NULL");
	}
	fn glBindBufferRange(&self, _: GLenum, _: GLuint, _: GLuint, _: GLintptr, _: GLsizeiptr) {
		panic!("OpenGL function pointer of `glBindBufferRange` is NULL");
	}
	fn glBindBufferBase(&self, _: GLenum, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glBindBufferBase` is NULL");
	}
	fn glTransformFeedbackVaryings(&self, _: GLuint, _: GLsizei, _: *const *const GLchar, _: GLenum) {
		panic!("OpenGL function pointer of `glTransformFeedbackVaryings` is NULL");
	}
	fn glGetTransformFeedbackVarying(&self, _: GLuint, _: GLuint, _: GLsizei, _: *mut GLsizei, _: *mut GLsizei, _: *mut GLenum, _: *mut GLchar) {
		panic!("OpenGL function pointer of `glGetTransformFeedbackVarying` is NULL");
	}
	fn glClampColor(&self, _: GLenum, _: GLenum) {
		panic!("OpenGL function pointer of `glClampColor` is NULL");
	}
	fn glBeginConditionalRender(&self, _: GLuint, _: GLenum) {
		panic!("OpenGL function pointer of `glBeginConditionalRender` is NULL");
	}
	fn glEndConditionalRender(&self) {
		panic!("OpenGL function pointer of `glEndConditionalRender` is NULL");
	}
	fn glVertexAttribIPointer(&self, _: GLuint, _: GLint, _: GLenum, _: GLsizei, _: *const c_void) {
		panic!("OpenGL function pointer of `glVertexAttribIPointer` is NULL");
	}
	fn glGetVertexAttribIiv(&self, _: GLuint, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetVertexAttribIiv` is NULL");
	}
	fn glGetVertexAttribIuiv(&self, _: GLuint, _: GLenum, _: *mut GLuint) {
		panic!("OpenGL function pointer of `glGetVertexAttribIuiv` is NULL");
	}
	fn glVertexAttribI1i(&self, _: GLuint, _: GLint) {
		panic!("OpenGL function pointer of `glVertexAttribI1i` is NULL");
	}
	fn glVertexAttribI2i(&self, _: GLuint, _: GLint, _: GLint) {
		panic!("OpenGL function pointer of `glVertexAttribI2i` is NULL");
	}
	fn glVertexAttribI3i(&self, _: GLuint, _: GLint, _: GLint, _: GLint) {
		panic!("OpenGL function pointer of `glVertexAttribI3i` is NULL");
	}
	fn glVertexAttribI4i(&self, _: GLuint, _: GLint, _: GLint, _: GLint, _: GLint) {
		panic!("OpenGL function pointer of `glVertexAttribI4i` is NULL");
	}
	fn glVertexAttribI1ui(&self, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glVertexAttribI1ui` is NULL");
	}
	fn glVertexAttribI2ui(&self, _: GLuint, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glVertexAttribI2ui` is NULL");
	}
	fn glVertexAttribI3ui(&self, _: GLuint, _: GLuint, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glVertexAttribI3ui` is NULL");
	}
	fn glVertexAttribI4ui(&self, _: GLuint, _: GLuint, _: GLuint, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glVertexAttribI4ui` is NULL");
	}
	fn glVertexAttribI1iv(&self, _: GLuint, _: *const GLint) {
		panic!("OpenGL function pointer of `glVertexAttribI1iv` is NULL");
	}
	fn glVertexAttribI2iv(&self, _: GLuint, _: *const GLint) {
		panic!("OpenGL function pointer of `glVertexAttribI2iv` is NULL");
	}
	fn glVertexAttribI3iv(&self, _: GLuint, _: *const GLint) {
		panic!("OpenGL function pointer of `glVertexAttribI3iv` is NULL");
	}
	fn glVertexAttribI4iv(&self, _: GLuint, _: *const GLint) {
		panic!("OpenGL function pointer of `glVertexAttribI4iv` is NULL");
	}
	fn glVertexAttribI1uiv(&self, _: GLuint, _: *const GLuint) {
		panic!("OpenGL function pointer of `glVertexAttribI1uiv` is NULL");
	}
	fn glVertexAttribI2uiv(&self, _: GLuint, _: *const GLuint) {
		panic!("OpenGL function pointer of `glVertexAttribI2uiv` is NULL");
	}
	fn glVertexAttribI3uiv(&self, _: GLuint, _: *const GLuint) {
		panic!("OpenGL function pointer of `glVertexAttribI3uiv` is NULL");
	}
	fn glVertexAttribI4uiv(&self, _: GLuint, _: *const GLuint) {
		panic!("OpenGL function pointer of `glVertexAttribI4uiv` is NULL");
	}
	fn glVertexAttribI4bv(&self, _: GLuint, _: *const GLbyte) {
		panic!("OpenGL function pointer of `glVertexAttribI4bv` is NULL");
	}
	fn glVertexAttribI4sv(&self, _: GLuint, _: *const GLshort) {
		panic!("OpenGL function pointer of `glVertexAttribI4sv` is NULL");
	}
	fn glVertexAttribI4ubv(&self, _: GLuint, _: *const GLubyte) {
		panic!("OpenGL function pointer of `glVertexAttribI4ubv` is NULL");
	}
	fn glVertexAttribI4usv(&self, _: GLuint, _: *const GLushort) {
		panic!("OpenGL function pointer of `glVertexAttribI4usv` is NULL");
	}
	fn glGetUniformuiv(&self, _: GLuint, _: GLint, _: *mut GLuint) {
		panic!("OpenGL function pointer of `glGetUniformuiv` is NULL");
	}
	fn glBindFragDataLocation(&self, _: GLuint, _: GLuint, _: *const GLchar) {
		panic!("OpenGL function pointer of `glBindFragDataLocation` is NULL");
	}
	fn glGetFragDataLocation(&self, _: GLuint, _: *const GLchar) -> GLint {
		panic!("OpenGL function pointer of `glGetFragDataLocation` is NULL");
	}
	fn glUniform1ui(&self, _: GLint, _: GLuint) {
		panic!("OpenGL function pointer of `glUniform1ui` is NULL");
	}
	fn glUniform2ui(&self, _: GLint, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glUniform2ui` is NULL");
	}
	fn glUniform3ui(&self, _: GLint, _: GLuint, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glUniform3ui` is NULL");
	}
	fn glUniform4ui(&self, _: GLint, _: GLuint, _: GLuint, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glUniform4ui` is NULL");
	}
	fn glUniform1uiv(&self, _: GLint, _: GLsizei, _: *const GLuint) {
		panic!("OpenGL function pointer of `glUniform1uiv` is NULL");
	}
	fn glUniform2uiv(&self, _: GLint, _: GLsizei, _: *const GLuint) {
		panic!("OpenGL function pointer of `glUniform2uiv` is NULL");
	}
	fn glUniform3uiv(&self, _: GLint, _: GLsizei, _: *const GLuint) {
		panic!("OpenGL function pointer of `glUniform3uiv` is NULL");
	}
	fn glUniform4uiv(&self, _: GLint, _: GLsizei, _: *const GLuint) {
		panic!("OpenGL function pointer of `glUniform4uiv` is NULL");
	}
	fn glTexParameterIiv(&self, _: GLenum, _: GLenum, _: *const GLint) {
		panic!("OpenGL function pointer of `glTexParameterIiv` is NULL");
	}
	fn glTexParameterIuiv(&self, _: GLenum, _: GLenum, _: *const GLuint) {
		panic!("OpenGL function pointer of `glTexParameterIuiv` is NULL");
	}
	fn glGetTexParameterIiv(&self, _: GLenum, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetTexParameterIiv` is NULL");
	}
	fn glGetTexParameterIuiv(&self, _: GLenum, _: GLenum, _: *mut GLuint) {
		panic!("OpenGL function pointer of `glGetTexParameterIuiv` is NULL");
	}
	fn glClearBufferiv(&self, _: GLenum, _: GLint, _: *const GLint) {
		panic!("OpenGL function pointer of `glClearBufferiv` is NULL");
	}
	fn glClearBufferuiv(&self, _: GLenum, _: GLint, _: *const GLuint) {
		panic!("OpenGL function pointer of `glClearBufferuiv` is NULL");
	}
	fn glClearBufferfv(&self, _: GLenum, _: GLint, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glClearBufferfv` is NULL");
	}
	fn glClearBufferfi(&self, _: GLenum, _: GLint, _: GLfloat, _: GLint) {
		panic!("OpenGL function pointer of `glClearBufferfi` is NULL");
	}
	fn glGetStringi(&self, _: GLenum, _: GLuint) -> *const GLubyte {
		panic!("OpenGL function pointer of `glGetStringi` is NULL");
	}
	fn glIsRenderbuffer(&self, _: GLuint) -> GLboolean {
		panic!("OpenGL function pointer of `glIsRenderbuffer` is NULL");
	}
	fn glBindRenderbuffer(&self, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glBindRenderbuffer` is NULL");
	}
	fn glDeleteRenderbuffers(&self, _: GLsizei, _: *const GLuint) {
		panic!("OpenGL function pointer of `glDeleteRenderbuffers` is NULL");
	}
	fn glGenRenderbuffers(&self, _: GLsizei, _: *mut GLuint) {
		panic!("OpenGL function pointer of `glGenRenderbuffers` is NULL");
	}
	fn glRenderbufferStorage(&self, _: GLenum, _: GLenum, _: GLsizei, _: GLsizei) {
		panic!("OpenGL function pointer of `glRenderbufferStorage` is NULL");
	}
	fn glGetRenderbufferParameteriv(&self, _: GLenum, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetRenderbufferParameteriv` is NULL");
	}
	fn glIsFramebuffer(&self, _: GLuint) -> GLboolean {
		panic!("OpenGL function pointer of `glIsFramebuffer` is NULL");
	}
	fn glBindFramebuffer(&self, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glBindFramebuffer` is NULL");
	}
	fn glDeleteFramebuffers(&self, _: GLsizei, _: *const GLuint) {
		panic!("OpenGL function pointer of `glDeleteFramebuffers` is NULL");
	}
	fn glGenFramebuffers(&self, _: GLsizei, _: *mut GLuint) {
		panic!("OpenGL function pointer of `glGenFramebuffers` is NULL");
	}
	fn glCheckFramebufferStatus(&self, _: GLenum) -> GLenum {
		panic!("OpenGL function pointer of `glCheckFramebufferStatus` is NULL");
	}
	fn glFramebufferTexture1D(&self, _: GLenum, _: GLenum, _: GLenum, _: GLuint, _: GLint) {
		panic!("OpenGL function pointer of `glFramebufferTexture1D` is NULL");
	}
	fn glFramebufferTexture2D(&self, _: GLenum, _: GLenum, _: GLenum, _: GLuint, _: GLint) {
		panic!("OpenGL function pointer of `glFramebufferTexture2D` is NULL");
	}
	fn glFramebufferTexture3D(&self, _: GLenum, _: GLenum, _: GLenum, _: GLuint, _: GLint, _: GLint) {
		panic!("OpenGL function pointer of `glFramebufferTexture3D` is NULL");
	}
	fn glFramebufferRenderbuffer(&self, _: GLenum, _: GLenum, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glFramebufferRenderbuffer` is NULL");
	}
	fn glGetFramebufferAttachmentParameteriv(&self, _: GLenum, _: GLenum, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetFramebufferAttachmentParameteriv` is NULL");
	}
	fn glGenerateMipmap(&self, _: GLenum) {
		panic!("OpenGL function pointer of `glGenerateMipmap` is NULL");
	}
	fn glBlitFramebuffer(&self, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLbitfield, _: GLenum) {
		panic!("OpenGL function pointer of `glBlitFramebuffer` is NULL");
	}
	fn glRenderbufferStorageMultisample(&self, _: GLenum, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei) {
		panic!("OpenGL function pointer of `glRenderbufferStorageMultisample` is NULL");
	}
	fn glFramebufferTextureLayer(&self, _: GLenum, _: GLenum, _: GLuint, _: GLint, _: GLint) {
		panic!("OpenGL function pointer of `glFramebufferTextureLayer` is NULL");
	}
	fn glMapBufferRange(&self, _: GLenum, _: GLintptr, _: GLsizeiptr, _: GLbitfield) -> *mut c_void {
		panic!("OpenGL function pointer of `glMapBufferRange` is NULL");
	}
	fn glFlushMappedBufferRange(&self, _: GLenum, _: GLintptr, _: GLsizeiptr) {
		panic!("OpenGL function pointer of `glFlushMappedBufferRange` is NULL");
	}
	fn glBindVertexArray(&self, _: GLuint) {
		panic!("OpenGL function pointer of `glBindVertexArray` is NULL");
	}
	fn glDeleteVertexArrays(&self, _: GLsizei, _: *const GLuint) {
		panic!("OpenGL function pointer of `glDeleteVertexArrays` is NULL");
	}
	fn glGenVertexArrays(&self, _: GLsizei, _: *mut GLuint) {
		panic!("OpenGL function pointer of `glGenVertexArrays` is NULL");
	}
	fn glIsVertexArray(&self, _: GLuint) -> GLboolean {
		panic!("OpenGL function pointer of `glIsVertexArray` is NULL");
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Version30 {
	available: bool,
	colormaski: PFNGLCOLORMASKIPROC,
	getbooleani_v: PFNGLGETBOOLEANI_VPROC,
	getintegeri_v: PFNGLGETINTEGERI_VPROC,
	enablei: PFNGLENABLEIPROC,
	disablei: PFNGLDISABLEIPROC,
	isenabledi: PFNGLISENABLEDIPROC,
	begintransformfeedback: PFNGLBEGINTRANSFORMFEEDBACKPROC,
	endtransformfeedback: PFNGLENDTRANSFORMFEEDBACKPROC,
	bindbufferrange: PFNGLBINDBUFFERRANGEPROC,
	bindbufferbase: PFNGLBINDBUFFERBASEPROC,
	transformfeedbackvaryings: PFNGLTRANSFORMFEEDBACKVARYINGSPROC,
	gettransformfeedbackvarying: PFNGLGETTRANSFORMFEEDBACKVARYINGPROC,
	clampcolor: PFNGLCLAMPCOLORPROC,
	beginconditionalrender: PFNGLBEGINCONDITIONALRENDERPROC,
	endconditionalrender: PFNGLENDCONDITIONALRENDERPROC,
	vertexattribipointer: PFNGLVERTEXATTRIBIPOINTERPROC,
	getvertexattribiiv: PFNGLGETVERTEXATTRIBIIVPROC,
	getvertexattribiuiv: PFNGLGETVERTEXATTRIBIUIVPROC,
	vertexattribi1i: PFNGLVERTEXATTRIBI1IPROC,
	vertexattribi2i: PFNGLVERTEXATTRIBI2IPROC,
	vertexattribi3i: PFNGLVERTEXATTRIBI3IPROC,
	vertexattribi4i: PFNGLVERTEXATTRIBI4IPROC,
	vertexattribi1ui: PFNGLVERTEXATTRIBI1UIPROC,
	vertexattribi2ui: PFNGLVERTEXATTRIBI2UIPROC,
	vertexattribi3ui: PFNGLVERTEXATTRIBI3UIPROC,
	vertexattribi4ui: PFNGLVERTEXATTRIBI4UIPROC,
	vertexattribi1iv: PFNGLVERTEXATTRIBI1IVPROC,
	vertexattribi2iv: PFNGLVERTEXATTRIBI2IVPROC,
	vertexattribi3iv: PFNGLVERTEXATTRIBI3IVPROC,
	vertexattribi4iv: PFNGLVERTEXATTRIBI4IVPROC,
	vertexattribi1uiv: PFNGLVERTEXATTRIBI1UIVPROC,
	vertexattribi2uiv: PFNGLVERTEXATTRIBI2UIVPROC,
	vertexattribi3uiv: PFNGLVERTEXATTRIBI3UIVPROC,
	vertexattribi4uiv: PFNGLVERTEXATTRIBI4UIVPROC,
	vertexattribi4bv: PFNGLVERTEXATTRIBI4BVPROC,
	vertexattribi4sv: PFNGLVERTEXATTRIBI4SVPROC,
	vertexattribi4ubv: PFNGLVERTEXATTRIBI4UBVPROC,
	vertexattribi4usv: PFNGLVERTEXATTRIBI4USVPROC,
	getuniformuiv: PFNGLGETUNIFORMUIVPROC,
	bindfragdatalocation: PFNGLBINDFRAGDATALOCATIONPROC,
	getfragdatalocation: PFNGLGETFRAGDATALOCATIONPROC,
	uniform1ui: PFNGLUNIFORM1UIPROC,
	uniform2ui: PFNGLUNIFORM2UIPROC,
	uniform3ui: PFNGLUNIFORM3UIPROC,
	uniform4ui: PFNGLUNIFORM4UIPROC,
	uniform1uiv: PFNGLUNIFORM1UIVPROC,
	uniform2uiv: PFNGLUNIFORM2UIVPROC,
	uniform3uiv: PFNGLUNIFORM3UIVPROC,
	uniform4uiv: PFNGLUNIFORM4UIVPROC,
	texparameteriiv: PFNGLTEXPARAMETERIIVPROC,
	texparameteriuiv: PFNGLTEXPARAMETERIUIVPROC,
	gettexparameteriiv: PFNGLGETTEXPARAMETERIIVPROC,
	gettexparameteriuiv: PFNGLGETTEXPARAMETERIUIVPROC,
	clearbufferiv: PFNGLCLEARBUFFERIVPROC,
	clearbufferuiv: PFNGLCLEARBUFFERUIVPROC,
	clearbufferfv: PFNGLCLEARBUFFERFVPROC,
	clearbufferfi: PFNGLCLEARBUFFERFIPROC,
	getstringi: PFNGLGETSTRINGIPROC,
	isrenderbuffer: PFNGLISRENDERBUFFERPROC,
	bindrenderbuffer: PFNGLBINDRENDERBUFFERPROC,
	deleterenderbuffers: PFNGLDELETERENDERBUFFERSPROC,
	genrenderbuffers: PFNGLGENRENDERBUFFERSPROC,
	renderbufferstorage: PFNGLRENDERBUFFERSTORAGEPROC,
	getrenderbufferparameteriv: PFNGLGETRENDERBUFFERPARAMETERIVPROC,
	isframebuffer: PFNGLISFRAMEBUFFERPROC,
	bindframebuffer: PFNGLBINDFRAMEBUFFERPROC,
	deleteframebuffers: PFNGLDELETEFRAMEBUFFERSPROC,
	genframebuffers: PFNGLGENFRAMEBUFFERSPROC,
	checkframebufferstatus: PFNGLCHECKFRAMEBUFFERSTATUSPROC,
	framebuffertexture1d: PFNGLFRAMEBUFFERTEXTURE1DPROC,
	framebuffertexture2d: PFNGLFRAMEBUFFERTEXTURE2DPROC,
	framebuffertexture3d: PFNGLFRAMEBUFFERTEXTURE3DPROC,
	framebufferrenderbuffer: PFNGLFRAMEBUFFERRENDERBUFFERPROC,
	getframebufferattachmentparameteriv: PFNGLGETFRAMEBUFFERATTACHMENTPARAMETERIVPROC,
	generatemipmap: PFNGLGENERATEMIPMAPPROC,
	blitframebuffer: PFNGLBLITFRAMEBUFFERPROC,
	renderbufferstoragemultisample: PFNGLRENDERBUFFERSTORAGEMULTISAMPLEPROC,
	framebuffertexturelayer: PFNGLFRAMEBUFFERTEXTURELAYERPROC,
	mapbufferrange: PFNGLMAPBUFFERRANGEPROC,
	flushmappedbufferrange: PFNGLFLUSHMAPPEDBUFFERRANGEPROC,
	bindvertexarray: PFNGLBINDVERTEXARRAYPROC,
	deletevertexarrays: PFNGLDELETEVERTEXARRAYSPROC,
	genvertexarrays: PFNGLGENVERTEXARRAYSPROC,
	isvertexarray: PFNGLISVERTEXARRAYPROC,
}

impl GL_3_0 for Version30 {
	#[inline(always)]
	fn glColorMaski(&self, index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean) {
		(self.colormaski)(index, r, g, b, a)
	}
	#[inline(always)]
	fn glGetBooleani_v(&self, target: GLenum, index: GLuint, data: *mut GLboolean) {
		(self.getbooleani_v)(target, index, data)
	}
	#[inline(always)]
	fn glGetIntegeri_v(&self, target: GLenum, index: GLuint, data: *mut GLint) {
		(self.getintegeri_v)(target, index, data)
	}
	#[inline(always)]
	fn glEnablei(&self, target: GLenum, index: GLuint) {
		(self.enablei)(target, index)
	}
	#[inline(always)]
	fn glDisablei(&self, target: GLenum, index: GLuint) {
		(self.disablei)(target, index)
	}
	#[inline(always)]
	fn glIsEnabledi(&self, target: GLenum, index: GLuint) -> GLboolean {
		(self.isenabledi)(target, index)
	}
	#[inline(always)]
	fn glBeginTransformFeedback(&self, primitiveMode: GLenum) {
		(self.begintransformfeedback)(primitiveMode)
	}
	#[inline(always)]
	fn glEndTransformFeedback(&self) {
		(self.endtransformfeedback)()
	}
	#[inline(always)]
	fn glBindBufferRange(&self, target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) {
		(self.bindbufferrange)(target, index, buffer, offset, size)
	}
	#[inline(always)]
	fn glBindBufferBase(&self, target: GLenum, index: GLuint, buffer: GLuint) {
		(self.bindbufferbase)(target, index, buffer)
	}
	#[inline(always)]
	fn glTransformFeedbackVaryings(&self, program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: GLenum) {
		(self.transformfeedbackvaryings)(program, count, varyings, bufferMode)
	}
	#[inline(always)]
	fn glGetTransformFeedbackVarying(&self, program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut GLenum, name: *mut GLchar) {
		(self.gettransformfeedbackvarying)(program, index, bufSize, length, size, type_, name)
	}
	#[inline(always)]
	fn glClampColor(&self, target: GLenum, clamp: GLenum) {
		(self.clampcolor)(target, clamp)
	}
	#[inline(always)]
	fn glBeginConditionalRender(&self, id: GLuint, mode: GLenum) {
		(self.beginconditionalrender)(id, mode)
	}
	#[inline(always)]
	fn glEndConditionalRender(&self) {
		(self.endconditionalrender)()
	}
	#[inline(always)]
	fn glVertexAttribIPointer(&self, index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void) {
		(self.vertexattribipointer)(index, size, type_, stride, pointer)
	}
	#[inline(always)]
	fn glGetVertexAttribIiv(&self, index: GLuint, pname: GLenum, params: *mut GLint) {
		(self.getvertexattribiiv)(index, pname, params)
	}
	#[inline(always)]
	fn glGetVertexAttribIuiv(&self, index: GLuint, pname: GLenum, params: *mut GLuint) {
		(self.getvertexattribiuiv)(index, pname, params)
	}
	#[inline(always)]
	fn glVertexAttribI1i(&self, index: GLuint, x: GLint) {
		(self.vertexattribi1i)(index, x)
	}
	#[inline(always)]
	fn glVertexAttribI2i(&self, index: GLuint, x: GLint, y: GLint) {
		(self.vertexattribi2i)(index, x, y)
	}
	#[inline(always)]
	fn glVertexAttribI3i(&self, index: GLuint, x: GLint, y: GLint, z: GLint) {
		(self.vertexattribi3i)(index, x, y, z)
	}
	#[inline(always)]
	fn glVertexAttribI4i(&self, index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) {
		(self.vertexattribi4i)(index, x, y, z, w)
	}
	#[inline(always)]
	fn glVertexAttribI1ui(&self, index: GLuint, x: GLuint) {
		(self.vertexattribi1ui)(index, x)
	}
	#[inline(always)]
	fn glVertexAttribI2ui(&self, index: GLuint, x: GLuint, y: GLuint) {
		(self.vertexattribi2ui)(index, x, y)
	}
	#[inline(always)]
	fn glVertexAttribI3ui(&self, index: GLuint, x: GLuint, y: GLuint, z: GLuint) {
		(self.vertexattribi3ui)(index, x, y, z)
	}
	#[inline(always)]
	fn glVertexAttribI4ui(&self, index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) {
		(self.vertexattribi4ui)(index, x, y, z, w)
	}
	#[inline(always)]
	fn glVertexAttribI1iv(&self, index: GLuint, v: *const GLint) {
		(self.vertexattribi1iv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribI2iv(&self, index: GLuint, v: *const GLint) {
		(self.vertexattribi2iv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribI3iv(&self, index: GLuint, v: *const GLint) {
		(self.vertexattribi3iv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribI4iv(&self, index: GLuint, v: *const GLint) {
		(self.vertexattribi4iv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribI1uiv(&self, index: GLuint, v: *const GLuint) {
		(self.vertexattribi1uiv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribI2uiv(&self, index: GLuint, v: *const GLuint) {
		(self.vertexattribi2uiv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribI3uiv(&self, index: GLuint, v: *const GLuint) {
		(self.vertexattribi3uiv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribI4uiv(&self, index: GLuint, v: *const GLuint) {
		(self.vertexattribi4uiv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribI4bv(&self, index: GLuint, v: *const GLbyte) {
		(self.vertexattribi4bv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribI4sv(&self, index: GLuint, v: *const GLshort) {
		(self.vertexattribi4sv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribI4ubv(&self, index: GLuint, v: *const GLubyte) {
		(self.vertexattribi4ubv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribI4usv(&self, index: GLuint, v: *const GLushort) {
		(self.vertexattribi4usv)(index, v)
	}
	#[inline(always)]
	fn glGetUniformuiv(&self, program: GLuint, location: GLint, params: *mut GLuint) {
		(self.getuniformuiv)(program, location, params)
	}
	#[inline(always)]
	fn glBindFragDataLocation(&self, program: GLuint, color: GLuint, name: *const GLchar) {
		(self.bindfragdatalocation)(program, color, name)
	}
	#[inline(always)]
	fn glGetFragDataLocation(&self, program: GLuint, name: *const GLchar) -> GLint {
		(self.getfragdatalocation)(program, name)
	}
	#[inline(always)]
	fn glUniform1ui(&self, location: GLint, v0: GLuint) {
		(self.uniform1ui)(location, v0)
	}
	#[inline(always)]
	fn glUniform2ui(&self, location: GLint, v0: GLuint, v1: GLuint) {
		(self.uniform2ui)(location, v0, v1)
	}
	#[inline(always)]
	fn glUniform3ui(&self, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) {
		(self.uniform3ui)(location, v0, v1, v2)
	}
	#[inline(always)]
	fn glUniform4ui(&self, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) {
		(self.uniform4ui)(location, v0, v1, v2, v3)
	}
	#[inline(always)]
	fn glUniform1uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
		(self.uniform1uiv)(location, count, value)
	}
	#[inline(always)]
	fn glUniform2uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
		(self.uniform2uiv)(location, count, value)
	}
	#[inline(always)]
	fn glUniform3uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
		(self.uniform3uiv)(location, count, value)
	}
	#[inline(always)]
	fn glUniform4uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
		(self.uniform4uiv)(location, count, value)
	}
	#[inline(always)]
	fn glTexParameterIiv(&self, target: GLenum, pname: GLenum, params: *const GLint) {
		(self.texparameteriiv)(target, pname, params)
	}
	#[inline(always)]
	fn glTexParameterIuiv(&self, target: GLenum, pname: GLenum, params: *const GLuint) {
		(self.texparameteriuiv)(target, pname, params)
	}
	#[inline(always)]
	fn glGetTexParameterIiv(&self, target: GLenum, pname: GLenum, params: *mut GLint) {
		(self.gettexparameteriiv)(target, pname, params)
	}
	#[inline(always)]
	fn glGetTexParameterIuiv(&self, target: GLenum, pname: GLenum, params: *mut GLuint) {
		(self.gettexparameteriuiv)(target, pname, params)
	}
	#[inline(always)]
	fn glClearBufferiv(&self, buffer: GLenum, drawbuffer: GLint, value: *const GLint) {
		(self.clearbufferiv)(buffer, drawbuffer, value)
	}
	#[inline(always)]
	fn glClearBufferuiv(&self, buffer: GLenum, drawbuffer: GLint, value: *const GLuint) {
		(self.clearbufferuiv)(buffer, drawbuffer, value)
	}
	#[inline(always)]
	fn glClearBufferfv(&self, buffer: GLenum, drawbuffer: GLint, value: *const GLfloat) {
		(self.clearbufferfv)(buffer, drawbuffer, value)
	}
	#[inline(always)]
	fn glClearBufferfi(&self, buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint) {
		(self.clearbufferfi)(buffer, drawbuffer, depth, stencil)
	}
	#[inline(always)]
	fn glGetStringi(&self, name: GLenum, index: GLuint) -> *const GLubyte {
		(self.getstringi)(name, index)
	}
	#[inline(always)]
	fn glIsRenderbuffer(&self, renderbuffer: GLuint) -> GLboolean {
		(self.isrenderbuffer)(renderbuffer)
	}
	#[inline(always)]
	fn glBindRenderbuffer(&self, target: GLenum, renderbuffer: GLuint) {
		(self.bindrenderbuffer)(target, renderbuffer)
	}
	#[inline(always)]
	fn glDeleteRenderbuffers(&self, n: GLsizei, renderbuffers: *const GLuint) {
		(self.deleterenderbuffers)(n, renderbuffers)
	}
	#[inline(always)]
	fn glGenRenderbuffers(&self, n: GLsizei, renderbuffers: *mut GLuint) {
		(self.genrenderbuffers)(n, renderbuffers)
	}
	#[inline(always)]
	fn glRenderbufferStorage(&self, target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei) {
		(self.renderbufferstorage)(target, internalformat, width, height)
	}
	#[inline(always)]
	fn glGetRenderbufferParameteriv(&self, target: GLenum, pname: GLenum, params: *mut GLint) {
		(self.getrenderbufferparameteriv)(target, pname, params)
	}
	#[inline(always)]
	fn glIsFramebuffer(&self, framebuffer: GLuint) -> GLboolean {
		(self.isframebuffer)(framebuffer)
	}
	#[inline(always)]
	fn glBindFramebuffer(&self, target: GLenum, framebuffer: GLuint) {
		(self.bindframebuffer)(target, framebuffer)
	}
	#[inline(always)]
	fn glDeleteFramebuffers(&self, n: GLsizei, framebuffers: *const GLuint) {
		(self.deleteframebuffers)(n, framebuffers)
	}
	#[inline(always)]
	fn glGenFramebuffers(&self, n: GLsizei, framebuffers: *mut GLuint) {
		(self.genframebuffers)(n, framebuffers)
	}
	#[inline(always)]
	fn glCheckFramebufferStatus(&self, target: GLenum) -> GLenum {
		(self.checkframebufferstatus)(target)
	}
	#[inline(always)]
	fn glFramebufferTexture1D(&self, target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) {
		(self.framebuffertexture1d)(target, attachment, textarget, texture, level)
	}
	#[inline(always)]
	fn glFramebufferTexture2D(&self, target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) {
		(self.framebuffertexture2d)(target, attachment, textarget, texture, level)
	}
	#[inline(always)]
	fn glFramebufferTexture3D(&self, target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint) {
		(self.framebuffertexture3d)(target, attachment, textarget, texture, level, zoffset)
	}
	#[inline(always)]
	fn glFramebufferRenderbuffer(&self, target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint) {
		(self.framebufferrenderbuffer)(target, attachment, renderbuffertarget, renderbuffer)
	}
	#[inline(always)]
	fn glGetFramebufferAttachmentParameteriv(&self, target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint) {
		(self.getframebufferattachmentparameteriv)(target, attachment, pname, params)
	}
	#[inline(always)]
	fn glGenerateMipmap(&self, target: GLenum) {
		(self.generatemipmap)(target)
	}
	#[inline(always)]
	fn glBlitFramebuffer(&self, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum) {
		(self.blitframebuffer)(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter)
	}
	#[inline(always)]
	fn glRenderbufferStorageMultisample(&self, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) {
		(self.renderbufferstoragemultisample)(target, samples, internalformat, width, height)
	}
	#[inline(always)]
	fn glFramebufferTextureLayer(&self, target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint) {
		(self.framebuffertexturelayer)(target, attachment, texture, level, layer)
	}
	#[inline(always)]
	fn glMapBufferRange(&self, target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut c_void {
		(self.mapbufferrange)(target, offset, length, access)
	}
	#[inline(always)]
	fn glFlushMappedBufferRange(&self, target: GLenum, offset: GLintptr, length: GLsizeiptr) {
		(self.flushmappedbufferrange)(target, offset, length)
	}
	#[inline(always)]
	fn glBindVertexArray(&self, array: GLuint) {
		(self.bindvertexarray)(array)
	}
	#[inline(always)]
	fn glDeleteVertexArrays(&self, n: GLsizei, arrays: *const GLuint) {
		(self.deletevertexarrays)(n, arrays)
	}
	#[inline(always)]
	fn glGenVertexArrays(&self, n: GLsizei, arrays: *mut GLuint) {
		(self.genvertexarrays)(n, arrays)
	}
	#[inline(always)]
	fn glIsVertexArray(&self, array: GLuint) -> GLboolean {
		(self.isvertexarray)(array)
	}
}

impl Version30 {
	pub fn new(base: impl GL_1_0, get_proc_address: &impl Fn(&'static str) -> *const c_void) -> Self {
		let (spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 30000 {
			return Self::default();
		}
		Self {
			available: true,
			colormaski: unsafe{transmute(get_proc_address("glColorMaski"))},
			getbooleani_v: unsafe{transmute(get_proc_address("glGetBooleani_v"))},
			getintegeri_v: unsafe{transmute(get_proc_address("glGetIntegeri_v"))},
			enablei: unsafe{transmute(get_proc_address("glEnablei"))},
			disablei: unsafe{transmute(get_proc_address("glDisablei"))},
			isenabledi: unsafe{transmute(get_proc_address("glIsEnabledi"))},
			begintransformfeedback: unsafe{transmute(get_proc_address("glBeginTransformFeedback"))},
			endtransformfeedback: unsafe{transmute(get_proc_address("glEndTransformFeedback"))},
			bindbufferrange: unsafe{transmute(get_proc_address("glBindBufferRange"))},
			bindbufferbase: unsafe{transmute(get_proc_address("glBindBufferBase"))},
			transformfeedbackvaryings: unsafe{transmute(get_proc_address("glTransformFeedbackVaryings"))},
			gettransformfeedbackvarying: unsafe{transmute(get_proc_address("glGetTransformFeedbackVarying"))},
			clampcolor: unsafe{transmute(get_proc_address("glClampColor"))},
			beginconditionalrender: unsafe{transmute(get_proc_address("glBeginConditionalRender"))},
			endconditionalrender: unsafe{transmute(get_proc_address("glEndConditionalRender"))},
			vertexattribipointer: unsafe{transmute(get_proc_address("glVertexAttribIPointer"))},
			getvertexattribiiv: unsafe{transmute(get_proc_address("glGetVertexAttribIiv"))},
			getvertexattribiuiv: unsafe{transmute(get_proc_address("glGetVertexAttribIuiv"))},
			vertexattribi1i: unsafe{transmute(get_proc_address("glVertexAttribI1i"))},
			vertexattribi2i: unsafe{transmute(get_proc_address("glVertexAttribI2i"))},
			vertexattribi3i: unsafe{transmute(get_proc_address("glVertexAttribI3i"))},
			vertexattribi4i: unsafe{transmute(get_proc_address("glVertexAttribI4i"))},
			vertexattribi1ui: unsafe{transmute(get_proc_address("glVertexAttribI1ui"))},
			vertexattribi2ui: unsafe{transmute(get_proc_address("glVertexAttribI2ui"))},
			vertexattribi3ui: unsafe{transmute(get_proc_address("glVertexAttribI3ui"))},
			vertexattribi4ui: unsafe{transmute(get_proc_address("glVertexAttribI4ui"))},
			vertexattribi1iv: unsafe{transmute(get_proc_address("glVertexAttribI1iv"))},
			vertexattribi2iv: unsafe{transmute(get_proc_address("glVertexAttribI2iv"))},
			vertexattribi3iv: unsafe{transmute(get_proc_address("glVertexAttribI3iv"))},
			vertexattribi4iv: unsafe{transmute(get_proc_address("glVertexAttribI4iv"))},
			vertexattribi1uiv: unsafe{transmute(get_proc_address("glVertexAttribI1uiv"))},
			vertexattribi2uiv: unsafe{transmute(get_proc_address("glVertexAttribI2uiv"))},
			vertexattribi3uiv: unsafe{transmute(get_proc_address("glVertexAttribI3uiv"))},
			vertexattribi4uiv: unsafe{transmute(get_proc_address("glVertexAttribI4uiv"))},
			vertexattribi4bv: unsafe{transmute(get_proc_address("glVertexAttribI4bv"))},
			vertexattribi4sv: unsafe{transmute(get_proc_address("glVertexAttribI4sv"))},
			vertexattribi4ubv: unsafe{transmute(get_proc_address("glVertexAttribI4ubv"))},
			vertexattribi4usv: unsafe{transmute(get_proc_address("glVertexAttribI4usv"))},
			getuniformuiv: unsafe{transmute(get_proc_address("glGetUniformuiv"))},
			bindfragdatalocation: unsafe{transmute(get_proc_address("glBindFragDataLocation"))},
			getfragdatalocation: unsafe{transmute(get_proc_address("glGetFragDataLocation"))},
			uniform1ui: unsafe{transmute(get_proc_address("glUniform1ui"))},
			uniform2ui: unsafe{transmute(get_proc_address("glUniform2ui"))},
			uniform3ui: unsafe{transmute(get_proc_address("glUniform3ui"))},
			uniform4ui: unsafe{transmute(get_proc_address("glUniform4ui"))},
			uniform1uiv: unsafe{transmute(get_proc_address("glUniform1uiv"))},
			uniform2uiv: unsafe{transmute(get_proc_address("glUniform2uiv"))},
			uniform3uiv: unsafe{transmute(get_proc_address("glUniform3uiv"))},
			uniform4uiv: unsafe{transmute(get_proc_address("glUniform4uiv"))},
			texparameteriiv: unsafe{transmute(get_proc_address("glTexParameterIiv"))},
			texparameteriuiv: unsafe{transmute(get_proc_address("glTexParameterIuiv"))},
			gettexparameteriiv: unsafe{transmute(get_proc_address("glGetTexParameterIiv"))},
			gettexparameteriuiv: unsafe{transmute(get_proc_address("glGetTexParameterIuiv"))},
			clearbufferiv: unsafe{transmute(get_proc_address("glClearBufferiv"))},
			clearbufferuiv: unsafe{transmute(get_proc_address("glClearBufferuiv"))},
			clearbufferfv: unsafe{transmute(get_proc_address("glClearBufferfv"))},
			clearbufferfi: unsafe{transmute(get_proc_address("glClearBufferfi"))},
			getstringi: unsafe{transmute(get_proc_address("glGetStringi"))},
			isrenderbuffer: unsafe{transmute(get_proc_address("glIsRenderbuffer"))},
			bindrenderbuffer: unsafe{transmute(get_proc_address("glBindRenderbuffer"))},
			deleterenderbuffers: unsafe{transmute(get_proc_address("glDeleteRenderbuffers"))},
			genrenderbuffers: unsafe{transmute(get_proc_address("glGenRenderbuffers"))},
			renderbufferstorage: unsafe{transmute(get_proc_address("glRenderbufferStorage"))},
			getrenderbufferparameteriv: unsafe{transmute(get_proc_address("glGetRenderbufferParameteriv"))},
			isframebuffer: unsafe{transmute(get_proc_address("glIsFramebuffer"))},
			bindframebuffer: unsafe{transmute(get_proc_address("glBindFramebuffer"))},
			deleteframebuffers: unsafe{transmute(get_proc_address("glDeleteFramebuffers"))},
			genframebuffers: unsafe{transmute(get_proc_address("glGenFramebuffers"))},
			checkframebufferstatus: unsafe{transmute(get_proc_address("glCheckFramebufferStatus"))},
			framebuffertexture1d: unsafe{transmute(get_proc_address("glFramebufferTexture1D"))},
			framebuffertexture2d: unsafe{transmute(get_proc_address("glFramebufferTexture2D"))},
			framebuffertexture3d: unsafe{transmute(get_proc_address("glFramebufferTexture3D"))},
			framebufferrenderbuffer: unsafe{transmute(get_proc_address("glFramebufferRenderbuffer"))},
			getframebufferattachmentparameteriv: unsafe{transmute(get_proc_address("glGetFramebufferAttachmentParameteriv"))},
			generatemipmap: unsafe{transmute(get_proc_address("glGenerateMipmap"))},
			blitframebuffer: unsafe{transmute(get_proc_address("glBlitFramebuffer"))},
			renderbufferstoragemultisample: unsafe{transmute(get_proc_address("glRenderbufferStorageMultisample"))},
			framebuffertexturelayer: unsafe{transmute(get_proc_address("glFramebufferTextureLayer"))},
			mapbufferrange: unsafe{transmute(get_proc_address("glMapBufferRange"))},
			flushmappedbufferrange: unsafe{transmute(get_proc_address("glFlushMappedBufferRange"))},
			bindvertexarray: unsafe{transmute(get_proc_address("glBindVertexArray"))},
			deletevertexarrays: unsafe{transmute(get_proc_address("glDeleteVertexArrays"))},
			genvertexarrays: unsafe{transmute(get_proc_address("glGenVertexArrays"))},
			isvertexarray: unsafe{transmute(get_proc_address("glIsVertexArray"))},
		}
	}
	#[inline(always)]
	pub fn get_available(&self) -> bool {
		self.available
	}
}

impl Default for Version30 {
	fn default() -> Self {
		Self {
			available: false,
			colormaski: null(),
			getbooleani_v: null(),
			getintegeri_v: null(),
			enablei: null(),
			disablei: null(),
			isenabledi: null(),
			begintransformfeedback: null(),
			endtransformfeedback: null(),
			bindbufferrange: null(),
			bindbufferbase: null(),
			transformfeedbackvaryings: null(),
			gettransformfeedbackvarying: null(),
			clampcolor: null(),
			beginconditionalrender: null(),
			endconditionalrender: null(),
			vertexattribipointer: null(),
			getvertexattribiiv: null(),
			getvertexattribiuiv: null(),
			vertexattribi1i: null(),
			vertexattribi2i: null(),
			vertexattribi3i: null(),
			vertexattribi4i: null(),
			vertexattribi1ui: null(),
			vertexattribi2ui: null(),
			vertexattribi3ui: null(),
			vertexattribi4ui: null(),
			vertexattribi1iv: null(),
			vertexattribi2iv: null(),
			vertexattribi3iv: null(),
			vertexattribi4iv: null(),
			vertexattribi1uiv: null(),
			vertexattribi2uiv: null(),
			vertexattribi3uiv: null(),
			vertexattribi4uiv: null(),
			vertexattribi4bv: null(),
			vertexattribi4sv: null(),
			vertexattribi4ubv: null(),
			vertexattribi4usv: null(),
			getuniformuiv: null(),
			bindfragdatalocation: null(),
			getfragdatalocation: null(),
			uniform1ui: null(),
			uniform2ui: null(),
			uniform3ui: null(),
			uniform4ui: null(),
			uniform1uiv: null(),
			uniform2uiv: null(),
			uniform3uiv: null(),
			uniform4uiv: null(),
			texparameteriiv: null(),
			texparameteriuiv: null(),
			gettexparameteriiv: null(),
			gettexparameteriuiv: null(),
			clearbufferiv: null(),
			clearbufferuiv: null(),
			clearbufferfv: null(),
			clearbufferfi: null(),
			getstringi: null(),
			isrenderbuffer: null(),
			bindrenderbuffer: null(),
			deleterenderbuffers: null(),
			genrenderbuffers: null(),
			renderbufferstorage: null(),
			getrenderbufferparameteriv: null(),
			isframebuffer: null(),
			bindframebuffer: null(),
			deleteframebuffers: null(),
			genframebuffers: null(),
			checkframebufferstatus: null(),
			framebuffertexture1d: null(),
			framebuffertexture2d: null(),
			framebuffertexture3d: null(),
			framebufferrenderbuffer: null(),
			getframebufferattachmentparameteriv: null(),
			generatemipmap: null(),
			blitframebuffer: null(),
			renderbufferstoragemultisample: null(),
			framebuffertexturelayer: null(),
			mapbufferrange: null(),
			flushmappedbufferrange: null(),
			bindvertexarray: null(),
			deletevertexarrays: null(),
			genvertexarrays: null(),
			isvertexarray: null(),
		}
	}
}

type PFNGLDRAWARRAYSINSTANCEDPROC = extern "system" fn(GLenum, GLint, GLsizei, GLsizei);
type PFNGLDRAWELEMENTSINSTANCEDPROC = extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei);
type PFNGLTEXBUFFERPROC = extern "system" fn(GLenum, GLenum, GLuint);
type PFNGLPRIMITIVERESTARTINDEXPROC = extern "system" fn(GLuint);
type PFNGLCOPYBUFFERSUBDATAPROC = extern "system" fn(GLenum, GLenum, GLintptr, GLintptr, GLsizeiptr);
type PFNGLGETUNIFORMINDICESPROC = extern "system" fn(GLuint, GLsizei, *const *const GLchar, *mut GLuint);
type PFNGLGETACTIVEUNIFORMSIVPROC = extern "system" fn(GLuint, GLsizei, *const GLuint, GLenum, *mut GLint);
type PFNGLGETACTIVEUNIFORMNAMEPROC = extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLchar);
type PFNGLGETUNIFORMBLOCKINDEXPROC = extern "system" fn(GLuint, *const GLchar) -> GLuint;
type PFNGLGETACTIVEUNIFORMBLOCKIVPROC = extern "system" fn(GLuint, GLuint, GLenum, *mut GLint);
type PFNGLGETACTIVEUNIFORMBLOCKNAMEPROC = extern "system" fn(GLuint, GLuint, GLsizei, *mut GLsizei, *mut GLchar);
type PFNGLUNIFORMBLOCKBINDINGPROC = extern "system" fn(GLuint, GLuint, GLuint);
const SAMPLER_2D_RECT: GLenum = 0x8B63;
const SAMPLER_2D_RECT_SHADOW: GLenum = 0x8B64;
const SAMPLER_BUFFER: GLenum = 0x8DC2;
const INT_SAMPLER_2D_RECT: GLenum = 0x8DCD;
const INT_SAMPLER_BUFFER: GLenum = 0x8DD0;
const UNSIGNED_INT_SAMPLER_2D_RECT: GLenum = 0x8DD5;
const UNSIGNED_INT_SAMPLER_BUFFER: GLenum = 0x8DD8;
const TEXTURE_BUFFER: GLenum = 0x8C2A;
const MAX_TEXTURE_BUFFER_SIZE: GLenum = 0x8C2B;
const TEXTURE_BINDING_BUFFER: GLenum = 0x8C2C;
const TEXTURE_BUFFER_DATA_STORE_BINDING: GLenum = 0x8C2D;
const TEXTURE_RECTANGLE: GLenum = 0x84F5;
const TEXTURE_BINDING_RECTANGLE: GLenum = 0x84F6;
const PROXY_TEXTURE_RECTANGLE: GLenum = 0x84F7;
const MAX_RECTANGLE_TEXTURE_SIZE: GLenum = 0x84F8;
const R8_SNORM: GLenum = 0x8F94;
const RG8_SNORM: GLenum = 0x8F95;
const RGB8_SNORM: GLenum = 0x8F96;
const RGBA8_SNORM: GLenum = 0x8F97;
const R16_SNORM: GLenum = 0x8F98;
const RG16_SNORM: GLenum = 0x8F99;
const RGB16_SNORM: GLenum = 0x8F9A;
const RGBA16_SNORM: GLenum = 0x8F9B;
const SIGNED_NORMALIZED: GLenum = 0x8F9C;
const PRIMITIVE_RESTART: GLenum = 0x8F9D;
const PRIMITIVE_RESTART_INDEX: GLenum = 0x8F9E;
const COPY_READ_BUFFER: GLenum = 0x8F36;
const COPY_WRITE_BUFFER: GLenum = 0x8F37;
const UNIFORM_BUFFER: GLenum = 0x8A11;
const UNIFORM_BUFFER_BINDING: GLenum = 0x8A28;
const UNIFORM_BUFFER_START: GLenum = 0x8A29;
const UNIFORM_BUFFER_SIZE: GLenum = 0x8A2A;
const MAX_VERTEX_UNIFORM_BLOCKS: GLenum = 0x8A2B;
const MAX_GEOMETRY_UNIFORM_BLOCKS: GLenum = 0x8A2C;
const MAX_FRAGMENT_UNIFORM_BLOCKS: GLenum = 0x8A2D;
const MAX_COMBINED_UNIFORM_BLOCKS: GLenum = 0x8A2E;
const MAX_UNIFORM_BUFFER_BINDINGS: GLenum = 0x8A2F;
const MAX_UNIFORM_BLOCK_SIZE: GLenum = 0x8A30;
const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8A31;
const MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 0x8A32;
const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8A33;
const UNIFORM_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x8A34;
const ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: GLenum = 0x8A35;
const ACTIVE_UNIFORM_BLOCKS: GLenum = 0x8A36;
const UNIFORM_TYPE: GLenum = 0x8A37;
const UNIFORM_SIZE: GLenum = 0x8A38;
const UNIFORM_NAME_LENGTH: GLenum = 0x8A39;
const UNIFORM_BLOCK_INDEX: GLenum = 0x8A3A;
const UNIFORM_OFFSET: GLenum = 0x8A3B;
const UNIFORM_ARRAY_STRIDE: GLenum = 0x8A3C;
const UNIFORM_MATRIX_STRIDE: GLenum = 0x8A3D;
const UNIFORM_IS_ROW_MAJOR: GLenum = 0x8A3E;
const UNIFORM_BLOCK_BINDING: GLenum = 0x8A3F;
const UNIFORM_BLOCK_DATA_SIZE: GLenum = 0x8A40;
const UNIFORM_BLOCK_NAME_LENGTH: GLenum = 0x8A41;
const UNIFORM_BLOCK_ACTIVE_UNIFORMS: GLenum = 0x8A42;
const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: GLenum = 0x8A43;
const UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x8A44;
const UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x8A45;
const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x8A46;
const INVALID_INDEX: GLuint = 0xFFFFFFFFu32;

pub trait GL_3_1 {
	fn glDrawArraysInstanced(&self, _: GLenum, _: GLint, _: GLsizei, _: GLsizei) {
		panic!("OpenGL function pointer of `glDrawArraysInstanced` is NULL");
	}
	fn glDrawElementsInstanced(&self, _: GLenum, _: GLsizei, _: GLenum, _: *const c_void, _: GLsizei) {
		panic!("OpenGL function pointer of `glDrawElementsInstanced` is NULL");
	}
	fn glTexBuffer(&self, _: GLenum, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glTexBuffer` is NULL");
	}
	fn glPrimitiveRestartIndex(&self, _: GLuint) {
		panic!("OpenGL function pointer of `glPrimitiveRestartIndex` is NULL");
	}
	fn glCopyBufferSubData(&self, _: GLenum, _: GLenum, _: GLintptr, _: GLintptr, _: GLsizeiptr) {
		panic!("OpenGL function pointer of `glCopyBufferSubData` is NULL");
	}
	fn glGetUniformIndices(&self, _: GLuint, _: GLsizei, _: *const *const GLchar, _: *mut GLuint) {
		panic!("OpenGL function pointer of `glGetUniformIndices` is NULL");
	}
	fn glGetActiveUniformsiv(&self, _: GLuint, _: GLsizei, _: *const GLuint, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetActiveUniformsiv` is NULL");
	}
	fn glGetActiveUniformName(&self, _: GLuint, _: GLuint, _: GLsizei, _: *mut GLsizei, _: *mut GLchar) {
		panic!("OpenGL function pointer of `glGetActiveUniformName` is NULL");
	}
	fn glGetUniformBlockIndex(&self, _: GLuint, _: *const GLchar) -> GLuint {
		panic!("OpenGL function pointer of `glGetUniformBlockIndex` is NULL");
	}
	fn glGetActiveUniformBlockiv(&self, _: GLuint, _: GLuint, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetActiveUniformBlockiv` is NULL");
	}
	fn glGetActiveUniformBlockName(&self, _: GLuint, _: GLuint, _: GLsizei, _: *mut GLsizei, _: *mut GLchar) {
		panic!("OpenGL function pointer of `glGetActiveUniformBlockName` is NULL");
	}
	fn glUniformBlockBinding(&self, _: GLuint, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glUniformBlockBinding` is NULL");
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Version31 {
	available: bool,
	drawarraysinstanced: PFNGLDRAWARRAYSINSTANCEDPROC,
	drawelementsinstanced: PFNGLDRAWELEMENTSINSTANCEDPROC,
	texbuffer: PFNGLTEXBUFFERPROC,
	primitiverestartindex: PFNGLPRIMITIVERESTARTINDEXPROC,
	copybuffersubdata: PFNGLCOPYBUFFERSUBDATAPROC,
	getuniformindices: PFNGLGETUNIFORMINDICESPROC,
	getactiveuniformsiv: PFNGLGETACTIVEUNIFORMSIVPROC,
	getactiveuniformname: PFNGLGETACTIVEUNIFORMNAMEPROC,
	getuniformblockindex: PFNGLGETUNIFORMBLOCKINDEXPROC,
	getactiveuniformblockiv: PFNGLGETACTIVEUNIFORMBLOCKIVPROC,
	getactiveuniformblockname: PFNGLGETACTIVEUNIFORMBLOCKNAMEPROC,
	uniformblockbinding: PFNGLUNIFORMBLOCKBINDINGPROC,
}

impl GL_3_1 for Version31 {
	#[inline(always)]
	fn glDrawArraysInstanced(&self, mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei) {
		(self.drawarraysinstanced)(mode, first, count, instancecount)
	}
	#[inline(always)]
	fn glDrawElementsInstanced(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei) {
		(self.drawelementsinstanced)(mode, count, type_, indices, instancecount)
	}
	#[inline(always)]
	fn glTexBuffer(&self, target: GLenum, internalformat: GLenum, buffer: GLuint) {
		(self.texbuffer)(target, internalformat, buffer)
	}
	#[inline(always)]
	fn glPrimitiveRestartIndex(&self, index: GLuint) {
		(self.primitiverestartindex)(index)
	}
	#[inline(always)]
	fn glCopyBufferSubData(&self, readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr) {
		(self.copybuffersubdata)(readTarget, writeTarget, readOffset, writeOffset, size)
	}
	#[inline(always)]
	fn glGetUniformIndices(&self, program: GLuint, uniformCount: GLsizei, uniformNames: *const *const GLchar, uniformIndices: *mut GLuint) {
		(self.getuniformindices)(program, uniformCount, uniformNames, uniformIndices)
	}
	#[inline(always)]
	fn glGetActiveUniformsiv(&self, program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint, pname: GLenum, params: *mut GLint) {
		(self.getactiveuniformsiv)(program, uniformCount, uniformIndices, pname, params)
	}
	#[inline(always)]
	fn glGetActiveUniformName(&self, program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar) {
		(self.getactiveuniformname)(program, uniformIndex, bufSize, length, uniformName)
	}
	#[inline(always)]
	fn glGetUniformBlockIndex(&self, program: GLuint, uniformBlockName: *const GLchar) -> GLuint {
		(self.getuniformblockindex)(program, uniformBlockName)
	}
	#[inline(always)]
	fn glGetActiveUniformBlockiv(&self, program: GLuint, uniformBlockIndex: GLuint, pname: GLenum, params: *mut GLint) {
		(self.getactiveuniformblockiv)(program, uniformBlockIndex, pname, params)
	}
	#[inline(always)]
	fn glGetActiveUniformBlockName(&self, program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar) {
		(self.getactiveuniformblockname)(program, uniformBlockIndex, bufSize, length, uniformBlockName)
	}
	#[inline(always)]
	fn glUniformBlockBinding(&self, program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint) {
		(self.uniformblockbinding)(program, uniformBlockIndex, uniformBlockBinding)
	}
}

impl Version31 {
	pub fn new(base: impl GL_1_0, get_proc_address: &impl Fn(&'static str) -> *const c_void) -> Self {
		let (spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 30100 {
			return Self::default();
		}
		Self {
			available: true,
			drawarraysinstanced: unsafe{transmute(get_proc_address("glDrawArraysInstanced"))},
			drawelementsinstanced: unsafe{transmute(get_proc_address("glDrawElementsInstanced"))},
			texbuffer: unsafe{transmute(get_proc_address("glTexBuffer"))},
			primitiverestartindex: unsafe{transmute(get_proc_address("glPrimitiveRestartIndex"))},
			copybuffersubdata: unsafe{transmute(get_proc_address("glCopyBufferSubData"))},
			getuniformindices: unsafe{transmute(get_proc_address("glGetUniformIndices"))},
			getactiveuniformsiv: unsafe{transmute(get_proc_address("glGetActiveUniformsiv"))},
			getactiveuniformname: unsafe{transmute(get_proc_address("glGetActiveUniformName"))},
			getuniformblockindex: unsafe{transmute(get_proc_address("glGetUniformBlockIndex"))},
			getactiveuniformblockiv: unsafe{transmute(get_proc_address("glGetActiveUniformBlockiv"))},
			getactiveuniformblockname: unsafe{transmute(get_proc_address("glGetActiveUniformBlockName"))},
			uniformblockbinding: unsafe{transmute(get_proc_address("glUniformBlockBinding"))},
		}
	}
	#[inline(always)]
	pub fn get_available(&self) -> bool {
		self.available
	}
}

impl Default for Version31 {
	fn default() -> Self {
		Self {
			available: false,
			drawarraysinstanced: null(),
			drawelementsinstanced: null(),
			texbuffer: null(),
			primitiverestartindex: null(),
			copybuffersubdata: null(),
			getuniformindices: null(),
			getactiveuniformsiv: null(),
			getactiveuniformname: null(),
			getuniformblockindex: null(),
			getactiveuniformblockiv: null(),
			getactiveuniformblockname: null(),
			uniformblockbinding: null(),
		}
	}
}

type GLsync = *mut c_void;
type GLuint64 = khronos_uint64_t;
type GLint64 = khronos_int64_t;
type PFNGLDRAWELEMENTSBASEVERTEXPROC = extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLint);
type PFNGLDRAWRANGEELEMENTSBASEVERTEXPROC = extern "system" fn(GLenum, GLuint, GLuint, GLsizei, GLenum, *const c_void, GLint);
type PFNGLDRAWELEMENTSINSTANCEDBASEVERTEXPROC = extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei, GLint);
type PFNGLMULTIDRAWELEMENTSBASEVERTEXPROC = extern "system" fn(GLenum, *const GLsizei, GLenum, *const *const c_void, GLsizei, *const GLint);
type PFNGLPROVOKINGVERTEXPROC = extern "system" fn(GLenum);
type PFNGLFENCESYNCPROC = extern "system" fn(GLenum, GLbitfield) -> GLsync;
type PFNGLISSYNCPROC = extern "system" fn(GLsync) -> GLboolean;
type PFNGLDELETESYNCPROC = extern "system" fn(GLsync);
type PFNGLCLIENTWAITSYNCPROC = extern "system" fn(GLsync, GLbitfield, GLuint64) -> GLenum;
type PFNGLWAITSYNCPROC = extern "system" fn(GLsync, GLbitfield, GLuint64);
type PFNGLGETINTEGER64VPROC = extern "system" fn(GLenum, *mut GLint64);
type PFNGLGETSYNCIVPROC = extern "system" fn(GLsync, GLenum, GLsizei, *mut GLsizei, *mut GLint);
type PFNGLGETINTEGER64I_VPROC = extern "system" fn(GLenum, GLuint, *mut GLint64);
type PFNGLGETBUFFERPARAMETERI64VPROC = extern "system" fn(GLenum, GLenum, *mut GLint64);
type PFNGLFRAMEBUFFERTEXTUREPROC = extern "system" fn(GLenum, GLenum, GLuint, GLint);
type PFNGLTEXIMAGE2DMULTISAMPLEPROC = extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLboolean);
type PFNGLTEXIMAGE3DMULTISAMPLEPROC = extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei, GLboolean);
type PFNGLGETMULTISAMPLEFVPROC = extern "system" fn(GLenum, GLuint, *mut GLfloat);
type PFNGLSAMPLEMASKIPROC = extern "system" fn(GLuint, GLbitfield);
const CONTEXT_CORE_PROFILE_BIT: GLbitfield = 0x00000001;
const CONTEXT_COMPATIBILITY_PROFILE_BIT: GLbitfield = 0x00000002;
const LINES_ADJACENCY: GLenum = 0x000A;
const LINE_STRIP_ADJACENCY: GLenum = 0x000B;
const TRIANGLES_ADJACENCY: GLenum = 0x000C;
const TRIANGLE_STRIP_ADJACENCY: GLenum = 0x000D;
const PROGRAM_POINT_SIZE: GLenum = 0x8642;
const MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: GLenum = 0x8C29;
const FRAMEBUFFER_ATTACHMENT_LAYERED: GLenum = 0x8DA7;
const FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: GLenum = 0x8DA8;
const GEOMETRY_SHADER: GLenum = 0x8DD9;
const GEOMETRY_VERTICES_OUT: GLenum = 0x8916;
const GEOMETRY_INPUT_TYPE: GLenum = 0x8917;
const GEOMETRY_OUTPUT_TYPE: GLenum = 0x8918;
const MAX_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 0x8DDF;
const MAX_GEOMETRY_OUTPUT_VERTICES: GLenum = 0x8DE0;
const MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: GLenum = 0x8DE1;
const MAX_VERTEX_OUTPUT_COMPONENTS: GLenum = 0x9122;
const MAX_GEOMETRY_INPUT_COMPONENTS: GLenum = 0x9123;
const MAX_GEOMETRY_OUTPUT_COMPONENTS: GLenum = 0x9124;
const MAX_FRAGMENT_INPUT_COMPONENTS: GLenum = 0x9125;
const CONTEXT_PROFILE_MASK: GLenum = 0x9126;
const DEPTH_CLAMP: GLenum = 0x864F;
const QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: GLenum = 0x8E4C;
const FIRST_VERTEX_CONVENTION: GLenum = 0x8E4D;
const LAST_VERTEX_CONVENTION: GLenum = 0x8E4E;
const PROVOKING_VERTEX: GLenum = 0x8E4F;
const TEXTURE_CUBE_MAP_SEAMLESS: GLenum = 0x884F;
const MAX_SERVER_WAIT_TIMEOUT: GLenum = 0x9111;
const OBJECT_TYPE: GLenum = 0x9112;
const SYNC_CONDITION: GLenum = 0x9113;
const SYNC_STATUS: GLenum = 0x9114;
const SYNC_FLAGS: GLenum = 0x9115;
const SYNC_FENCE: GLenum = 0x9116;
const SYNC_GPU_COMMANDS_COMPLETE: GLenum = 0x9117;
const UNSIGNALED: GLenum = 0x9118;
const SIGNALED: GLenum = 0x9119;
const ALREADY_SIGNALED: GLenum = 0x911A;
const TIMEOUT_EXPIRED: GLenum = 0x911B;
const CONDITION_SATISFIED: GLenum = 0x911C;
const WAIT_FAILED: GLenum = 0x911D;
const TIMEOUT_IGNORED: GLuint64 = 0xFFFFFFFFFFFFFFFFu64;
const SYNC_FLUSH_COMMANDS_BIT: GLbitfield = 0x00000001;
const SAMPLE_POSITION: GLenum = 0x8E50;
const SAMPLE_MASK: GLenum = 0x8E51;
const SAMPLE_MASK_VALUE: GLenum = 0x8E52;
const MAX_SAMPLE_MASK_WORDS: GLenum = 0x8E59;
const TEXTURE_2D_MULTISAMPLE: GLenum = 0x9100;
const PROXY_TEXTURE_2D_MULTISAMPLE: GLenum = 0x9101;
const TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9102;
const PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9103;
const TEXTURE_BINDING_2D_MULTISAMPLE: GLenum = 0x9104;
const TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: GLenum = 0x9105;
const TEXTURE_SAMPLES: GLenum = 0x9106;
const TEXTURE_FIXED_SAMPLE_LOCATIONS: GLenum = 0x9107;
const SAMPLER_2D_MULTISAMPLE: GLenum = 0x9108;
const INT_SAMPLER_2D_MULTISAMPLE: GLenum = 0x9109;
const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: GLenum = 0x910A;
const SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910B;
const INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910C;
const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910D;
const MAX_COLOR_TEXTURE_SAMPLES: GLenum = 0x910E;
const MAX_DEPTH_TEXTURE_SAMPLES: GLenum = 0x910F;
const MAX_INTEGER_SAMPLES: GLenum = 0x9110;

pub trait GL_3_2 {
	fn glDrawElementsBaseVertex(&self, _: GLenum, _: GLsizei, _: GLenum, _: *const c_void, _: GLint) {
		panic!("OpenGL function pointer of `glDrawElementsBaseVertex` is NULL");
	}
	fn glDrawRangeElementsBaseVertex(&self, _: GLenum, _: GLuint, _: GLuint, _: GLsizei, _: GLenum, _: *const c_void, _: GLint) {
		panic!("OpenGL function pointer of `glDrawRangeElementsBaseVertex` is NULL");
	}
	fn glDrawElementsInstancedBaseVertex(&self, _: GLenum, _: GLsizei, _: GLenum, _: *const c_void, _: GLsizei, _: GLint) {
		panic!("OpenGL function pointer of `glDrawElementsInstancedBaseVertex` is NULL");
	}
	fn glMultiDrawElementsBaseVertex(&self, _: GLenum, _: *const GLsizei, _: GLenum, _: *const *const c_void, _: GLsizei, _: *const GLint) {
		panic!("OpenGL function pointer of `glMultiDrawElementsBaseVertex` is NULL");
	}
	fn glProvokingVertex(&self, _: GLenum) {
		panic!("OpenGL function pointer of `glProvokingVertex` is NULL");
	}
	fn glFenceSync(&self, _: GLenum, _: GLbitfield) -> GLsync {
		panic!("OpenGL function pointer of `glFenceSync` is NULL");
	}
	fn glIsSync(&self, _: GLsync) -> GLboolean {
		panic!("OpenGL function pointer of `glIsSync` is NULL");
	}
	fn glDeleteSync(&self, _: GLsync) {
		panic!("OpenGL function pointer of `glDeleteSync` is NULL");
	}
	fn glClientWaitSync(&self, _: GLsync, _: GLbitfield, _: GLuint64) -> GLenum {
		panic!("OpenGL function pointer of `glClientWaitSync` is NULL");
	}
	fn glWaitSync(&self, _: GLsync, _: GLbitfield, _: GLuint64) {
		panic!("OpenGL function pointer of `glWaitSync` is NULL");
	}
	fn glGetInteger64v(&self, _: GLenum, _: *mut GLint64) {
		panic!("OpenGL function pointer of `glGetInteger64v` is NULL");
	}
	fn glGetSynciv(&self, _: GLsync, _: GLenum, _: GLsizei, _: *mut GLsizei, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetSynciv` is NULL");
	}
	fn glGetInteger64i_v(&self, _: GLenum, _: GLuint, _: *mut GLint64) {
		panic!("OpenGL function pointer of `glGetInteger64i_v` is NULL");
	}
	fn glGetBufferParameteri64v(&self, _: GLenum, _: GLenum, _: *mut GLint64) {
		panic!("OpenGL function pointer of `glGetBufferParameteri64v` is NULL");
	}
	fn glFramebufferTexture(&self, _: GLenum, _: GLenum, _: GLuint, _: GLint) {
		panic!("OpenGL function pointer of `glFramebufferTexture` is NULL");
	}
	fn glTexImage2DMultisample(&self, _: GLenum, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei, _: GLboolean) {
		panic!("OpenGL function pointer of `glTexImage2DMultisample` is NULL");
	}
	fn glTexImage3DMultisample(&self, _: GLenum, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei, _: GLsizei, _: GLboolean) {
		panic!("OpenGL function pointer of `glTexImage3DMultisample` is NULL");
	}
	fn glGetMultisamplefv(&self, _: GLenum, _: GLuint, _: *mut GLfloat) {
		panic!("OpenGL function pointer of `glGetMultisamplefv` is NULL");
	}
	fn glSampleMaski(&self, _: GLuint, _: GLbitfield) {
		panic!("OpenGL function pointer of `glSampleMaski` is NULL");
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Version32 {
	available: bool,
	drawelementsbasevertex: PFNGLDRAWELEMENTSBASEVERTEXPROC,
	drawrangeelementsbasevertex: PFNGLDRAWRANGEELEMENTSBASEVERTEXPROC,
	drawelementsinstancedbasevertex: PFNGLDRAWELEMENTSINSTANCEDBASEVERTEXPROC,
	multidrawelementsbasevertex: PFNGLMULTIDRAWELEMENTSBASEVERTEXPROC,
	provokingvertex: PFNGLPROVOKINGVERTEXPROC,
	fencesync: PFNGLFENCESYNCPROC,
	issync: PFNGLISSYNCPROC,
	deletesync: PFNGLDELETESYNCPROC,
	clientwaitsync: PFNGLCLIENTWAITSYNCPROC,
	waitsync: PFNGLWAITSYNCPROC,
	getinteger64v: PFNGLGETINTEGER64VPROC,
	getsynciv: PFNGLGETSYNCIVPROC,
	getinteger64i_v: PFNGLGETINTEGER64I_VPROC,
	getbufferparameteri64v: PFNGLGETBUFFERPARAMETERI64VPROC,
	framebuffertexture: PFNGLFRAMEBUFFERTEXTUREPROC,
	teximage2dmultisample: PFNGLTEXIMAGE2DMULTISAMPLEPROC,
	teximage3dmultisample: PFNGLTEXIMAGE3DMULTISAMPLEPROC,
	getmultisamplefv: PFNGLGETMULTISAMPLEFVPROC,
	samplemaski: PFNGLSAMPLEMASKIPROC,
}

impl GL_3_2 for Version32 {
	#[inline(always)]
	fn glDrawElementsBaseVertex(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, basevertex: GLint) {
		(self.drawelementsbasevertex)(mode, count, type_, indices, basevertex)
	}
	#[inline(always)]
	fn glDrawRangeElementsBaseVertex(&self, mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const c_void, basevertex: GLint) {
		(self.drawrangeelementsbasevertex)(mode, start, end, count, type_, indices, basevertex)
	}
	#[inline(always)]
	fn glDrawElementsInstancedBaseVertex(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei, basevertex: GLint) {
		(self.drawelementsinstancedbasevertex)(mode, count, type_, indices, instancecount, basevertex)
	}
	#[inline(always)]
	fn glMultiDrawElementsBaseVertex(&self, mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const c_void, drawcount: GLsizei, basevertex: *const GLint) {
		(self.multidrawelementsbasevertex)(mode, count, type_, indices, drawcount, basevertex)
	}
	#[inline(always)]
	fn glProvokingVertex(&self, mode: GLenum) {
		(self.provokingvertex)(mode)
	}
	#[inline(always)]
	fn glFenceSync(&self, condition: GLenum, flags: GLbitfield) -> GLsync {
		(self.fencesync)(condition, flags)
	}
	#[inline(always)]
	fn glIsSync(&self, sync: GLsync) -> GLboolean {
		(self.issync)(sync)
	}
	#[inline(always)]
	fn glDeleteSync(&self, sync: GLsync) {
		(self.deletesync)(sync)
	}
	#[inline(always)]
	fn glClientWaitSync(&self, sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum {
		(self.clientwaitsync)(sync, flags, timeout)
	}
	#[inline(always)]
	fn glWaitSync(&self, sync: GLsync, flags: GLbitfield, timeout: GLuint64) {
		(self.waitsync)(sync, flags, timeout)
	}
	#[inline(always)]
	fn glGetInteger64v(&self, pname: GLenum, data: *mut GLint64) {
		(self.getinteger64v)(pname, data)
	}
	#[inline(always)]
	fn glGetSynciv(&self, sync: GLsync, pname: GLenum, count: GLsizei, length: *mut GLsizei, values: *mut GLint) {
		(self.getsynciv)(sync, pname, count, length, values)
	}
	#[inline(always)]
	fn glGetInteger64i_v(&self, target: GLenum, index: GLuint, data: *mut GLint64) {
		(self.getinteger64i_v)(target, index, data)
	}
	#[inline(always)]
	fn glGetBufferParameteri64v(&self, target: GLenum, pname: GLenum, params: *mut GLint64) {
		(self.getbufferparameteri64v)(target, pname, params)
	}
	#[inline(always)]
	fn glFramebufferTexture(&self, target: GLenum, attachment: GLenum, texture: GLuint, level: GLint) {
		(self.framebuffertexture)(target, attachment, texture, level)
	}
	#[inline(always)]
	fn glTexImage2DMultisample(&self, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) {
		(self.teximage2dmultisample)(target, samples, internalformat, width, height, fixedsamplelocations)
	}
	#[inline(always)]
	fn glTexImage3DMultisample(&self, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) {
		(self.teximage3dmultisample)(target, samples, internalformat, width, height, depth, fixedsamplelocations)
	}
	#[inline(always)]
	fn glGetMultisamplefv(&self, pname: GLenum, index: GLuint, val: *mut GLfloat) {
		(self.getmultisamplefv)(pname, index, val)
	}
	#[inline(always)]
	fn glSampleMaski(&self, maskNumber: GLuint, mask: GLbitfield) {
		(self.samplemaski)(maskNumber, mask)
	}
}

impl Version32 {
	pub fn new(base: impl GL_1_0, get_proc_address: &impl Fn(&'static str) -> *const c_void) -> Self {
		let (spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 30200 {
			return Self::default();
		}
		Self {
			available: true,
			drawelementsbasevertex: unsafe{transmute(get_proc_address("glDrawElementsBaseVertex"))},
			drawrangeelementsbasevertex: unsafe{transmute(get_proc_address("glDrawRangeElementsBaseVertex"))},
			drawelementsinstancedbasevertex: unsafe{transmute(get_proc_address("glDrawElementsInstancedBaseVertex"))},
			multidrawelementsbasevertex: unsafe{transmute(get_proc_address("glMultiDrawElementsBaseVertex"))},
			provokingvertex: unsafe{transmute(get_proc_address("glProvokingVertex"))},
			fencesync: unsafe{transmute(get_proc_address("glFenceSync"))},
			issync: unsafe{transmute(get_proc_address("glIsSync"))},
			deletesync: unsafe{transmute(get_proc_address("glDeleteSync"))},
			clientwaitsync: unsafe{transmute(get_proc_address("glClientWaitSync"))},
			waitsync: unsafe{transmute(get_proc_address("glWaitSync"))},
			getinteger64v: unsafe{transmute(get_proc_address("glGetInteger64v"))},
			getsynciv: unsafe{transmute(get_proc_address("glGetSynciv"))},
			getinteger64i_v: unsafe{transmute(get_proc_address("glGetInteger64i_v"))},
			getbufferparameteri64v: unsafe{transmute(get_proc_address("glGetBufferParameteri64v"))},
			framebuffertexture: unsafe{transmute(get_proc_address("glFramebufferTexture"))},
			teximage2dmultisample: unsafe{transmute(get_proc_address("glTexImage2DMultisample"))},
			teximage3dmultisample: unsafe{transmute(get_proc_address("glTexImage3DMultisample"))},
			getmultisamplefv: unsafe{transmute(get_proc_address("glGetMultisamplefv"))},
			samplemaski: unsafe{transmute(get_proc_address("glSampleMaski"))},
		}
	}
	#[inline(always)]
	pub fn get_available(&self) -> bool {
		self.available
	}
}

impl Default for Version32 {
	fn default() -> Self {
		Self {
			available: false,
			drawelementsbasevertex: null(),
			drawrangeelementsbasevertex: null(),
			drawelementsinstancedbasevertex: null(),
			multidrawelementsbasevertex: null(),
			provokingvertex: null(),
			fencesync: null(),
			issync: null(),
			deletesync: null(),
			clientwaitsync: null(),
			waitsync: null(),
			getinteger64v: null(),
			getsynciv: null(),
			getinteger64i_v: null(),
			getbufferparameteri64v: null(),
			framebuffertexture: null(),
			teximage2dmultisample: null(),
			teximage3dmultisample: null(),
			getmultisamplefv: null(),
			samplemaski: null(),
		}
	}
}

type PFNGLBINDFRAGDATALOCATIONINDEXEDPROC = extern "system" fn(GLuint, GLuint, GLuint, *const GLchar);
type PFNGLGETFRAGDATAINDEXPROC = extern "system" fn(GLuint, *const GLchar) -> GLint;
type PFNGLGENSAMPLERSPROC = extern "system" fn(GLsizei, *mut GLuint);
type PFNGLDELETESAMPLERSPROC = extern "system" fn(GLsizei, *const GLuint);
type PFNGLISSAMPLERPROC = extern "system" fn(GLuint) -> GLboolean;
type PFNGLBINDSAMPLERPROC = extern "system" fn(GLuint, GLuint);
type PFNGLSAMPLERPARAMETERIPROC = extern "system" fn(GLuint, GLenum, GLint);
type PFNGLSAMPLERPARAMETERIVPROC = extern "system" fn(GLuint, GLenum, *const GLint);
type PFNGLSAMPLERPARAMETERFPROC = extern "system" fn(GLuint, GLenum, GLfloat);
type PFNGLSAMPLERPARAMETERFVPROC = extern "system" fn(GLuint, GLenum, *const GLfloat);
type PFNGLSAMPLERPARAMETERIIVPROC = extern "system" fn(GLuint, GLenum, *const GLint);
type PFNGLSAMPLERPARAMETERIUIVPROC = extern "system" fn(GLuint, GLenum, *const GLuint);
type PFNGLGETSAMPLERPARAMETERIVPROC = extern "system" fn(GLuint, GLenum, *mut GLint);
type PFNGLGETSAMPLERPARAMETERIIVPROC = extern "system" fn(GLuint, GLenum, *mut GLint);
type PFNGLGETSAMPLERPARAMETERFVPROC = extern "system" fn(GLuint, GLenum, *mut GLfloat);
type PFNGLGETSAMPLERPARAMETERIUIVPROC = extern "system" fn(GLuint, GLenum, *mut GLuint);
type PFNGLQUERYCOUNTERPROC = extern "system" fn(GLuint, GLenum);
type PFNGLGETQUERYOBJECTI64VPROC = extern "system" fn(GLuint, GLenum, *mut GLint64);
type PFNGLGETQUERYOBJECTUI64VPROC = extern "system" fn(GLuint, GLenum, *mut GLuint64);
type PFNGLVERTEXATTRIBDIVISORPROC = extern "system" fn(GLuint, GLuint);
type PFNGLVERTEXATTRIBP1UIPROC = extern "system" fn(GLuint, GLenum, GLboolean, GLuint);
type PFNGLVERTEXATTRIBP1UIVPROC = extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint);
type PFNGLVERTEXATTRIBP2UIPROC = extern "system" fn(GLuint, GLenum, GLboolean, GLuint);
type PFNGLVERTEXATTRIBP2UIVPROC = extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint);
type PFNGLVERTEXATTRIBP3UIPROC = extern "system" fn(GLuint, GLenum, GLboolean, GLuint);
type PFNGLVERTEXATTRIBP3UIVPROC = extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint);
type PFNGLVERTEXATTRIBP4UIPROC = extern "system" fn(GLuint, GLenum, GLboolean, GLuint);
type PFNGLVERTEXATTRIBP4UIVPROC = extern "system" fn(GLuint, GLenum, GLboolean, *const GLuint);
type PFNGLVERTEXP2UIPROC = extern "system" fn(GLenum, GLuint);
type PFNGLVERTEXP2UIVPROC = extern "system" fn(GLenum, *const GLuint);
type PFNGLVERTEXP3UIPROC = extern "system" fn(GLenum, GLuint);
type PFNGLVERTEXP3UIVPROC = extern "system" fn(GLenum, *const GLuint);
type PFNGLVERTEXP4UIPROC = extern "system" fn(GLenum, GLuint);
type PFNGLVERTEXP4UIVPROC = extern "system" fn(GLenum, *const GLuint);
type PFNGLTEXCOORDP1UIPROC = extern "system" fn(GLenum, GLuint);
type PFNGLTEXCOORDP1UIVPROC = extern "system" fn(GLenum, *const GLuint);
type PFNGLTEXCOORDP2UIPROC = extern "system" fn(GLenum, GLuint);
type PFNGLTEXCOORDP2UIVPROC = extern "system" fn(GLenum, *const GLuint);
type PFNGLTEXCOORDP3UIPROC = extern "system" fn(GLenum, GLuint);
type PFNGLTEXCOORDP3UIVPROC = extern "system" fn(GLenum, *const GLuint);
type PFNGLTEXCOORDP4UIPROC = extern "system" fn(GLenum, GLuint);
type PFNGLTEXCOORDP4UIVPROC = extern "system" fn(GLenum, *const GLuint);
type PFNGLMULTITEXCOORDP1UIPROC = extern "system" fn(GLenum, GLenum, GLuint);
type PFNGLMULTITEXCOORDP1UIVPROC = extern "system" fn(GLenum, GLenum, *const GLuint);
type PFNGLMULTITEXCOORDP2UIPROC = extern "system" fn(GLenum, GLenum, GLuint);
type PFNGLMULTITEXCOORDP2UIVPROC = extern "system" fn(GLenum, GLenum, *const GLuint);
type PFNGLMULTITEXCOORDP3UIPROC = extern "system" fn(GLenum, GLenum, GLuint);
type PFNGLMULTITEXCOORDP3UIVPROC = extern "system" fn(GLenum, GLenum, *const GLuint);
type PFNGLMULTITEXCOORDP4UIPROC = extern "system" fn(GLenum, GLenum, GLuint);
type PFNGLMULTITEXCOORDP4UIVPROC = extern "system" fn(GLenum, GLenum, *const GLuint);
type PFNGLNORMALP3UIPROC = extern "system" fn(GLenum, GLuint);
type PFNGLNORMALP3UIVPROC = extern "system" fn(GLenum, *const GLuint);
type PFNGLCOLORP3UIPROC = extern "system" fn(GLenum, GLuint);
type PFNGLCOLORP3UIVPROC = extern "system" fn(GLenum, *const GLuint);
type PFNGLCOLORP4UIPROC = extern "system" fn(GLenum, GLuint);
type PFNGLCOLORP4UIVPROC = extern "system" fn(GLenum, *const GLuint);
type PFNGLSECONDARYCOLORP3UIPROC = extern "system" fn(GLenum, GLuint);
type PFNGLSECONDARYCOLORP3UIVPROC = extern "system" fn(GLenum, *const GLuint);
const VERTEX_ATTRIB_ARRAY_DIVISOR: GLenum = 0x88FE;
const SRC1_COLOR: GLenum = 0x88F9;
const ONE_MINUS_SRC1_COLOR: GLenum = 0x88FA;
const ONE_MINUS_SRC1_ALPHA: GLenum = 0x88FB;
const MAX_DUAL_SOURCE_DRAW_BUFFERS: GLenum = 0x88FC;
const ANY_SAMPLES_PASSED: GLenum = 0x8C2F;
const SAMPLER_BINDING: GLenum = 0x8919;
const RGB10_A2UI: GLenum = 0x906F;
const TEXTURE_SWIZZLE_R: GLenum = 0x8E42;
const TEXTURE_SWIZZLE_G: GLenum = 0x8E43;
const TEXTURE_SWIZZLE_B: GLenum = 0x8E44;
const TEXTURE_SWIZZLE_A: GLenum = 0x8E45;
const TEXTURE_SWIZZLE_RGBA: GLenum = 0x8E46;
const TIME_ELAPSED: GLenum = 0x88BF;
const TIMESTAMP: GLenum = 0x8E28;
const INT_2_10_10_10_REV: GLenum = 0x8D9F;

pub trait GL_3_3 {
	fn glBindFragDataLocationIndexed(&self, _: GLuint, _: GLuint, _: GLuint, _: *const GLchar) {
		panic!("OpenGL function pointer of `glBindFragDataLocationIndexed` is NULL");
	}
	fn glGetFragDataIndex(&self, _: GLuint, _: *const GLchar) -> GLint {
		panic!("OpenGL function pointer of `glGetFragDataIndex` is NULL");
	}
	fn glGenSamplers(&self, _: GLsizei, _: *mut GLuint) {
		panic!("OpenGL function pointer of `glGenSamplers` is NULL");
	}
	fn glDeleteSamplers(&self, _: GLsizei, _: *const GLuint) {
		panic!("OpenGL function pointer of `glDeleteSamplers` is NULL");
	}
	fn glIsSampler(&self, _: GLuint) -> GLboolean {
		panic!("OpenGL function pointer of `glIsSampler` is NULL");
	}
	fn glBindSampler(&self, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glBindSampler` is NULL");
	}
	fn glSamplerParameteri(&self, _: GLuint, _: GLenum, _: GLint) {
		panic!("OpenGL function pointer of `glSamplerParameteri` is NULL");
	}
	fn glSamplerParameteriv(&self, _: GLuint, _: GLenum, _: *const GLint) {
		panic!("OpenGL function pointer of `glSamplerParameteriv` is NULL");
	}
	fn glSamplerParameterf(&self, _: GLuint, _: GLenum, _: GLfloat) {
		panic!("OpenGL function pointer of `glSamplerParameterf` is NULL");
	}
	fn glSamplerParameterfv(&self, _: GLuint, _: GLenum, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glSamplerParameterfv` is NULL");
	}
	fn glSamplerParameterIiv(&self, _: GLuint, _: GLenum, _: *const GLint) {
		panic!("OpenGL function pointer of `glSamplerParameterIiv` is NULL");
	}
	fn glSamplerParameterIuiv(&self, _: GLuint, _: GLenum, _: *const GLuint) {
		panic!("OpenGL function pointer of `glSamplerParameterIuiv` is NULL");
	}
	fn glGetSamplerParameteriv(&self, _: GLuint, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetSamplerParameteriv` is NULL");
	}
	fn glGetSamplerParameterIiv(&self, _: GLuint, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetSamplerParameterIiv` is NULL");
	}
	fn glGetSamplerParameterfv(&self, _: GLuint, _: GLenum, _: *mut GLfloat) {
		panic!("OpenGL function pointer of `glGetSamplerParameterfv` is NULL");
	}
	fn glGetSamplerParameterIuiv(&self, _: GLuint, _: GLenum, _: *mut GLuint) {
		panic!("OpenGL function pointer of `glGetSamplerParameterIuiv` is NULL");
	}
	fn glQueryCounter(&self, _: GLuint, _: GLenum) {
		panic!("OpenGL function pointer of `glQueryCounter` is NULL");
	}
	fn glGetQueryObjecti64v(&self, _: GLuint, _: GLenum, _: *mut GLint64) {
		panic!("OpenGL function pointer of `glGetQueryObjecti64v` is NULL");
	}
	fn glGetQueryObjectui64v(&self, _: GLuint, _: GLenum, _: *mut GLuint64) {
		panic!("OpenGL function pointer of `glGetQueryObjectui64v` is NULL");
	}
	fn glVertexAttribDivisor(&self, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glVertexAttribDivisor` is NULL");
	}
	fn glVertexAttribP1ui(&self, _: GLuint, _: GLenum, _: GLboolean, _: GLuint) {
		panic!("OpenGL function pointer of `glVertexAttribP1ui` is NULL");
	}
	fn glVertexAttribP1uiv(&self, _: GLuint, _: GLenum, _: GLboolean, _: *const GLuint) {
		panic!("OpenGL function pointer of `glVertexAttribP1uiv` is NULL");
	}
	fn glVertexAttribP2ui(&self, _: GLuint, _: GLenum, _: GLboolean, _: GLuint) {
		panic!("OpenGL function pointer of `glVertexAttribP2ui` is NULL");
	}
	fn glVertexAttribP2uiv(&self, _: GLuint, _: GLenum, _: GLboolean, _: *const GLuint) {
		panic!("OpenGL function pointer of `glVertexAttribP2uiv` is NULL");
	}
	fn glVertexAttribP3ui(&self, _: GLuint, _: GLenum, _: GLboolean, _: GLuint) {
		panic!("OpenGL function pointer of `glVertexAttribP3ui` is NULL");
	}
	fn glVertexAttribP3uiv(&self, _: GLuint, _: GLenum, _: GLboolean, _: *const GLuint) {
		panic!("OpenGL function pointer of `glVertexAttribP3uiv` is NULL");
	}
	fn glVertexAttribP4ui(&self, _: GLuint, _: GLenum, _: GLboolean, _: GLuint) {
		panic!("OpenGL function pointer of `glVertexAttribP4ui` is NULL");
	}
	fn glVertexAttribP4uiv(&self, _: GLuint, _: GLenum, _: GLboolean, _: *const GLuint) {
		panic!("OpenGL function pointer of `glVertexAttribP4uiv` is NULL");
	}
	fn glVertexP2ui(&self, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glVertexP2ui` is NULL");
	}
	fn glVertexP2uiv(&self, _: GLenum, _: *const GLuint) {
		panic!("OpenGL function pointer of `glVertexP2uiv` is NULL");
	}
	fn glVertexP3ui(&self, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glVertexP3ui` is NULL");
	}
	fn glVertexP3uiv(&self, _: GLenum, _: *const GLuint) {
		panic!("OpenGL function pointer of `glVertexP3uiv` is NULL");
	}
	fn glVertexP4ui(&self, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glVertexP4ui` is NULL");
	}
	fn glVertexP4uiv(&self, _: GLenum, _: *const GLuint) {
		panic!("OpenGL function pointer of `glVertexP4uiv` is NULL");
	}
	fn glTexCoordP1ui(&self, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glTexCoordP1ui` is NULL");
	}
	fn glTexCoordP1uiv(&self, _: GLenum, _: *const GLuint) {
		panic!("OpenGL function pointer of `glTexCoordP1uiv` is NULL");
	}
	fn glTexCoordP2ui(&self, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glTexCoordP2ui` is NULL");
	}
	fn glTexCoordP2uiv(&self, _: GLenum, _: *const GLuint) {
		panic!("OpenGL function pointer of `glTexCoordP2uiv` is NULL");
	}
	fn glTexCoordP3ui(&self, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glTexCoordP3ui` is NULL");
	}
	fn glTexCoordP3uiv(&self, _: GLenum, _: *const GLuint) {
		panic!("OpenGL function pointer of `glTexCoordP3uiv` is NULL");
	}
	fn glTexCoordP4ui(&self, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glTexCoordP4ui` is NULL");
	}
	fn glTexCoordP4uiv(&self, _: GLenum, _: *const GLuint) {
		panic!("OpenGL function pointer of `glTexCoordP4uiv` is NULL");
	}
	fn glMultiTexCoordP1ui(&self, _: GLenum, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glMultiTexCoordP1ui` is NULL");
	}
	fn glMultiTexCoordP1uiv(&self, _: GLenum, _: GLenum, _: *const GLuint) {
		panic!("OpenGL function pointer of `glMultiTexCoordP1uiv` is NULL");
	}
	fn glMultiTexCoordP2ui(&self, _: GLenum, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glMultiTexCoordP2ui` is NULL");
	}
	fn glMultiTexCoordP2uiv(&self, _: GLenum, _: GLenum, _: *const GLuint) {
		panic!("OpenGL function pointer of `glMultiTexCoordP2uiv` is NULL");
	}
	fn glMultiTexCoordP3ui(&self, _: GLenum, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glMultiTexCoordP3ui` is NULL");
	}
	fn glMultiTexCoordP3uiv(&self, _: GLenum, _: GLenum, _: *const GLuint) {
		panic!("OpenGL function pointer of `glMultiTexCoordP3uiv` is NULL");
	}
	fn glMultiTexCoordP4ui(&self, _: GLenum, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glMultiTexCoordP4ui` is NULL");
	}
	fn glMultiTexCoordP4uiv(&self, _: GLenum, _: GLenum, _: *const GLuint) {
		panic!("OpenGL function pointer of `glMultiTexCoordP4uiv` is NULL");
	}
	fn glNormalP3ui(&self, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glNormalP3ui` is NULL");
	}
	fn glNormalP3uiv(&self, _: GLenum, _: *const GLuint) {
		panic!("OpenGL function pointer of `glNormalP3uiv` is NULL");
	}
	fn glColorP3ui(&self, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glColorP3ui` is NULL");
	}
	fn glColorP3uiv(&self, _: GLenum, _: *const GLuint) {
		panic!("OpenGL function pointer of `glColorP3uiv` is NULL");
	}
	fn glColorP4ui(&self, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glColorP4ui` is NULL");
	}
	fn glColorP4uiv(&self, _: GLenum, _: *const GLuint) {
		panic!("OpenGL function pointer of `glColorP4uiv` is NULL");
	}
	fn glSecondaryColorP3ui(&self, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glSecondaryColorP3ui` is NULL");
	}
	fn glSecondaryColorP3uiv(&self, _: GLenum, _: *const GLuint) {
		panic!("OpenGL function pointer of `glSecondaryColorP3uiv` is NULL");
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Version33 {
	available: bool,
	bindfragdatalocationindexed: PFNGLBINDFRAGDATALOCATIONINDEXEDPROC,
	getfragdataindex: PFNGLGETFRAGDATAINDEXPROC,
	gensamplers: PFNGLGENSAMPLERSPROC,
	deletesamplers: PFNGLDELETESAMPLERSPROC,
	issampler: PFNGLISSAMPLERPROC,
	bindsampler: PFNGLBINDSAMPLERPROC,
	samplerparameteri: PFNGLSAMPLERPARAMETERIPROC,
	samplerparameteriv: PFNGLSAMPLERPARAMETERIVPROC,
	samplerparameterf: PFNGLSAMPLERPARAMETERFPROC,
	samplerparameterfv: PFNGLSAMPLERPARAMETERFVPROC,
	samplerparameteriiv: PFNGLSAMPLERPARAMETERIIVPROC,
	samplerparameteriuiv: PFNGLSAMPLERPARAMETERIUIVPROC,
	getsamplerparameteriv: PFNGLGETSAMPLERPARAMETERIVPROC,
	getsamplerparameteriiv: PFNGLGETSAMPLERPARAMETERIIVPROC,
	getsamplerparameterfv: PFNGLGETSAMPLERPARAMETERFVPROC,
	getsamplerparameteriuiv: PFNGLGETSAMPLERPARAMETERIUIVPROC,
	querycounter: PFNGLQUERYCOUNTERPROC,
	getqueryobjecti64v: PFNGLGETQUERYOBJECTI64VPROC,
	getqueryobjectui64v: PFNGLGETQUERYOBJECTUI64VPROC,
	vertexattribdivisor: PFNGLVERTEXATTRIBDIVISORPROC,
	vertexattribp1ui: PFNGLVERTEXATTRIBP1UIPROC,
	vertexattribp1uiv: PFNGLVERTEXATTRIBP1UIVPROC,
	vertexattribp2ui: PFNGLVERTEXATTRIBP2UIPROC,
	vertexattribp2uiv: PFNGLVERTEXATTRIBP2UIVPROC,
	vertexattribp3ui: PFNGLVERTEXATTRIBP3UIPROC,
	vertexattribp3uiv: PFNGLVERTEXATTRIBP3UIVPROC,
	vertexattribp4ui: PFNGLVERTEXATTRIBP4UIPROC,
	vertexattribp4uiv: PFNGLVERTEXATTRIBP4UIVPROC,
	vertexp2ui: PFNGLVERTEXP2UIPROC,
	vertexp2uiv: PFNGLVERTEXP2UIVPROC,
	vertexp3ui: PFNGLVERTEXP3UIPROC,
	vertexp3uiv: PFNGLVERTEXP3UIVPROC,
	vertexp4ui: PFNGLVERTEXP4UIPROC,
	vertexp4uiv: PFNGLVERTEXP4UIVPROC,
	texcoordp1ui: PFNGLTEXCOORDP1UIPROC,
	texcoordp1uiv: PFNGLTEXCOORDP1UIVPROC,
	texcoordp2ui: PFNGLTEXCOORDP2UIPROC,
	texcoordp2uiv: PFNGLTEXCOORDP2UIVPROC,
	texcoordp3ui: PFNGLTEXCOORDP3UIPROC,
	texcoordp3uiv: PFNGLTEXCOORDP3UIVPROC,
	texcoordp4ui: PFNGLTEXCOORDP4UIPROC,
	texcoordp4uiv: PFNGLTEXCOORDP4UIVPROC,
	multitexcoordp1ui: PFNGLMULTITEXCOORDP1UIPROC,
	multitexcoordp1uiv: PFNGLMULTITEXCOORDP1UIVPROC,
	multitexcoordp2ui: PFNGLMULTITEXCOORDP2UIPROC,
	multitexcoordp2uiv: PFNGLMULTITEXCOORDP2UIVPROC,
	multitexcoordp3ui: PFNGLMULTITEXCOORDP3UIPROC,
	multitexcoordp3uiv: PFNGLMULTITEXCOORDP3UIVPROC,
	multitexcoordp4ui: PFNGLMULTITEXCOORDP4UIPROC,
	multitexcoordp4uiv: PFNGLMULTITEXCOORDP4UIVPROC,
	normalp3ui: PFNGLNORMALP3UIPROC,
	normalp3uiv: PFNGLNORMALP3UIVPROC,
	colorp3ui: PFNGLCOLORP3UIPROC,
	colorp3uiv: PFNGLCOLORP3UIVPROC,
	colorp4ui: PFNGLCOLORP4UIPROC,
	colorp4uiv: PFNGLCOLORP4UIVPROC,
	secondarycolorp3ui: PFNGLSECONDARYCOLORP3UIPROC,
	secondarycolorp3uiv: PFNGLSECONDARYCOLORP3UIVPROC,
}

impl GL_3_3 for Version33 {
	#[inline(always)]
	fn glBindFragDataLocationIndexed(&self, program: GLuint, colorNumber: GLuint, index: GLuint, name: *const GLchar) {
		(self.bindfragdatalocationindexed)(program, colorNumber, index, name)
	}
	#[inline(always)]
	fn glGetFragDataIndex(&self, program: GLuint, name: *const GLchar) -> GLint {
		(self.getfragdataindex)(program, name)
	}
	#[inline(always)]
	fn glGenSamplers(&self, count: GLsizei, samplers: *mut GLuint) {
		(self.gensamplers)(count, samplers)
	}
	#[inline(always)]
	fn glDeleteSamplers(&self, count: GLsizei, samplers: *const GLuint) {
		(self.deletesamplers)(count, samplers)
	}
	#[inline(always)]
	fn glIsSampler(&self, sampler: GLuint) -> GLboolean {
		(self.issampler)(sampler)
	}
	#[inline(always)]
	fn glBindSampler(&self, unit: GLuint, sampler: GLuint) {
		(self.bindsampler)(unit, sampler)
	}
	#[inline(always)]
	fn glSamplerParameteri(&self, sampler: GLuint, pname: GLenum, param: GLint) {
		(self.samplerparameteri)(sampler, pname, param)
	}
	#[inline(always)]
	fn glSamplerParameteriv(&self, sampler: GLuint, pname: GLenum, param: *const GLint) {
		(self.samplerparameteriv)(sampler, pname, param)
	}
	#[inline(always)]
	fn glSamplerParameterf(&self, sampler: GLuint, pname: GLenum, param: GLfloat) {
		(self.samplerparameterf)(sampler, pname, param)
	}
	#[inline(always)]
	fn glSamplerParameterfv(&self, sampler: GLuint, pname: GLenum, param: *const GLfloat) {
		(self.samplerparameterfv)(sampler, pname, param)
	}
	#[inline(always)]
	fn glSamplerParameterIiv(&self, sampler: GLuint, pname: GLenum, param: *const GLint) {
		(self.samplerparameteriiv)(sampler, pname, param)
	}
	#[inline(always)]
	fn glSamplerParameterIuiv(&self, sampler: GLuint, pname: GLenum, param: *const GLuint) {
		(self.samplerparameteriuiv)(sampler, pname, param)
	}
	#[inline(always)]
	fn glGetSamplerParameteriv(&self, sampler: GLuint, pname: GLenum, params: *mut GLint) {
		(self.getsamplerparameteriv)(sampler, pname, params)
	}
	#[inline(always)]
	fn glGetSamplerParameterIiv(&self, sampler: GLuint, pname: GLenum, params: *mut GLint) {
		(self.getsamplerparameteriiv)(sampler, pname, params)
	}
	#[inline(always)]
	fn glGetSamplerParameterfv(&self, sampler: GLuint, pname: GLenum, params: *mut GLfloat) {
		(self.getsamplerparameterfv)(sampler, pname, params)
	}
	#[inline(always)]
	fn glGetSamplerParameterIuiv(&self, sampler: GLuint, pname: GLenum, params: *mut GLuint) {
		(self.getsamplerparameteriuiv)(sampler, pname, params)
	}
	#[inline(always)]
	fn glQueryCounter(&self, id: GLuint, target: GLenum) {
		(self.querycounter)(id, target)
	}
	#[inline(always)]
	fn glGetQueryObjecti64v(&self, id: GLuint, pname: GLenum, params: *mut GLint64) {
		(self.getqueryobjecti64v)(id, pname, params)
	}
	#[inline(always)]
	fn glGetQueryObjectui64v(&self, id: GLuint, pname: GLenum, params: *mut GLuint64) {
		(self.getqueryobjectui64v)(id, pname, params)
	}
	#[inline(always)]
	fn glVertexAttribDivisor(&self, index: GLuint, divisor: GLuint) {
		(self.vertexattribdivisor)(index, divisor)
	}
	#[inline(always)]
	fn glVertexAttribP1ui(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) {
		(self.vertexattribp1ui)(index, type_, normalized, value)
	}
	#[inline(always)]
	fn glVertexAttribP1uiv(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) {
		(self.vertexattribp1uiv)(index, type_, normalized, value)
	}
	#[inline(always)]
	fn glVertexAttribP2ui(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) {
		(self.vertexattribp2ui)(index, type_, normalized, value)
	}
	#[inline(always)]
	fn glVertexAttribP2uiv(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) {
		(self.vertexattribp2uiv)(index, type_, normalized, value)
	}
	#[inline(always)]
	fn glVertexAttribP3ui(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) {
		(self.vertexattribp3ui)(index, type_, normalized, value)
	}
	#[inline(always)]
	fn glVertexAttribP3uiv(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) {
		(self.vertexattribp3uiv)(index, type_, normalized, value)
	}
	#[inline(always)]
	fn glVertexAttribP4ui(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) {
		(self.vertexattribp4ui)(index, type_, normalized, value)
	}
	#[inline(always)]
	fn glVertexAttribP4uiv(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) {
		(self.vertexattribp4uiv)(index, type_, normalized, value)
	}
	#[inline(always)]
	fn glVertexP2ui(&self, type_: GLenum, value: GLuint) {
		(self.vertexp2ui)(type_, value)
	}
	#[inline(always)]
	fn glVertexP2uiv(&self, type_: GLenum, value: *const GLuint) {
		(self.vertexp2uiv)(type_, value)
	}
	#[inline(always)]
	fn glVertexP3ui(&self, type_: GLenum, value: GLuint) {
		(self.vertexp3ui)(type_, value)
	}
	#[inline(always)]
	fn glVertexP3uiv(&self, type_: GLenum, value: *const GLuint) {
		(self.vertexp3uiv)(type_, value)
	}
	#[inline(always)]
	fn glVertexP4ui(&self, type_: GLenum, value: GLuint) {
		(self.vertexp4ui)(type_, value)
	}
	#[inline(always)]
	fn glVertexP4uiv(&self, type_: GLenum, value: *const GLuint) {
		(self.vertexp4uiv)(type_, value)
	}
	#[inline(always)]
	fn glTexCoordP1ui(&self, type_: GLenum, coords: GLuint) {
		(self.texcoordp1ui)(type_, coords)
	}
	#[inline(always)]
	fn glTexCoordP1uiv(&self, type_: GLenum, coords: *const GLuint) {
		(self.texcoordp1uiv)(type_, coords)
	}
	#[inline(always)]
	fn glTexCoordP2ui(&self, type_: GLenum, coords: GLuint) {
		(self.texcoordp2ui)(type_, coords)
	}
	#[inline(always)]
	fn glTexCoordP2uiv(&self, type_: GLenum, coords: *const GLuint) {
		(self.texcoordp2uiv)(type_, coords)
	}
	#[inline(always)]
	fn glTexCoordP3ui(&self, type_: GLenum, coords: GLuint) {
		(self.texcoordp3ui)(type_, coords)
	}
	#[inline(always)]
	fn glTexCoordP3uiv(&self, type_: GLenum, coords: *const GLuint) {
		(self.texcoordp3uiv)(type_, coords)
	}
	#[inline(always)]
	fn glTexCoordP4ui(&self, type_: GLenum, coords: GLuint) {
		(self.texcoordp4ui)(type_, coords)
	}
	#[inline(always)]
	fn glTexCoordP4uiv(&self, type_: GLenum, coords: *const GLuint) {
		(self.texcoordp4uiv)(type_, coords)
	}
	#[inline(always)]
	fn glMultiTexCoordP1ui(&self, texture: GLenum, type_: GLenum, coords: GLuint) {
		(self.multitexcoordp1ui)(texture, type_, coords)
	}
	#[inline(always)]
	fn glMultiTexCoordP1uiv(&self, texture: GLenum, type_: GLenum, coords: *const GLuint) {
		(self.multitexcoordp1uiv)(texture, type_, coords)
	}
	#[inline(always)]
	fn glMultiTexCoordP2ui(&self, texture: GLenum, type_: GLenum, coords: GLuint) {
		(self.multitexcoordp2ui)(texture, type_, coords)
	}
	#[inline(always)]
	fn glMultiTexCoordP2uiv(&self, texture: GLenum, type_: GLenum, coords: *const GLuint) {
		(self.multitexcoordp2uiv)(texture, type_, coords)
	}
	#[inline(always)]
	fn glMultiTexCoordP3ui(&self, texture: GLenum, type_: GLenum, coords: GLuint) {
		(self.multitexcoordp3ui)(texture, type_, coords)
	}
	#[inline(always)]
	fn glMultiTexCoordP3uiv(&self, texture: GLenum, type_: GLenum, coords: *const GLuint) {
		(self.multitexcoordp3uiv)(texture, type_, coords)
	}
	#[inline(always)]
	fn glMultiTexCoordP4ui(&self, texture: GLenum, type_: GLenum, coords: GLuint) {
		(self.multitexcoordp4ui)(texture, type_, coords)
	}
	#[inline(always)]
	fn glMultiTexCoordP4uiv(&self, texture: GLenum, type_: GLenum, coords: *const GLuint) {
		(self.multitexcoordp4uiv)(texture, type_, coords)
	}
	#[inline(always)]
	fn glNormalP3ui(&self, type_: GLenum, coords: GLuint) {
		(self.normalp3ui)(type_, coords)
	}
	#[inline(always)]
	fn glNormalP3uiv(&self, type_: GLenum, coords: *const GLuint) {
		(self.normalp3uiv)(type_, coords)
	}
	#[inline(always)]
	fn glColorP3ui(&self, type_: GLenum, color: GLuint) {
		(self.colorp3ui)(type_, color)
	}
	#[inline(always)]
	fn glColorP3uiv(&self, type_: GLenum, color: *const GLuint) {
		(self.colorp3uiv)(type_, color)
	}
	#[inline(always)]
	fn glColorP4ui(&self, type_: GLenum, color: GLuint) {
		(self.colorp4ui)(type_, color)
	}
	#[inline(always)]
	fn glColorP4uiv(&self, type_: GLenum, color: *const GLuint) {
		(self.colorp4uiv)(type_, color)
	}
	#[inline(always)]
	fn glSecondaryColorP3ui(&self, type_: GLenum, color: GLuint) {
		(self.secondarycolorp3ui)(type_, color)
	}
	#[inline(always)]
	fn glSecondaryColorP3uiv(&self, type_: GLenum, color: *const GLuint) {
		(self.secondarycolorp3uiv)(type_, color)
	}
}

impl Version33 {
	pub fn new(base: impl GL_1_0, get_proc_address: &impl Fn(&'static str) -> *const c_void) -> Self {
		let (spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 30300 {
			return Self::default();
		}
		Self {
			available: true,
			bindfragdatalocationindexed: unsafe{transmute(get_proc_address("glBindFragDataLocationIndexed"))},
			getfragdataindex: unsafe{transmute(get_proc_address("glGetFragDataIndex"))},
			gensamplers: unsafe{transmute(get_proc_address("glGenSamplers"))},
			deletesamplers: unsafe{transmute(get_proc_address("glDeleteSamplers"))},
			issampler: unsafe{transmute(get_proc_address("glIsSampler"))},
			bindsampler: unsafe{transmute(get_proc_address("glBindSampler"))},
			samplerparameteri: unsafe{transmute(get_proc_address("glSamplerParameteri"))},
			samplerparameteriv: unsafe{transmute(get_proc_address("glSamplerParameteriv"))},
			samplerparameterf: unsafe{transmute(get_proc_address("glSamplerParameterf"))},
			samplerparameterfv: unsafe{transmute(get_proc_address("glSamplerParameterfv"))},
			samplerparameteriiv: unsafe{transmute(get_proc_address("glSamplerParameterIiv"))},
			samplerparameteriuiv: unsafe{transmute(get_proc_address("glSamplerParameterIuiv"))},
			getsamplerparameteriv: unsafe{transmute(get_proc_address("glGetSamplerParameteriv"))},
			getsamplerparameteriiv: unsafe{transmute(get_proc_address("glGetSamplerParameterIiv"))},
			getsamplerparameterfv: unsafe{transmute(get_proc_address("glGetSamplerParameterfv"))},
			getsamplerparameteriuiv: unsafe{transmute(get_proc_address("glGetSamplerParameterIuiv"))},
			querycounter: unsafe{transmute(get_proc_address("glQueryCounter"))},
			getqueryobjecti64v: unsafe{transmute(get_proc_address("glGetQueryObjecti64v"))},
			getqueryobjectui64v: unsafe{transmute(get_proc_address("glGetQueryObjectui64v"))},
			vertexattribdivisor: unsafe{transmute(get_proc_address("glVertexAttribDivisor"))},
			vertexattribp1ui: unsafe{transmute(get_proc_address("glVertexAttribP1ui"))},
			vertexattribp1uiv: unsafe{transmute(get_proc_address("glVertexAttribP1uiv"))},
			vertexattribp2ui: unsafe{transmute(get_proc_address("glVertexAttribP2ui"))},
			vertexattribp2uiv: unsafe{transmute(get_proc_address("glVertexAttribP2uiv"))},
			vertexattribp3ui: unsafe{transmute(get_proc_address("glVertexAttribP3ui"))},
			vertexattribp3uiv: unsafe{transmute(get_proc_address("glVertexAttribP3uiv"))},
			vertexattribp4ui: unsafe{transmute(get_proc_address("glVertexAttribP4ui"))},
			vertexattribp4uiv: unsafe{transmute(get_proc_address("glVertexAttribP4uiv"))},
			vertexp2ui: unsafe{transmute(get_proc_address("glVertexP2ui"))},
			vertexp2uiv: unsafe{transmute(get_proc_address("glVertexP2uiv"))},
			vertexp3ui: unsafe{transmute(get_proc_address("glVertexP3ui"))},
			vertexp3uiv: unsafe{transmute(get_proc_address("glVertexP3uiv"))},
			vertexp4ui: unsafe{transmute(get_proc_address("glVertexP4ui"))},
			vertexp4uiv: unsafe{transmute(get_proc_address("glVertexP4uiv"))},
			texcoordp1ui: unsafe{transmute(get_proc_address("glTexCoordP1ui"))},
			texcoordp1uiv: unsafe{transmute(get_proc_address("glTexCoordP1uiv"))},
			texcoordp2ui: unsafe{transmute(get_proc_address("glTexCoordP2ui"))},
			texcoordp2uiv: unsafe{transmute(get_proc_address("glTexCoordP2uiv"))},
			texcoordp3ui: unsafe{transmute(get_proc_address("glTexCoordP3ui"))},
			texcoordp3uiv: unsafe{transmute(get_proc_address("glTexCoordP3uiv"))},
			texcoordp4ui: unsafe{transmute(get_proc_address("glTexCoordP4ui"))},
			texcoordp4uiv: unsafe{transmute(get_proc_address("glTexCoordP4uiv"))},
			multitexcoordp1ui: unsafe{transmute(get_proc_address("glMultiTexCoordP1ui"))},
			multitexcoordp1uiv: unsafe{transmute(get_proc_address("glMultiTexCoordP1uiv"))},
			multitexcoordp2ui: unsafe{transmute(get_proc_address("glMultiTexCoordP2ui"))},
			multitexcoordp2uiv: unsafe{transmute(get_proc_address("glMultiTexCoordP2uiv"))},
			multitexcoordp3ui: unsafe{transmute(get_proc_address("glMultiTexCoordP3ui"))},
			multitexcoordp3uiv: unsafe{transmute(get_proc_address("glMultiTexCoordP3uiv"))},
			multitexcoordp4ui: unsafe{transmute(get_proc_address("glMultiTexCoordP4ui"))},
			multitexcoordp4uiv: unsafe{transmute(get_proc_address("glMultiTexCoordP4uiv"))},
			normalp3ui: unsafe{transmute(get_proc_address("glNormalP3ui"))},
			normalp3uiv: unsafe{transmute(get_proc_address("glNormalP3uiv"))},
			colorp3ui: unsafe{transmute(get_proc_address("glColorP3ui"))},
			colorp3uiv: unsafe{transmute(get_proc_address("glColorP3uiv"))},
			colorp4ui: unsafe{transmute(get_proc_address("glColorP4ui"))},
			colorp4uiv: unsafe{transmute(get_proc_address("glColorP4uiv"))},
			secondarycolorp3ui: unsafe{transmute(get_proc_address("glSecondaryColorP3ui"))},
			secondarycolorp3uiv: unsafe{transmute(get_proc_address("glSecondaryColorP3uiv"))},
		}
	}
	#[inline(always)]
	pub fn get_available(&self) -> bool {
		self.available
	}
}

impl Default for Version33 {
	fn default() -> Self {
		Self {
			available: false,
			bindfragdatalocationindexed: null(),
			getfragdataindex: null(),
			gensamplers: null(),
			deletesamplers: null(),
			issampler: null(),
			bindsampler: null(),
			samplerparameteri: null(),
			samplerparameteriv: null(),
			samplerparameterf: null(),
			samplerparameterfv: null(),
			samplerparameteriiv: null(),
			samplerparameteriuiv: null(),
			getsamplerparameteriv: null(),
			getsamplerparameteriiv: null(),
			getsamplerparameterfv: null(),
			getsamplerparameteriuiv: null(),
			querycounter: null(),
			getqueryobjecti64v: null(),
			getqueryobjectui64v: null(),
			vertexattribdivisor: null(),
			vertexattribp1ui: null(),
			vertexattribp1uiv: null(),
			vertexattribp2ui: null(),
			vertexattribp2uiv: null(),
			vertexattribp3ui: null(),
			vertexattribp3uiv: null(),
			vertexattribp4ui: null(),
			vertexattribp4uiv: null(),
			vertexp2ui: null(),
			vertexp2uiv: null(),
			vertexp3ui: null(),
			vertexp3uiv: null(),
			vertexp4ui: null(),
			vertexp4uiv: null(),
			texcoordp1ui: null(),
			texcoordp1uiv: null(),
			texcoordp2ui: null(),
			texcoordp2uiv: null(),
			texcoordp3ui: null(),
			texcoordp3uiv: null(),
			texcoordp4ui: null(),
			texcoordp4uiv: null(),
			multitexcoordp1ui: null(),
			multitexcoordp1uiv: null(),
			multitexcoordp2ui: null(),
			multitexcoordp2uiv: null(),
			multitexcoordp3ui: null(),
			multitexcoordp3uiv: null(),
			multitexcoordp4ui: null(),
			multitexcoordp4uiv: null(),
			normalp3ui: null(),
			normalp3uiv: null(),
			colorp3ui: null(),
			colorp3uiv: null(),
			colorp4ui: null(),
			colorp4uiv: null(),
			secondarycolorp3ui: null(),
			secondarycolorp3uiv: null(),
		}
	}
}

type PFNGLMINSAMPLESHADINGPROC = extern "system" fn(GLfloat);
type PFNGLBLENDEQUATIONIPROC = extern "system" fn(GLuint, GLenum);
type PFNGLBLENDEQUATIONSEPARATEIPROC = extern "system" fn(GLuint, GLenum, GLenum);
type PFNGLBLENDFUNCIPROC = extern "system" fn(GLuint, GLenum, GLenum);
type PFNGLBLENDFUNCSEPARATEIPROC = extern "system" fn(GLuint, GLenum, GLenum, GLenum, GLenum);
type PFNGLDRAWARRAYSINDIRECTPROC = extern "system" fn(GLenum, *const c_void);
type PFNGLDRAWELEMENTSINDIRECTPROC = extern "system" fn(GLenum, GLenum, *const c_void);
type PFNGLUNIFORM1DPROC = extern "system" fn(GLint, GLdouble);
type PFNGLUNIFORM2DPROC = extern "system" fn(GLint, GLdouble, GLdouble);
type PFNGLUNIFORM3DPROC = extern "system" fn(GLint, GLdouble, GLdouble, GLdouble);
type PFNGLUNIFORM4DPROC = extern "system" fn(GLint, GLdouble, GLdouble, GLdouble, GLdouble);
type PFNGLUNIFORM1DVPROC = extern "system" fn(GLint, GLsizei, *const GLdouble);
type PFNGLUNIFORM2DVPROC = extern "system" fn(GLint, GLsizei, *const GLdouble);
type PFNGLUNIFORM3DVPROC = extern "system" fn(GLint, GLsizei, *const GLdouble);
type PFNGLUNIFORM4DVPROC = extern "system" fn(GLint, GLsizei, *const GLdouble);
type PFNGLUNIFORMMATRIX2DVPROC = extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble);
type PFNGLUNIFORMMATRIX3DVPROC = extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble);
type PFNGLUNIFORMMATRIX4DVPROC = extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble);
type PFNGLUNIFORMMATRIX2X3DVPROC = extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble);
type PFNGLUNIFORMMATRIX2X4DVPROC = extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble);
type PFNGLUNIFORMMATRIX3X2DVPROC = extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble);
type PFNGLUNIFORMMATRIX3X4DVPROC = extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble);
type PFNGLUNIFORMMATRIX4X2DVPROC = extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble);
type PFNGLUNIFORMMATRIX4X3DVPROC = extern "system" fn(GLint, GLsizei, GLboolean, *const GLdouble);
type PFNGLGETUNIFORMDVPROC = extern "system" fn(GLuint, GLint, *mut GLdouble);
type PFNGLGETSUBROUTINEUNIFORMLOCATIONPROC = extern "system" fn(GLuint, GLenum, *const GLchar) -> GLint;
type PFNGLGETSUBROUTINEINDEXPROC = extern "system" fn(GLuint, GLenum, *const GLchar) -> GLuint;
type PFNGLGETACTIVESUBROUTINEUNIFORMIVPROC = extern "system" fn(GLuint, GLenum, GLuint, GLenum, *mut GLint);
type PFNGLGETACTIVESUBROUTINEUNIFORMNAMEPROC = extern "system" fn(GLuint, GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar);
type PFNGLGETACTIVESUBROUTINENAMEPROC = extern "system" fn(GLuint, GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar);
type PFNGLUNIFORMSUBROUTINESUIVPROC = extern "system" fn(GLenum, GLsizei, *const GLuint);
type PFNGLGETUNIFORMSUBROUTINEUIVPROC = extern "system" fn(GLenum, GLint, *mut GLuint);
type PFNGLGETPROGRAMSTAGEIVPROC = extern "system" fn(GLuint, GLenum, GLenum, *mut GLint);
type PFNGLPATCHPARAMETERIPROC = extern "system" fn(GLenum, GLint);
type PFNGLPATCHPARAMETERFVPROC = extern "system" fn(GLenum, *const GLfloat);
type PFNGLBINDTRANSFORMFEEDBACKPROC = extern "system" fn(GLenum, GLuint);
type PFNGLDELETETRANSFORMFEEDBACKSPROC = extern "system" fn(GLsizei, *const GLuint);
type PFNGLGENTRANSFORMFEEDBACKSPROC = extern "system" fn(GLsizei, *mut GLuint);
type PFNGLISTRANSFORMFEEDBACKPROC = extern "system" fn(GLuint) -> GLboolean;
type PFNGLPAUSETRANSFORMFEEDBACKPROC = extern "system" fn();
type PFNGLRESUMETRANSFORMFEEDBACKPROC = extern "system" fn();
type PFNGLDRAWTRANSFORMFEEDBACKPROC = extern "system" fn(GLenum, GLuint);
type PFNGLDRAWTRANSFORMFEEDBACKSTREAMPROC = extern "system" fn(GLenum, GLuint, GLuint);
type PFNGLBEGINQUERYINDEXEDPROC = extern "system" fn(GLenum, GLuint, GLuint);
type PFNGLENDQUERYINDEXEDPROC = extern "system" fn(GLenum, GLuint);
type PFNGLGETQUERYINDEXEDIVPROC = extern "system" fn(GLenum, GLuint, GLenum, *mut GLint);
const SAMPLE_SHADING: GLenum = 0x8C36;
const MIN_SAMPLE_SHADING_VALUE: GLenum = 0x8C37;
const MIN_PROGRAM_TEXTURE_GATHER_OFFSET: GLenum = 0x8E5E;
const MAX_PROGRAM_TEXTURE_GATHER_OFFSET: GLenum = 0x8E5F;
const TEXTURE_CUBE_MAP_ARRAY: GLenum = 0x9009;
const TEXTURE_BINDING_CUBE_MAP_ARRAY: GLenum = 0x900A;
const PROXY_TEXTURE_CUBE_MAP_ARRAY: GLenum = 0x900B;
const SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900C;
const SAMPLER_CUBE_MAP_ARRAY_SHADOW: GLenum = 0x900D;
const INT_SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900E;
const UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900F;
const DRAW_INDIRECT_BUFFER: GLenum = 0x8F3F;
const DRAW_INDIRECT_BUFFER_BINDING: GLenum = 0x8F43;
const GEOMETRY_SHADER_INVOCATIONS: GLenum = 0x887F;
const MAX_GEOMETRY_SHADER_INVOCATIONS: GLenum = 0x8E5A;
const MIN_FRAGMENT_INTERPOLATION_OFFSET: GLenum = 0x8E5B;
const MAX_FRAGMENT_INTERPOLATION_OFFSET: GLenum = 0x8E5C;
const FRAGMENT_INTERPOLATION_OFFSET_BITS: GLenum = 0x8E5D;
const MAX_VERTEX_STREAMS: GLenum = 0x8E71;
const DOUBLE_VEC2: GLenum = 0x8FFC;
const DOUBLE_VEC3: GLenum = 0x8FFD;
const DOUBLE_VEC4: GLenum = 0x8FFE;
const DOUBLE_MAT2: GLenum = 0x8F46;
const DOUBLE_MAT3: GLenum = 0x8F47;
const DOUBLE_MAT4: GLenum = 0x8F48;
const DOUBLE_MAT2x3: GLenum = 0x8F49;
const DOUBLE_MAT2x4: GLenum = 0x8F4A;
const DOUBLE_MAT3x2: GLenum = 0x8F4B;
const DOUBLE_MAT3x4: GLenum = 0x8F4C;
const DOUBLE_MAT4x2: GLenum = 0x8F4D;
const DOUBLE_MAT4x3: GLenum = 0x8F4E;
const ACTIVE_SUBROUTINES: GLenum = 0x8DE5;
const ACTIVE_SUBROUTINE_UNIFORMS: GLenum = 0x8DE6;
const ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS: GLenum = 0x8E47;
const ACTIVE_SUBROUTINE_MAX_LENGTH: GLenum = 0x8E48;
const ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH: GLenum = 0x8E49;
const MAX_SUBROUTINES: GLenum = 0x8DE7;
const MAX_SUBROUTINE_UNIFORM_LOCATIONS: GLenum = 0x8DE8;
const NUM_COMPATIBLE_SUBROUTINES: GLenum = 0x8E4A;
const COMPATIBLE_SUBROUTINES: GLenum = 0x8E4B;
const PATCHES: GLenum = 0x000E;
const PATCH_VERTICES: GLenum = 0x8E72;
const PATCH_DEFAULT_INNER_LEVEL: GLenum = 0x8E73;
const PATCH_DEFAULT_OUTER_LEVEL: GLenum = 0x8E74;
const TESS_CONTROL_OUTPUT_VERTICES: GLenum = 0x8E75;
const TESS_GEN_MODE: GLenum = 0x8E76;
const TESS_GEN_SPACING: GLenum = 0x8E77;
const TESS_GEN_VERTEX_ORDER: GLenum = 0x8E78;
const TESS_GEN_POINT_MODE: GLenum = 0x8E79;
const ISOLINES: GLenum = 0x8E7A;
const FRACTIONAL_ODD: GLenum = 0x8E7B;
const FRACTIONAL_EVEN: GLenum = 0x8E7C;
const MAX_PATCH_VERTICES: GLenum = 0x8E7D;
const MAX_TESS_GEN_LEVEL: GLenum = 0x8E7E;
const MAX_TESS_CONTROL_UNIFORM_COMPONENTS: GLenum = 0x8E7F;
const MAX_TESS_EVALUATION_UNIFORM_COMPONENTS: GLenum = 0x8E80;
const MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS: GLenum = 0x8E81;
const MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS: GLenum = 0x8E82;
const MAX_TESS_CONTROL_OUTPUT_COMPONENTS: GLenum = 0x8E83;
const MAX_TESS_PATCH_COMPONENTS: GLenum = 0x8E84;
const MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS: GLenum = 0x8E85;
const MAX_TESS_EVALUATION_OUTPUT_COMPONENTS: GLenum = 0x8E86;
const MAX_TESS_CONTROL_UNIFORM_BLOCKS: GLenum = 0x8E89;
const MAX_TESS_EVALUATION_UNIFORM_BLOCKS: GLenum = 0x8E8A;
const MAX_TESS_CONTROL_INPUT_COMPONENTS: GLenum = 0x886C;
const MAX_TESS_EVALUATION_INPUT_COMPONENTS: GLenum = 0x886D;
const MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS: GLenum = 0x8E1E;
const MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS: GLenum = 0x8E1F;
const UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x84F0;
const UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x84F1;
const TESS_EVALUATION_SHADER: GLenum = 0x8E87;
const TESS_CONTROL_SHADER: GLenum = 0x8E88;
const TRANSFORM_FEEDBACK: GLenum = 0x8E22;
const TRANSFORM_FEEDBACK_BUFFER_PAUSED: GLenum = 0x8E23;
const TRANSFORM_FEEDBACK_BUFFER_ACTIVE: GLenum = 0x8E24;
const TRANSFORM_FEEDBACK_BINDING: GLenum = 0x8E25;
const MAX_TRANSFORM_FEEDBACK_BUFFERS: GLenum = 0x8E70;

pub trait GL_4_0 {
	fn glMinSampleShading(&self, _: GLfloat) {
		panic!("OpenGL function pointer of `glMinSampleShading` is NULL");
	}
	fn glBlendEquationi(&self, _: GLuint, _: GLenum) {
		panic!("OpenGL function pointer of `glBlendEquationi` is NULL");
	}
	fn glBlendEquationSeparatei(&self, _: GLuint, _: GLenum, _: GLenum) {
		panic!("OpenGL function pointer of `glBlendEquationSeparatei` is NULL");
	}
	fn glBlendFunci(&self, _: GLuint, _: GLenum, _: GLenum) {
		panic!("OpenGL function pointer of `glBlendFunci` is NULL");
	}
	fn glBlendFuncSeparatei(&self, _: GLuint, _: GLenum, _: GLenum, _: GLenum, _: GLenum) {
		panic!("OpenGL function pointer of `glBlendFuncSeparatei` is NULL");
	}
	fn glDrawArraysIndirect(&self, _: GLenum, _: *const c_void) {
		panic!("OpenGL function pointer of `glDrawArraysIndirect` is NULL");
	}
	fn glDrawElementsIndirect(&self, _: GLenum, _: GLenum, _: *const c_void) {
		panic!("OpenGL function pointer of `glDrawElementsIndirect` is NULL");
	}
	fn glUniform1d(&self, _: GLint, _: GLdouble) {
		panic!("OpenGL function pointer of `glUniform1d` is NULL");
	}
	fn glUniform2d(&self, _: GLint, _: GLdouble, _: GLdouble) {
		panic!("OpenGL function pointer of `glUniform2d` is NULL");
	}
	fn glUniform3d(&self, _: GLint, _: GLdouble, _: GLdouble, _: GLdouble) {
		panic!("OpenGL function pointer of `glUniform3d` is NULL");
	}
	fn glUniform4d(&self, _: GLint, _: GLdouble, _: GLdouble, _: GLdouble, _: GLdouble) {
		panic!("OpenGL function pointer of `glUniform4d` is NULL");
	}
	fn glUniform1dv(&self, _: GLint, _: GLsizei, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glUniform1dv` is NULL");
	}
	fn glUniform2dv(&self, _: GLint, _: GLsizei, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glUniform2dv` is NULL");
	}
	fn glUniform3dv(&self, _: GLint, _: GLsizei, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glUniform3dv` is NULL");
	}
	fn glUniform4dv(&self, _: GLint, _: GLsizei, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glUniform4dv` is NULL");
	}
	fn glUniformMatrix2dv(&self, _: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glUniformMatrix2dv` is NULL");
	}
	fn glUniformMatrix3dv(&self, _: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glUniformMatrix3dv` is NULL");
	}
	fn glUniformMatrix4dv(&self, _: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glUniformMatrix4dv` is NULL");
	}
	fn glUniformMatrix2x3dv(&self, _: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glUniformMatrix2x3dv` is NULL");
	}
	fn glUniformMatrix2x4dv(&self, _: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glUniformMatrix2x4dv` is NULL");
	}
	fn glUniformMatrix3x2dv(&self, _: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glUniformMatrix3x2dv` is NULL");
	}
	fn glUniformMatrix3x4dv(&self, _: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glUniformMatrix3x4dv` is NULL");
	}
	fn glUniformMatrix4x2dv(&self, _: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glUniformMatrix4x2dv` is NULL");
	}
	fn glUniformMatrix4x3dv(&self, _: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glUniformMatrix4x3dv` is NULL");
	}
	fn glGetUniformdv(&self, _: GLuint, _: GLint, _: *mut GLdouble) {
		panic!("OpenGL function pointer of `glGetUniformdv` is NULL");
	}
	fn glGetSubroutineUniformLocation(&self, _: GLuint, _: GLenum, _: *const GLchar) -> GLint {
		panic!("OpenGL function pointer of `glGetSubroutineUniformLocation` is NULL");
	}
	fn glGetSubroutineIndex(&self, _: GLuint, _: GLenum, _: *const GLchar) -> GLuint {
		panic!("OpenGL function pointer of `glGetSubroutineIndex` is NULL");
	}
	fn glGetActiveSubroutineUniformiv(&self, _: GLuint, _: GLenum, _: GLuint, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetActiveSubroutineUniformiv` is NULL");
	}
	fn glGetActiveSubroutineUniformName(&self, _: GLuint, _: GLenum, _: GLuint, _: GLsizei, _: *mut GLsizei, _: *mut GLchar) {
		panic!("OpenGL function pointer of `glGetActiveSubroutineUniformName` is NULL");
	}
	fn glGetActiveSubroutineName(&self, _: GLuint, _: GLenum, _: GLuint, _: GLsizei, _: *mut GLsizei, _: *mut GLchar) {
		panic!("OpenGL function pointer of `glGetActiveSubroutineName` is NULL");
	}
	fn glUniformSubroutinesuiv(&self, _: GLenum, _: GLsizei, _: *const GLuint) {
		panic!("OpenGL function pointer of `glUniformSubroutinesuiv` is NULL");
	}
	fn glGetUniformSubroutineuiv(&self, _: GLenum, _: GLint, _: *mut GLuint) {
		panic!("OpenGL function pointer of `glGetUniformSubroutineuiv` is NULL");
	}
	fn glGetProgramStageiv(&self, _: GLuint, _: GLenum, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetProgramStageiv` is NULL");
	}
	fn glPatchParameteri(&self, _: GLenum, _: GLint) {
		panic!("OpenGL function pointer of `glPatchParameteri` is NULL");
	}
	fn glPatchParameterfv(&self, _: GLenum, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glPatchParameterfv` is NULL");
	}
	fn glBindTransformFeedback(&self, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glBindTransformFeedback` is NULL");
	}
	fn glDeleteTransformFeedbacks(&self, _: GLsizei, _: *const GLuint) {
		panic!("OpenGL function pointer of `glDeleteTransformFeedbacks` is NULL");
	}
	fn glGenTransformFeedbacks(&self, _: GLsizei, _: *mut GLuint) {
		panic!("OpenGL function pointer of `glGenTransformFeedbacks` is NULL");
	}
	fn glIsTransformFeedback(&self, _: GLuint) -> GLboolean {
		panic!("OpenGL function pointer of `glIsTransformFeedback` is NULL");
	}
	fn glPauseTransformFeedback(&self) {
		panic!("OpenGL function pointer of `glPauseTransformFeedback` is NULL");
	}
	fn glResumeTransformFeedback(&self) {
		panic!("OpenGL function pointer of `glResumeTransformFeedback` is NULL");
	}
	fn glDrawTransformFeedback(&self, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glDrawTransformFeedback` is NULL");
	}
	fn glDrawTransformFeedbackStream(&self, _: GLenum, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glDrawTransformFeedbackStream` is NULL");
	}
	fn glBeginQueryIndexed(&self, _: GLenum, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glBeginQueryIndexed` is NULL");
	}
	fn glEndQueryIndexed(&self, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glEndQueryIndexed` is NULL");
	}
	fn glGetQueryIndexediv(&self, _: GLenum, _: GLuint, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetQueryIndexediv` is NULL");
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Version40 {
	available: bool,
	minsampleshading: PFNGLMINSAMPLESHADINGPROC,
	blendequationi: PFNGLBLENDEQUATIONIPROC,
	blendequationseparatei: PFNGLBLENDEQUATIONSEPARATEIPROC,
	blendfunci: PFNGLBLENDFUNCIPROC,
	blendfuncseparatei: PFNGLBLENDFUNCSEPARATEIPROC,
	drawarraysindirect: PFNGLDRAWARRAYSINDIRECTPROC,
	drawelementsindirect: PFNGLDRAWELEMENTSINDIRECTPROC,
	uniform1d: PFNGLUNIFORM1DPROC,
	uniform2d: PFNGLUNIFORM2DPROC,
	uniform3d: PFNGLUNIFORM3DPROC,
	uniform4d: PFNGLUNIFORM4DPROC,
	uniform1dv: PFNGLUNIFORM1DVPROC,
	uniform2dv: PFNGLUNIFORM2DVPROC,
	uniform3dv: PFNGLUNIFORM3DVPROC,
	uniform4dv: PFNGLUNIFORM4DVPROC,
	uniformmatrix2dv: PFNGLUNIFORMMATRIX2DVPROC,
	uniformmatrix3dv: PFNGLUNIFORMMATRIX3DVPROC,
	uniformmatrix4dv: PFNGLUNIFORMMATRIX4DVPROC,
	uniformmatrix2x3dv: PFNGLUNIFORMMATRIX2X3DVPROC,
	uniformmatrix2x4dv: PFNGLUNIFORMMATRIX2X4DVPROC,
	uniformmatrix3x2dv: PFNGLUNIFORMMATRIX3X2DVPROC,
	uniformmatrix3x4dv: PFNGLUNIFORMMATRIX3X4DVPROC,
	uniformmatrix4x2dv: PFNGLUNIFORMMATRIX4X2DVPROC,
	uniformmatrix4x3dv: PFNGLUNIFORMMATRIX4X3DVPROC,
	getuniformdv: PFNGLGETUNIFORMDVPROC,
	getsubroutineuniformlocation: PFNGLGETSUBROUTINEUNIFORMLOCATIONPROC,
	getsubroutineindex: PFNGLGETSUBROUTINEINDEXPROC,
	getactivesubroutineuniformiv: PFNGLGETACTIVESUBROUTINEUNIFORMIVPROC,
	getactivesubroutineuniformname: PFNGLGETACTIVESUBROUTINEUNIFORMNAMEPROC,
	getactivesubroutinename: PFNGLGETACTIVESUBROUTINENAMEPROC,
	uniformsubroutinesuiv: PFNGLUNIFORMSUBROUTINESUIVPROC,
	getuniformsubroutineuiv: PFNGLGETUNIFORMSUBROUTINEUIVPROC,
	getprogramstageiv: PFNGLGETPROGRAMSTAGEIVPROC,
	patchparameteri: PFNGLPATCHPARAMETERIPROC,
	patchparameterfv: PFNGLPATCHPARAMETERFVPROC,
	bindtransformfeedback: PFNGLBINDTRANSFORMFEEDBACKPROC,
	deletetransformfeedbacks: PFNGLDELETETRANSFORMFEEDBACKSPROC,
	gentransformfeedbacks: PFNGLGENTRANSFORMFEEDBACKSPROC,
	istransformfeedback: PFNGLISTRANSFORMFEEDBACKPROC,
	pausetransformfeedback: PFNGLPAUSETRANSFORMFEEDBACKPROC,
	resumetransformfeedback: PFNGLRESUMETRANSFORMFEEDBACKPROC,
	drawtransformfeedback: PFNGLDRAWTRANSFORMFEEDBACKPROC,
	drawtransformfeedbackstream: PFNGLDRAWTRANSFORMFEEDBACKSTREAMPROC,
	beginqueryindexed: PFNGLBEGINQUERYINDEXEDPROC,
	endqueryindexed: PFNGLENDQUERYINDEXEDPROC,
	getqueryindexediv: PFNGLGETQUERYINDEXEDIVPROC,
}

impl GL_4_0 for Version40 {
	#[inline(always)]
	fn glMinSampleShading(&self, value: GLfloat) {
		(self.minsampleshading)(value)
	}
	#[inline(always)]
	fn glBlendEquationi(&self, buf: GLuint, mode: GLenum) {
		(self.blendequationi)(buf, mode)
	}
	#[inline(always)]
	fn glBlendEquationSeparatei(&self, buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum) {
		(self.blendequationseparatei)(buf, modeRGB, modeAlpha)
	}
	#[inline(always)]
	fn glBlendFunci(&self, buf: GLuint, src: GLenum, dst: GLenum) {
		(self.blendfunci)(buf, src, dst)
	}
	#[inline(always)]
	fn glBlendFuncSeparatei(&self, buf: GLuint, srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum) {
		(self.blendfuncseparatei)(buf, srcRGB, dstRGB, srcAlpha, dstAlpha)
	}
	#[inline(always)]
	fn glDrawArraysIndirect(&self, mode: GLenum, indirect: *const c_void) {
		(self.drawarraysindirect)(mode, indirect)
	}
	#[inline(always)]
	fn glDrawElementsIndirect(&self, mode: GLenum, type_: GLenum, indirect: *const c_void) {
		(self.drawelementsindirect)(mode, type_, indirect)
	}
	#[inline(always)]
	fn glUniform1d(&self, location: GLint, x: GLdouble) {
		(self.uniform1d)(location, x)
	}
	#[inline(always)]
	fn glUniform2d(&self, location: GLint, x: GLdouble, y: GLdouble) {
		(self.uniform2d)(location, x, y)
	}
	#[inline(always)]
	fn glUniform3d(&self, location: GLint, x: GLdouble, y: GLdouble, z: GLdouble) {
		(self.uniform3d)(location, x, y, z)
	}
	#[inline(always)]
	fn glUniform4d(&self, location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) {
		(self.uniform4d)(location, x, y, z, w)
	}
	#[inline(always)]
	fn glUniform1dv(&self, location: GLint, count: GLsizei, value: *const GLdouble) {
		(self.uniform1dv)(location, count, value)
	}
	#[inline(always)]
	fn glUniform2dv(&self, location: GLint, count: GLsizei, value: *const GLdouble) {
		(self.uniform2dv)(location, count, value)
	}
	#[inline(always)]
	fn glUniform3dv(&self, location: GLint, count: GLsizei, value: *const GLdouble) {
		(self.uniform3dv)(location, count, value)
	}
	#[inline(always)]
	fn glUniform4dv(&self, location: GLint, count: GLsizei, value: *const GLdouble) {
		(self.uniform4dv)(location, count, value)
	}
	#[inline(always)]
	fn glUniformMatrix2dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.uniformmatrix2dv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glUniformMatrix3dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.uniformmatrix3dv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glUniformMatrix4dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.uniformmatrix4dv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glUniformMatrix2x3dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.uniformmatrix2x3dv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glUniformMatrix2x4dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.uniformmatrix2x4dv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glUniformMatrix3x2dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.uniformmatrix3x2dv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glUniformMatrix3x4dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.uniformmatrix3x4dv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glUniformMatrix4x2dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.uniformmatrix4x2dv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glUniformMatrix4x3dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.uniformmatrix4x3dv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glGetUniformdv(&self, program: GLuint, location: GLint, params: *mut GLdouble) {
		(self.getuniformdv)(program, location, params)
	}
	#[inline(always)]
	fn glGetSubroutineUniformLocation(&self, program: GLuint, shadertype: GLenum, name: *const GLchar) -> GLint {
		(self.getsubroutineuniformlocation)(program, shadertype, name)
	}
	#[inline(always)]
	fn glGetSubroutineIndex(&self, program: GLuint, shadertype: GLenum, name: *const GLchar) -> GLuint {
		(self.getsubroutineindex)(program, shadertype, name)
	}
	#[inline(always)]
	fn glGetActiveSubroutineUniformiv(&self, program: GLuint, shadertype: GLenum, index: GLuint, pname: GLenum, values: *mut GLint) {
		(self.getactivesubroutineuniformiv)(program, shadertype, index, pname, values)
	}
	#[inline(always)]
	fn glGetActiveSubroutineUniformName(&self, program: GLuint, shadertype: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar) {
		(self.getactivesubroutineuniformname)(program, shadertype, index, bufSize, length, name)
	}
	#[inline(always)]
	fn glGetActiveSubroutineName(&self, program: GLuint, shadertype: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar) {
		(self.getactivesubroutinename)(program, shadertype, index, bufSize, length, name)
	}
	#[inline(always)]
	fn glUniformSubroutinesuiv(&self, shadertype: GLenum, count: GLsizei, indices: *const GLuint) {
		(self.uniformsubroutinesuiv)(shadertype, count, indices)
	}
	#[inline(always)]
	fn glGetUniformSubroutineuiv(&self, shadertype: GLenum, location: GLint, params: *mut GLuint) {
		(self.getuniformsubroutineuiv)(shadertype, location, params)
	}
	#[inline(always)]
	fn glGetProgramStageiv(&self, program: GLuint, shadertype: GLenum, pname: GLenum, values: *mut GLint) {
		(self.getprogramstageiv)(program, shadertype, pname, values)
	}
	#[inline(always)]
	fn glPatchParameteri(&self, pname: GLenum, value: GLint) {
		(self.patchparameteri)(pname, value)
	}
	#[inline(always)]
	fn glPatchParameterfv(&self, pname: GLenum, values: *const GLfloat) {
		(self.patchparameterfv)(pname, values)
	}
	#[inline(always)]
	fn glBindTransformFeedback(&self, target: GLenum, id: GLuint) {
		(self.bindtransformfeedback)(target, id)
	}
	#[inline(always)]
	fn glDeleteTransformFeedbacks(&self, n: GLsizei, ids: *const GLuint) {
		(self.deletetransformfeedbacks)(n, ids)
	}
	#[inline(always)]
	fn glGenTransformFeedbacks(&self, n: GLsizei, ids: *mut GLuint) {
		(self.gentransformfeedbacks)(n, ids)
	}
	#[inline(always)]
	fn glIsTransformFeedback(&self, id: GLuint) -> GLboolean {
		(self.istransformfeedback)(id)
	}
	#[inline(always)]
	fn glPauseTransformFeedback(&self) {
		(self.pausetransformfeedback)()
	}
	#[inline(always)]
	fn glResumeTransformFeedback(&self) {
		(self.resumetransformfeedback)()
	}
	#[inline(always)]
	fn glDrawTransformFeedback(&self, mode: GLenum, id: GLuint) {
		(self.drawtransformfeedback)(mode, id)
	}
	#[inline(always)]
	fn glDrawTransformFeedbackStream(&self, mode: GLenum, id: GLuint, stream: GLuint) {
		(self.drawtransformfeedbackstream)(mode, id, stream)
	}
	#[inline(always)]
	fn glBeginQueryIndexed(&self, target: GLenum, index: GLuint, id: GLuint) {
		(self.beginqueryindexed)(target, index, id)
	}
	#[inline(always)]
	fn glEndQueryIndexed(&self, target: GLenum, index: GLuint) {
		(self.endqueryindexed)(target, index)
	}
	#[inline(always)]
	fn glGetQueryIndexediv(&self, target: GLenum, index: GLuint, pname: GLenum, params: *mut GLint) {
		(self.getqueryindexediv)(target, index, pname, params)
	}
}

impl Version40 {
	pub fn new(base: impl GL_1_0, get_proc_address: &impl Fn(&'static str) -> *const c_void) -> Self {
		let (spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 40000 {
			return Self::default();
		}
		Self {
			available: true,
			minsampleshading: unsafe{transmute(get_proc_address("glMinSampleShading"))},
			blendequationi: unsafe{transmute(get_proc_address("glBlendEquationi"))},
			blendequationseparatei: unsafe{transmute(get_proc_address("glBlendEquationSeparatei"))},
			blendfunci: unsafe{transmute(get_proc_address("glBlendFunci"))},
			blendfuncseparatei: unsafe{transmute(get_proc_address("glBlendFuncSeparatei"))},
			drawarraysindirect: unsafe{transmute(get_proc_address("glDrawArraysIndirect"))},
			drawelementsindirect: unsafe{transmute(get_proc_address("glDrawElementsIndirect"))},
			uniform1d: unsafe{transmute(get_proc_address("glUniform1d"))},
			uniform2d: unsafe{transmute(get_proc_address("glUniform2d"))},
			uniform3d: unsafe{transmute(get_proc_address("glUniform3d"))},
			uniform4d: unsafe{transmute(get_proc_address("glUniform4d"))},
			uniform1dv: unsafe{transmute(get_proc_address("glUniform1dv"))},
			uniform2dv: unsafe{transmute(get_proc_address("glUniform2dv"))},
			uniform3dv: unsafe{transmute(get_proc_address("glUniform3dv"))},
			uniform4dv: unsafe{transmute(get_proc_address("glUniform4dv"))},
			uniformmatrix2dv: unsafe{transmute(get_proc_address("glUniformMatrix2dv"))},
			uniformmatrix3dv: unsafe{transmute(get_proc_address("glUniformMatrix3dv"))},
			uniformmatrix4dv: unsafe{transmute(get_proc_address("glUniformMatrix4dv"))},
			uniformmatrix2x3dv: unsafe{transmute(get_proc_address("glUniformMatrix2x3dv"))},
			uniformmatrix2x4dv: unsafe{transmute(get_proc_address("glUniformMatrix2x4dv"))},
			uniformmatrix3x2dv: unsafe{transmute(get_proc_address("glUniformMatrix3x2dv"))},
			uniformmatrix3x4dv: unsafe{transmute(get_proc_address("glUniformMatrix3x4dv"))},
			uniformmatrix4x2dv: unsafe{transmute(get_proc_address("glUniformMatrix4x2dv"))},
			uniformmatrix4x3dv: unsafe{transmute(get_proc_address("glUniformMatrix4x3dv"))},
			getuniformdv: unsafe{transmute(get_proc_address("glGetUniformdv"))},
			getsubroutineuniformlocation: unsafe{transmute(get_proc_address("glGetSubroutineUniformLocation"))},
			getsubroutineindex: unsafe{transmute(get_proc_address("glGetSubroutineIndex"))},
			getactivesubroutineuniformiv: unsafe{transmute(get_proc_address("glGetActiveSubroutineUniformiv"))},
			getactivesubroutineuniformname: unsafe{transmute(get_proc_address("glGetActiveSubroutineUniformName"))},
			getactivesubroutinename: unsafe{transmute(get_proc_address("glGetActiveSubroutineName"))},
			uniformsubroutinesuiv: unsafe{transmute(get_proc_address("glUniformSubroutinesuiv"))},
			getuniformsubroutineuiv: unsafe{transmute(get_proc_address("glGetUniformSubroutineuiv"))},
			getprogramstageiv: unsafe{transmute(get_proc_address("glGetProgramStageiv"))},
			patchparameteri: unsafe{transmute(get_proc_address("glPatchParameteri"))},
			patchparameterfv: unsafe{transmute(get_proc_address("glPatchParameterfv"))},
			bindtransformfeedback: unsafe{transmute(get_proc_address("glBindTransformFeedback"))},
			deletetransformfeedbacks: unsafe{transmute(get_proc_address("glDeleteTransformFeedbacks"))},
			gentransformfeedbacks: unsafe{transmute(get_proc_address("glGenTransformFeedbacks"))},
			istransformfeedback: unsafe{transmute(get_proc_address("glIsTransformFeedback"))},
			pausetransformfeedback: unsafe{transmute(get_proc_address("glPauseTransformFeedback"))},
			resumetransformfeedback: unsafe{transmute(get_proc_address("glResumeTransformFeedback"))},
			drawtransformfeedback: unsafe{transmute(get_proc_address("glDrawTransformFeedback"))},
			drawtransformfeedbackstream: unsafe{transmute(get_proc_address("glDrawTransformFeedbackStream"))},
			beginqueryindexed: unsafe{transmute(get_proc_address("glBeginQueryIndexed"))},
			endqueryindexed: unsafe{transmute(get_proc_address("glEndQueryIndexed"))},
			getqueryindexediv: unsafe{transmute(get_proc_address("glGetQueryIndexediv"))},
		}
	}
	#[inline(always)]
	pub fn get_available(&self) -> bool {
		self.available
	}
}

impl Default for Version40 {
	fn default() -> Self {
		Self {
			available: false,
			minsampleshading: null(),
			blendequationi: null(),
			blendequationseparatei: null(),
			blendfunci: null(),
			blendfuncseparatei: null(),
			drawarraysindirect: null(),
			drawelementsindirect: null(),
			uniform1d: null(),
			uniform2d: null(),
			uniform3d: null(),
			uniform4d: null(),
			uniform1dv: null(),
			uniform2dv: null(),
			uniform3dv: null(),
			uniform4dv: null(),
			uniformmatrix2dv: null(),
			uniformmatrix3dv: null(),
			uniformmatrix4dv: null(),
			uniformmatrix2x3dv: null(),
			uniformmatrix2x4dv: null(),
			uniformmatrix3x2dv: null(),
			uniformmatrix3x4dv: null(),
			uniformmatrix4x2dv: null(),
			uniformmatrix4x3dv: null(),
			getuniformdv: null(),
			getsubroutineuniformlocation: null(),
			getsubroutineindex: null(),
			getactivesubroutineuniformiv: null(),
			getactivesubroutineuniformname: null(),
			getactivesubroutinename: null(),
			uniformsubroutinesuiv: null(),
			getuniformsubroutineuiv: null(),
			getprogramstageiv: null(),
			patchparameteri: null(),
			patchparameterfv: null(),
			bindtransformfeedback: null(),
			deletetransformfeedbacks: null(),
			gentransformfeedbacks: null(),
			istransformfeedback: null(),
			pausetransformfeedback: null(),
			resumetransformfeedback: null(),
			drawtransformfeedback: null(),
			drawtransformfeedbackstream: null(),
			beginqueryindexed: null(),
			endqueryindexed: null(),
			getqueryindexediv: null(),
		}
	}
}

type PFNGLRELEASESHADERCOMPILERPROC = extern "system" fn();
type PFNGLSHADERBINARYPROC = extern "system" fn(GLsizei, *const GLuint, GLenum, *const c_void, GLsizei);
type PFNGLGETSHADERPRECISIONFORMATPROC = extern "system" fn(GLenum, GLenum, *mut GLint, *mut GLint);
type PFNGLDEPTHRANGEFPROC = extern "system" fn(GLfloat, GLfloat);
type PFNGLCLEARDEPTHFPROC = extern "system" fn(GLfloat);
type PFNGLGETPROGRAMBINARYPROC = extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLenum, *mut c_void);
type PFNGLPROGRAMBINARYPROC = extern "system" fn(GLuint, GLenum, *const c_void, GLsizei);
type PFNGLPROGRAMPARAMETERIPROC = extern "system" fn(GLuint, GLenum, GLint);
type PFNGLUSEPROGRAMSTAGESPROC = extern "system" fn(GLuint, GLbitfield, GLuint);
type PFNGLACTIVESHADERPROGRAMPROC = extern "system" fn(GLuint, GLuint);
type PFNGLCREATESHADERPROGRAMVPROC = extern "system" fn(GLenum, GLsizei, *const *const GLchar) -> GLuint;
type PFNGLBINDPROGRAMPIPELINEPROC = extern "system" fn(GLuint);
type PFNGLDELETEPROGRAMPIPELINESPROC = extern "system" fn(GLsizei, *const GLuint);
type PFNGLGENPROGRAMPIPELINESPROC = extern "system" fn(GLsizei, *mut GLuint);
type PFNGLISPROGRAMPIPELINEPROC = extern "system" fn(GLuint) -> GLboolean;
type PFNGLGETPROGRAMPIPELINEIVPROC = extern "system" fn(GLuint, GLenum, *mut GLint);
type PFNGLPROGRAMUNIFORM1IPROC = extern "system" fn(GLuint, GLint, GLint);
type PFNGLPROGRAMUNIFORM1IVPROC = extern "system" fn(GLuint, GLint, GLsizei, *const GLint);
type PFNGLPROGRAMUNIFORM1FPROC = extern "system" fn(GLuint, GLint, GLfloat);
type PFNGLPROGRAMUNIFORM1FVPROC = extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat);
type PFNGLPROGRAMUNIFORM1DPROC = extern "system" fn(GLuint, GLint, GLdouble);
type PFNGLPROGRAMUNIFORM1DVPROC = extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble);
type PFNGLPROGRAMUNIFORM1UIPROC = extern "system" fn(GLuint, GLint, GLuint);
type PFNGLPROGRAMUNIFORM1UIVPROC = extern "system" fn(GLuint, GLint, GLsizei, *const GLuint);
type PFNGLPROGRAMUNIFORM2IPROC = extern "system" fn(GLuint, GLint, GLint, GLint);
type PFNGLPROGRAMUNIFORM2IVPROC = extern "system" fn(GLuint, GLint, GLsizei, *const GLint);
type PFNGLPROGRAMUNIFORM2FPROC = extern "system" fn(GLuint, GLint, GLfloat, GLfloat);
type PFNGLPROGRAMUNIFORM2FVPROC = extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat);
type PFNGLPROGRAMUNIFORM2DPROC = extern "system" fn(GLuint, GLint, GLdouble, GLdouble);
type PFNGLPROGRAMUNIFORM2DVPROC = extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble);
type PFNGLPROGRAMUNIFORM2UIPROC = extern "system" fn(GLuint, GLint, GLuint, GLuint);
type PFNGLPROGRAMUNIFORM2UIVPROC = extern "system" fn(GLuint, GLint, GLsizei, *const GLuint);
type PFNGLPROGRAMUNIFORM3IPROC = extern "system" fn(GLuint, GLint, GLint, GLint, GLint);
type PFNGLPROGRAMUNIFORM3IVPROC = extern "system" fn(GLuint, GLint, GLsizei, *const GLint);
type PFNGLPROGRAMUNIFORM3FPROC = extern "system" fn(GLuint, GLint, GLfloat, GLfloat, GLfloat);
type PFNGLPROGRAMUNIFORM3FVPROC = extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat);
type PFNGLPROGRAMUNIFORM3DPROC = extern "system" fn(GLuint, GLint, GLdouble, GLdouble, GLdouble);
type PFNGLPROGRAMUNIFORM3DVPROC = extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble);
type PFNGLPROGRAMUNIFORM3UIPROC = extern "system" fn(GLuint, GLint, GLuint, GLuint, GLuint);
type PFNGLPROGRAMUNIFORM3UIVPROC = extern "system" fn(GLuint, GLint, GLsizei, *const GLuint);
type PFNGLPROGRAMUNIFORM4IPROC = extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLint);
type PFNGLPROGRAMUNIFORM4IVPROC = extern "system" fn(GLuint, GLint, GLsizei, *const GLint);
type PFNGLPROGRAMUNIFORM4FPROC = extern "system" fn(GLuint, GLint, GLfloat, GLfloat, GLfloat, GLfloat);
type PFNGLPROGRAMUNIFORM4FVPROC = extern "system" fn(GLuint, GLint, GLsizei, *const GLfloat);
type PFNGLPROGRAMUNIFORM4DPROC = extern "system" fn(GLuint, GLint, GLdouble, GLdouble, GLdouble, GLdouble);
type PFNGLPROGRAMUNIFORM4DVPROC = extern "system" fn(GLuint, GLint, GLsizei, *const GLdouble);
type PFNGLPROGRAMUNIFORM4UIPROC = extern "system" fn(GLuint, GLint, GLuint, GLuint, GLuint, GLuint);
type PFNGLPROGRAMUNIFORM4UIVPROC = extern "system" fn(GLuint, GLint, GLsizei, *const GLuint);
type PFNGLPROGRAMUNIFORMMATRIX2FVPROC = extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat);
type PFNGLPROGRAMUNIFORMMATRIX3FVPROC = extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat);
type PFNGLPROGRAMUNIFORMMATRIX4FVPROC = extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat);
type PFNGLPROGRAMUNIFORMMATRIX2DVPROC = extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble);
type PFNGLPROGRAMUNIFORMMATRIX3DVPROC = extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble);
type PFNGLPROGRAMUNIFORMMATRIX4DVPROC = extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble);
type PFNGLPROGRAMUNIFORMMATRIX2X3FVPROC = extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat);
type PFNGLPROGRAMUNIFORMMATRIX3X2FVPROC = extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat);
type PFNGLPROGRAMUNIFORMMATRIX2X4FVPROC = extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat);
type PFNGLPROGRAMUNIFORMMATRIX4X2FVPROC = extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat);
type PFNGLPROGRAMUNIFORMMATRIX3X4FVPROC = extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat);
type PFNGLPROGRAMUNIFORMMATRIX4X3FVPROC = extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLfloat);
type PFNGLPROGRAMUNIFORMMATRIX2X3DVPROC = extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble);
type PFNGLPROGRAMUNIFORMMATRIX3X2DVPROC = extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble);
type PFNGLPROGRAMUNIFORMMATRIX2X4DVPROC = extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble);
type PFNGLPROGRAMUNIFORMMATRIX4X2DVPROC = extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble);
type PFNGLPROGRAMUNIFORMMATRIX3X4DVPROC = extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble);
type PFNGLPROGRAMUNIFORMMATRIX4X3DVPROC = extern "system" fn(GLuint, GLint, GLsizei, GLboolean, *const GLdouble);
type PFNGLVALIDATEPROGRAMPIPELINEPROC = extern "system" fn(GLuint);
type PFNGLGETPROGRAMPIPELINEINFOLOGPROC = extern "system" fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar);
type PFNGLVERTEXATTRIBL1DPROC = extern "system" fn(GLuint, GLdouble);
type PFNGLVERTEXATTRIBL2DPROC = extern "system" fn(GLuint, GLdouble, GLdouble);
type PFNGLVERTEXATTRIBL3DPROC = extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble);
type PFNGLVERTEXATTRIBL4DPROC = extern "system" fn(GLuint, GLdouble, GLdouble, GLdouble, GLdouble);
type PFNGLVERTEXATTRIBL1DVPROC = extern "system" fn(GLuint, *const GLdouble);
type PFNGLVERTEXATTRIBL2DVPROC = extern "system" fn(GLuint, *const GLdouble);
type PFNGLVERTEXATTRIBL3DVPROC = extern "system" fn(GLuint, *const GLdouble);
type PFNGLVERTEXATTRIBL4DVPROC = extern "system" fn(GLuint, *const GLdouble);
type PFNGLVERTEXATTRIBLPOINTERPROC = extern "system" fn(GLuint, GLint, GLenum, GLsizei, *const c_void);
type PFNGLGETVERTEXATTRIBLDVPROC = extern "system" fn(GLuint, GLenum, *mut GLdouble);
type PFNGLVIEWPORTARRAYVPROC = extern "system" fn(GLuint, GLsizei, *const GLfloat);
type PFNGLVIEWPORTINDEXEDFPROC = extern "system" fn(GLuint, GLfloat, GLfloat, GLfloat, GLfloat);
type PFNGLVIEWPORTINDEXEDFVPROC = extern "system" fn(GLuint, *const GLfloat);
type PFNGLSCISSORARRAYVPROC = extern "system" fn(GLuint, GLsizei, *const GLint);
type PFNGLSCISSORINDEXEDPROC = extern "system" fn(GLuint, GLint, GLint, GLsizei, GLsizei);
type PFNGLSCISSORINDEXEDVPROC = extern "system" fn(GLuint, *const GLint);
type PFNGLDEPTHRANGEARRAYVPROC = extern "system" fn(GLuint, GLsizei, *const GLdouble);
type PFNGLDEPTHRANGEINDEXEDPROC = extern "system" fn(GLuint, GLdouble, GLdouble);
type PFNGLGETFLOATI_VPROC = extern "system" fn(GLenum, GLuint, *mut GLfloat);
type PFNGLGETDOUBLEI_VPROC = extern "system" fn(GLenum, GLuint, *mut GLdouble);
const FIXED: GLenum = 0x140C;
const IMPLEMENTATION_COLOR_READ_TYPE: GLenum = 0x8B9A;
const IMPLEMENTATION_COLOR_READ_FORMAT: GLenum = 0x8B9B;
const LOW_FLOAT: GLenum = 0x8DF0;
const MEDIUM_FLOAT: GLenum = 0x8DF1;
const HIGH_FLOAT: GLenum = 0x8DF2;
const LOW_INT: GLenum = 0x8DF3;
const MEDIUM_INT: GLenum = 0x8DF4;
const HIGH_INT: GLenum = 0x8DF5;
const SHADER_COMPILER: GLenum = 0x8DFA;
const SHADER_BINARY_FORMATS: GLenum = 0x8DF8;
const NUM_SHADER_BINARY_FORMATS: GLenum = 0x8DF9;
const MAX_VERTEX_UNIFORM_VECTORS: GLenum = 0x8DFB;
const MAX_VARYING_VECTORS: GLenum = 0x8DFC;
const MAX_FRAGMENT_UNIFORM_VECTORS: GLenum = 0x8DFD;
const RGB565: GLenum = 0x8D62;
const PROGRAM_BINARY_RETRIEVABLE_HINT: GLenum = 0x8257;
const PROGRAM_BINARY_LENGTH: GLenum = 0x8741;
const NUM_PROGRAM_BINARY_FORMATS: GLenum = 0x87FE;
const PROGRAM_BINARY_FORMATS: GLenum = 0x87FF;
const VERTEX_SHADER_BIT: GLbitfield = 0x00000001;
const FRAGMENT_SHADER_BIT: GLbitfield = 0x00000002;
const GEOMETRY_SHADER_BIT: GLbitfield = 0x00000004;
const TESS_CONTROL_SHADER_BIT: GLbitfield = 0x00000008;
const TESS_EVALUATION_SHADER_BIT: GLbitfield = 0x00000010;
const ALL_SHADER_BITS: GLbitfield = 0xFFFFFFFF;
const PROGRAM_SEPARABLE: GLenum = 0x8258;
const ACTIVE_PROGRAM: GLenum = 0x8259;
const PROGRAM_PIPELINE_BINDING: GLenum = 0x825A;
const MAX_VIEWPORTS: GLenum = 0x825B;
const VIEWPORT_SUBPIXEL_BITS: GLenum = 0x825C;
const VIEWPORT_BOUNDS_RANGE: GLenum = 0x825D;
const LAYER_PROVOKING_VERTEX: GLenum = 0x825E;
const VIEWPORT_INDEX_PROVOKING_VERTEX: GLenum = 0x825F;
const UNDEFINED_VERTEX: GLenum = 0x8260;

pub trait GL_4_1 {
	fn glReleaseShaderCompiler(&self) {
		panic!("OpenGL function pointer of `glReleaseShaderCompiler` is NULL");
	}
	fn glShaderBinary(&self, _: GLsizei, _: *const GLuint, _: GLenum, _: *const c_void, _: GLsizei) {
		panic!("OpenGL function pointer of `glShaderBinary` is NULL");
	}
	fn glGetShaderPrecisionFormat(&self, _: GLenum, _: GLenum, _: *mut GLint, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetShaderPrecisionFormat` is NULL");
	}
	fn glDepthRangef(&self, _: GLfloat, _: GLfloat) {
		panic!("OpenGL function pointer of `glDepthRangef` is NULL");
	}
	fn glClearDepthf(&self, _: GLfloat) {
		panic!("OpenGL function pointer of `glClearDepthf` is NULL");
	}
	fn glGetProgramBinary(&self, _: GLuint, _: GLsizei, _: *mut GLsizei, _: *mut GLenum, _: *mut c_void) {
		panic!("OpenGL function pointer of `glGetProgramBinary` is NULL");
	}
	fn glProgramBinary(&self, _: GLuint, _: GLenum, _: *const c_void, _: GLsizei) {
		panic!("OpenGL function pointer of `glProgramBinary` is NULL");
	}
	fn glProgramParameteri(&self, _: GLuint, _: GLenum, _: GLint) {
		panic!("OpenGL function pointer of `glProgramParameteri` is NULL");
	}
	fn glUseProgramStages(&self, _: GLuint, _: GLbitfield, _: GLuint) {
		panic!("OpenGL function pointer of `glUseProgramStages` is NULL");
	}
	fn glActiveShaderProgram(&self, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glActiveShaderProgram` is NULL");
	}
	fn glCreateShaderProgramv(&self, _: GLenum, _: GLsizei, _: *const *const GLchar) -> GLuint {
		panic!("OpenGL function pointer of `glCreateShaderProgramv` is NULL");
	}
	fn glBindProgramPipeline(&self, _: GLuint) {
		panic!("OpenGL function pointer of `glBindProgramPipeline` is NULL");
	}
	fn glDeleteProgramPipelines(&self, _: GLsizei, _: *const GLuint) {
		panic!("OpenGL function pointer of `glDeleteProgramPipelines` is NULL");
	}
	fn glGenProgramPipelines(&self, _: GLsizei, _: *mut GLuint) {
		panic!("OpenGL function pointer of `glGenProgramPipelines` is NULL");
	}
	fn glIsProgramPipeline(&self, _: GLuint) -> GLboolean {
		panic!("OpenGL function pointer of `glIsProgramPipeline` is NULL");
	}
	fn glGetProgramPipelineiv(&self, _: GLuint, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetProgramPipelineiv` is NULL");
	}
	fn glProgramUniform1i(&self, _: GLuint, _: GLint, _: GLint) {
		panic!("OpenGL function pointer of `glProgramUniform1i` is NULL");
	}
	fn glProgramUniform1iv(&self, _: GLuint, _: GLint, _: GLsizei, _: *const GLint) {
		panic!("OpenGL function pointer of `glProgramUniform1iv` is NULL");
	}
	fn glProgramUniform1f(&self, _: GLuint, _: GLint, _: GLfloat) {
		panic!("OpenGL function pointer of `glProgramUniform1f` is NULL");
	}
	fn glProgramUniform1fv(&self, _: GLuint, _: GLint, _: GLsizei, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glProgramUniform1fv` is NULL");
	}
	fn glProgramUniform1d(&self, _: GLuint, _: GLint, _: GLdouble) {
		panic!("OpenGL function pointer of `glProgramUniform1d` is NULL");
	}
	fn glProgramUniform1dv(&self, _: GLuint, _: GLint, _: GLsizei, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glProgramUniform1dv` is NULL");
	}
	fn glProgramUniform1ui(&self, _: GLuint, _: GLint, _: GLuint) {
		panic!("OpenGL function pointer of `glProgramUniform1ui` is NULL");
	}
	fn glProgramUniform1uiv(&self, _: GLuint, _: GLint, _: GLsizei, _: *const GLuint) {
		panic!("OpenGL function pointer of `glProgramUniform1uiv` is NULL");
	}
	fn glProgramUniform2i(&self, _: GLuint, _: GLint, _: GLint, _: GLint) {
		panic!("OpenGL function pointer of `glProgramUniform2i` is NULL");
	}
	fn glProgramUniform2iv(&self, _: GLuint, _: GLint, _: GLsizei, _: *const GLint) {
		panic!("OpenGL function pointer of `glProgramUniform2iv` is NULL");
	}
	fn glProgramUniform2f(&self, _: GLuint, _: GLint, _: GLfloat, _: GLfloat) {
		panic!("OpenGL function pointer of `glProgramUniform2f` is NULL");
	}
	fn glProgramUniform2fv(&self, _: GLuint, _: GLint, _: GLsizei, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glProgramUniform2fv` is NULL");
	}
	fn glProgramUniform2d(&self, _: GLuint, _: GLint, _: GLdouble, _: GLdouble) {
		panic!("OpenGL function pointer of `glProgramUniform2d` is NULL");
	}
	fn glProgramUniform2dv(&self, _: GLuint, _: GLint, _: GLsizei, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glProgramUniform2dv` is NULL");
	}
	fn glProgramUniform2ui(&self, _: GLuint, _: GLint, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glProgramUniform2ui` is NULL");
	}
	fn glProgramUniform2uiv(&self, _: GLuint, _: GLint, _: GLsizei, _: *const GLuint) {
		panic!("OpenGL function pointer of `glProgramUniform2uiv` is NULL");
	}
	fn glProgramUniform3i(&self, _: GLuint, _: GLint, _: GLint, _: GLint, _: GLint) {
		panic!("OpenGL function pointer of `glProgramUniform3i` is NULL");
	}
	fn glProgramUniform3iv(&self, _: GLuint, _: GLint, _: GLsizei, _: *const GLint) {
		panic!("OpenGL function pointer of `glProgramUniform3iv` is NULL");
	}
	fn glProgramUniform3f(&self, _: GLuint, _: GLint, _: GLfloat, _: GLfloat, _: GLfloat) {
		panic!("OpenGL function pointer of `glProgramUniform3f` is NULL");
	}
	fn glProgramUniform3fv(&self, _: GLuint, _: GLint, _: GLsizei, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glProgramUniform3fv` is NULL");
	}
	fn glProgramUniform3d(&self, _: GLuint, _: GLint, _: GLdouble, _: GLdouble, _: GLdouble) {
		panic!("OpenGL function pointer of `glProgramUniform3d` is NULL");
	}
	fn glProgramUniform3dv(&self, _: GLuint, _: GLint, _: GLsizei, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glProgramUniform3dv` is NULL");
	}
	fn glProgramUniform3ui(&self, _: GLuint, _: GLint, _: GLuint, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glProgramUniform3ui` is NULL");
	}
	fn glProgramUniform3uiv(&self, _: GLuint, _: GLint, _: GLsizei, _: *const GLuint) {
		panic!("OpenGL function pointer of `glProgramUniform3uiv` is NULL");
	}
	fn glProgramUniform4i(&self, _: GLuint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint) {
		panic!("OpenGL function pointer of `glProgramUniform4i` is NULL");
	}
	fn glProgramUniform4iv(&self, _: GLuint, _: GLint, _: GLsizei, _: *const GLint) {
		panic!("OpenGL function pointer of `glProgramUniform4iv` is NULL");
	}
	fn glProgramUniform4f(&self, _: GLuint, _: GLint, _: GLfloat, _: GLfloat, _: GLfloat, _: GLfloat) {
		panic!("OpenGL function pointer of `glProgramUniform4f` is NULL");
	}
	fn glProgramUniform4fv(&self, _: GLuint, _: GLint, _: GLsizei, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glProgramUniform4fv` is NULL");
	}
	fn glProgramUniform4d(&self, _: GLuint, _: GLint, _: GLdouble, _: GLdouble, _: GLdouble, _: GLdouble) {
		panic!("OpenGL function pointer of `glProgramUniform4d` is NULL");
	}
	fn glProgramUniform4dv(&self, _: GLuint, _: GLint, _: GLsizei, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glProgramUniform4dv` is NULL");
	}
	fn glProgramUniform4ui(&self, _: GLuint, _: GLint, _: GLuint, _: GLuint, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glProgramUniform4ui` is NULL");
	}
	fn glProgramUniform4uiv(&self, _: GLuint, _: GLint, _: GLsizei, _: *const GLuint) {
		panic!("OpenGL function pointer of `glProgramUniform4uiv` is NULL");
	}
	fn glProgramUniformMatrix2fv(&self, _: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glProgramUniformMatrix2fv` is NULL");
	}
	fn glProgramUniformMatrix3fv(&self, _: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glProgramUniformMatrix3fv` is NULL");
	}
	fn glProgramUniformMatrix4fv(&self, _: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glProgramUniformMatrix4fv` is NULL");
	}
	fn glProgramUniformMatrix2dv(&self, _: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glProgramUniformMatrix2dv` is NULL");
	}
	fn glProgramUniformMatrix3dv(&self, _: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glProgramUniformMatrix3dv` is NULL");
	}
	fn glProgramUniformMatrix4dv(&self, _: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glProgramUniformMatrix4dv` is NULL");
	}
	fn glProgramUniformMatrix2x3fv(&self, _: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glProgramUniformMatrix2x3fv` is NULL");
	}
	fn glProgramUniformMatrix3x2fv(&self, _: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glProgramUniformMatrix3x2fv` is NULL");
	}
	fn glProgramUniformMatrix2x4fv(&self, _: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glProgramUniformMatrix2x4fv` is NULL");
	}
	fn glProgramUniformMatrix4x2fv(&self, _: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glProgramUniformMatrix4x2fv` is NULL");
	}
	fn glProgramUniformMatrix3x4fv(&self, _: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glProgramUniformMatrix3x4fv` is NULL");
	}
	fn glProgramUniformMatrix4x3fv(&self, _: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glProgramUniformMatrix4x3fv` is NULL");
	}
	fn glProgramUniformMatrix2x3dv(&self, _: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glProgramUniformMatrix2x3dv` is NULL");
	}
	fn glProgramUniformMatrix3x2dv(&self, _: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glProgramUniformMatrix3x2dv` is NULL");
	}
	fn glProgramUniformMatrix2x4dv(&self, _: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glProgramUniformMatrix2x4dv` is NULL");
	}
	fn glProgramUniformMatrix4x2dv(&self, _: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glProgramUniformMatrix4x2dv` is NULL");
	}
	fn glProgramUniformMatrix3x4dv(&self, _: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glProgramUniformMatrix3x4dv` is NULL");
	}
	fn glProgramUniformMatrix4x3dv(&self, _: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glProgramUniformMatrix4x3dv` is NULL");
	}
	fn glValidateProgramPipeline(&self, _: GLuint) {
		panic!("OpenGL function pointer of `glValidateProgramPipeline` is NULL");
	}
	fn glGetProgramPipelineInfoLog(&self, _: GLuint, _: GLsizei, _: *mut GLsizei, _: *mut GLchar) {
		panic!("OpenGL function pointer of `glGetProgramPipelineInfoLog` is NULL");
	}
	fn glVertexAttribL1d(&self, _: GLuint, _: GLdouble) {
		panic!("OpenGL function pointer of `glVertexAttribL1d` is NULL");
	}
	fn glVertexAttribL2d(&self, _: GLuint, _: GLdouble, _: GLdouble) {
		panic!("OpenGL function pointer of `glVertexAttribL2d` is NULL");
	}
	fn glVertexAttribL3d(&self, _: GLuint, _: GLdouble, _: GLdouble, _: GLdouble) {
		panic!("OpenGL function pointer of `glVertexAttribL3d` is NULL");
	}
	fn glVertexAttribL4d(&self, _: GLuint, _: GLdouble, _: GLdouble, _: GLdouble, _: GLdouble) {
		panic!("OpenGL function pointer of `glVertexAttribL4d` is NULL");
	}
	fn glVertexAttribL1dv(&self, _: GLuint, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glVertexAttribL1dv` is NULL");
	}
	fn glVertexAttribL2dv(&self, _: GLuint, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glVertexAttribL2dv` is NULL");
	}
	fn glVertexAttribL3dv(&self, _: GLuint, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glVertexAttribL3dv` is NULL");
	}
	fn glVertexAttribL4dv(&self, _: GLuint, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glVertexAttribL4dv` is NULL");
	}
	fn glVertexAttribLPointer(&self, _: GLuint, _: GLint, _: GLenum, _: GLsizei, _: *const c_void) {
		panic!("OpenGL function pointer of `glVertexAttribLPointer` is NULL");
	}
	fn glGetVertexAttribLdv(&self, _: GLuint, _: GLenum, _: *mut GLdouble) {
		panic!("OpenGL function pointer of `glGetVertexAttribLdv` is NULL");
	}
	fn glViewportArrayv(&self, _: GLuint, _: GLsizei, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glViewportArrayv` is NULL");
	}
	fn glViewportIndexedf(&self, _: GLuint, _: GLfloat, _: GLfloat, _: GLfloat, _: GLfloat) {
		panic!("OpenGL function pointer of `glViewportIndexedf` is NULL");
	}
	fn glViewportIndexedfv(&self, _: GLuint, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glViewportIndexedfv` is NULL");
	}
	fn glScissorArrayv(&self, _: GLuint, _: GLsizei, _: *const GLint) {
		panic!("OpenGL function pointer of `glScissorArrayv` is NULL");
	}
	fn glScissorIndexed(&self, _: GLuint, _: GLint, _: GLint, _: GLsizei, _: GLsizei) {
		panic!("OpenGL function pointer of `glScissorIndexed` is NULL");
	}
	fn glScissorIndexedv(&self, _: GLuint, _: *const GLint) {
		panic!("OpenGL function pointer of `glScissorIndexedv` is NULL");
	}
	fn glDepthRangeArrayv(&self, _: GLuint, _: GLsizei, _: *const GLdouble) {
		panic!("OpenGL function pointer of `glDepthRangeArrayv` is NULL");
	}
	fn glDepthRangeIndexed(&self, _: GLuint, _: GLdouble, _: GLdouble) {
		panic!("OpenGL function pointer of `glDepthRangeIndexed` is NULL");
	}
	fn glGetFloati_v(&self, _: GLenum, _: GLuint, _: *mut GLfloat) {
		panic!("OpenGL function pointer of `glGetFloati_v` is NULL");
	}
	fn glGetDoublei_v(&self, _: GLenum, _: GLuint, _: *mut GLdouble) {
		panic!("OpenGL function pointer of `glGetDoublei_v` is NULL");
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Version41 {
	available: bool,
	releaseshadercompiler: PFNGLRELEASESHADERCOMPILERPROC,
	shaderbinary: PFNGLSHADERBINARYPROC,
	getshaderprecisionformat: PFNGLGETSHADERPRECISIONFORMATPROC,
	depthrangef: PFNGLDEPTHRANGEFPROC,
	cleardepthf: PFNGLCLEARDEPTHFPROC,
	getprogrambinary: PFNGLGETPROGRAMBINARYPROC,
	programbinary: PFNGLPROGRAMBINARYPROC,
	programparameteri: PFNGLPROGRAMPARAMETERIPROC,
	useprogramstages: PFNGLUSEPROGRAMSTAGESPROC,
	activeshaderprogram: PFNGLACTIVESHADERPROGRAMPROC,
	createshaderprogramv: PFNGLCREATESHADERPROGRAMVPROC,
	bindprogrampipeline: PFNGLBINDPROGRAMPIPELINEPROC,
	deleteprogrampipelines: PFNGLDELETEPROGRAMPIPELINESPROC,
	genprogrampipelines: PFNGLGENPROGRAMPIPELINESPROC,
	isprogrampipeline: PFNGLISPROGRAMPIPELINEPROC,
	getprogrampipelineiv: PFNGLGETPROGRAMPIPELINEIVPROC,
	programuniform1i: PFNGLPROGRAMUNIFORM1IPROC,
	programuniform1iv: PFNGLPROGRAMUNIFORM1IVPROC,
	programuniform1f: PFNGLPROGRAMUNIFORM1FPROC,
	programuniform1fv: PFNGLPROGRAMUNIFORM1FVPROC,
	programuniform1d: PFNGLPROGRAMUNIFORM1DPROC,
	programuniform1dv: PFNGLPROGRAMUNIFORM1DVPROC,
	programuniform1ui: PFNGLPROGRAMUNIFORM1UIPROC,
	programuniform1uiv: PFNGLPROGRAMUNIFORM1UIVPROC,
	programuniform2i: PFNGLPROGRAMUNIFORM2IPROC,
	programuniform2iv: PFNGLPROGRAMUNIFORM2IVPROC,
	programuniform2f: PFNGLPROGRAMUNIFORM2FPROC,
	programuniform2fv: PFNGLPROGRAMUNIFORM2FVPROC,
	programuniform2d: PFNGLPROGRAMUNIFORM2DPROC,
	programuniform2dv: PFNGLPROGRAMUNIFORM2DVPROC,
	programuniform2ui: PFNGLPROGRAMUNIFORM2UIPROC,
	programuniform2uiv: PFNGLPROGRAMUNIFORM2UIVPROC,
	programuniform3i: PFNGLPROGRAMUNIFORM3IPROC,
	programuniform3iv: PFNGLPROGRAMUNIFORM3IVPROC,
	programuniform3f: PFNGLPROGRAMUNIFORM3FPROC,
	programuniform3fv: PFNGLPROGRAMUNIFORM3FVPROC,
	programuniform3d: PFNGLPROGRAMUNIFORM3DPROC,
	programuniform3dv: PFNGLPROGRAMUNIFORM3DVPROC,
	programuniform3ui: PFNGLPROGRAMUNIFORM3UIPROC,
	programuniform3uiv: PFNGLPROGRAMUNIFORM3UIVPROC,
	programuniform4i: PFNGLPROGRAMUNIFORM4IPROC,
	programuniform4iv: PFNGLPROGRAMUNIFORM4IVPROC,
	programuniform4f: PFNGLPROGRAMUNIFORM4FPROC,
	programuniform4fv: PFNGLPROGRAMUNIFORM4FVPROC,
	programuniform4d: PFNGLPROGRAMUNIFORM4DPROC,
	programuniform4dv: PFNGLPROGRAMUNIFORM4DVPROC,
	programuniform4ui: PFNGLPROGRAMUNIFORM4UIPROC,
	programuniform4uiv: PFNGLPROGRAMUNIFORM4UIVPROC,
	programuniformmatrix2fv: PFNGLPROGRAMUNIFORMMATRIX2FVPROC,
	programuniformmatrix3fv: PFNGLPROGRAMUNIFORMMATRIX3FVPROC,
	programuniformmatrix4fv: PFNGLPROGRAMUNIFORMMATRIX4FVPROC,
	programuniformmatrix2dv: PFNGLPROGRAMUNIFORMMATRIX2DVPROC,
	programuniformmatrix3dv: PFNGLPROGRAMUNIFORMMATRIX3DVPROC,
	programuniformmatrix4dv: PFNGLPROGRAMUNIFORMMATRIX4DVPROC,
	programuniformmatrix2x3fv: PFNGLPROGRAMUNIFORMMATRIX2X3FVPROC,
	programuniformmatrix3x2fv: PFNGLPROGRAMUNIFORMMATRIX3X2FVPROC,
	programuniformmatrix2x4fv: PFNGLPROGRAMUNIFORMMATRIX2X4FVPROC,
	programuniformmatrix4x2fv: PFNGLPROGRAMUNIFORMMATRIX4X2FVPROC,
	programuniformmatrix3x4fv: PFNGLPROGRAMUNIFORMMATRIX3X4FVPROC,
	programuniformmatrix4x3fv: PFNGLPROGRAMUNIFORMMATRIX4X3FVPROC,
	programuniformmatrix2x3dv: PFNGLPROGRAMUNIFORMMATRIX2X3DVPROC,
	programuniformmatrix3x2dv: PFNGLPROGRAMUNIFORMMATRIX3X2DVPROC,
	programuniformmatrix2x4dv: PFNGLPROGRAMUNIFORMMATRIX2X4DVPROC,
	programuniformmatrix4x2dv: PFNGLPROGRAMUNIFORMMATRIX4X2DVPROC,
	programuniformmatrix3x4dv: PFNGLPROGRAMUNIFORMMATRIX3X4DVPROC,
	programuniformmatrix4x3dv: PFNGLPROGRAMUNIFORMMATRIX4X3DVPROC,
	validateprogrampipeline: PFNGLVALIDATEPROGRAMPIPELINEPROC,
	getprogrampipelineinfolog: PFNGLGETPROGRAMPIPELINEINFOLOGPROC,
	vertexattribl1d: PFNGLVERTEXATTRIBL1DPROC,
	vertexattribl2d: PFNGLVERTEXATTRIBL2DPROC,
	vertexattribl3d: PFNGLVERTEXATTRIBL3DPROC,
	vertexattribl4d: PFNGLVERTEXATTRIBL4DPROC,
	vertexattribl1dv: PFNGLVERTEXATTRIBL1DVPROC,
	vertexattribl2dv: PFNGLVERTEXATTRIBL2DVPROC,
	vertexattribl3dv: PFNGLVERTEXATTRIBL3DVPROC,
	vertexattribl4dv: PFNGLVERTEXATTRIBL4DVPROC,
	vertexattriblpointer: PFNGLVERTEXATTRIBLPOINTERPROC,
	getvertexattribldv: PFNGLGETVERTEXATTRIBLDVPROC,
	viewportarrayv: PFNGLVIEWPORTARRAYVPROC,
	viewportindexedf: PFNGLVIEWPORTINDEXEDFPROC,
	viewportindexedfv: PFNGLVIEWPORTINDEXEDFVPROC,
	scissorarrayv: PFNGLSCISSORARRAYVPROC,
	scissorindexed: PFNGLSCISSORINDEXEDPROC,
	scissorindexedv: PFNGLSCISSORINDEXEDVPROC,
	depthrangearrayv: PFNGLDEPTHRANGEARRAYVPROC,
	depthrangeindexed: PFNGLDEPTHRANGEINDEXEDPROC,
	getfloati_v: PFNGLGETFLOATI_VPROC,
	getdoublei_v: PFNGLGETDOUBLEI_VPROC,
}

impl GL_4_1 for Version41 {
	#[inline(always)]
	fn glReleaseShaderCompiler(&self) {
		(self.releaseshadercompiler)()
	}
	#[inline(always)]
	fn glShaderBinary(&self, count: GLsizei, shaders: *const GLuint, binaryFormat: GLenum, binary: *const c_void, length: GLsizei) {
		(self.shaderbinary)(count, shaders, binaryFormat, binary, length)
	}
	#[inline(always)]
	fn glGetShaderPrecisionFormat(&self, shadertype: GLenum, precisiontype: GLenum, range: *mut GLint, precision: *mut GLint) {
		(self.getshaderprecisionformat)(shadertype, precisiontype, range, precision)
	}
	#[inline(always)]
	fn glDepthRangef(&self, n: GLfloat, f: GLfloat) {
		(self.depthrangef)(n, f)
	}
	#[inline(always)]
	fn glClearDepthf(&self, d: GLfloat) {
		(self.cleardepthf)(d)
	}
	#[inline(always)]
	fn glGetProgramBinary(&self, program: GLuint, bufSize: GLsizei, length: *mut GLsizei, binaryFormat: *mut GLenum, binary: *mut c_void) {
		(self.getprogrambinary)(program, bufSize, length, binaryFormat, binary)
	}
	#[inline(always)]
	fn glProgramBinary(&self, program: GLuint, binaryFormat: GLenum, binary: *const c_void, length: GLsizei) {
		(self.programbinary)(program, binaryFormat, binary, length)
	}
	#[inline(always)]
	fn glProgramParameteri(&self, program: GLuint, pname: GLenum, value: GLint) {
		(self.programparameteri)(program, pname, value)
	}
	#[inline(always)]
	fn glUseProgramStages(&self, pipeline: GLuint, stages: GLbitfield, program: GLuint) {
		(self.useprogramstages)(pipeline, stages, program)
	}
	#[inline(always)]
	fn glActiveShaderProgram(&self, pipeline: GLuint, program: GLuint) {
		(self.activeshaderprogram)(pipeline, program)
	}
	#[inline(always)]
	fn glCreateShaderProgramv(&self, type_: GLenum, count: GLsizei, strings: *const *const GLchar) -> GLuint {
		(self.createshaderprogramv)(type_, count, strings)
	}
	#[inline(always)]
	fn glBindProgramPipeline(&self, pipeline: GLuint) {
		(self.bindprogrampipeline)(pipeline)
	}
	#[inline(always)]
	fn glDeleteProgramPipelines(&self, n: GLsizei, pipelines: *const GLuint) {
		(self.deleteprogrampipelines)(n, pipelines)
	}
	#[inline(always)]
	fn glGenProgramPipelines(&self, n: GLsizei, pipelines: *mut GLuint) {
		(self.genprogrampipelines)(n, pipelines)
	}
	#[inline(always)]
	fn glIsProgramPipeline(&self, pipeline: GLuint) -> GLboolean {
		(self.isprogrampipeline)(pipeline)
	}
	#[inline(always)]
	fn glGetProgramPipelineiv(&self, pipeline: GLuint, pname: GLenum, params: *mut GLint) {
		(self.getprogrampipelineiv)(pipeline, pname, params)
	}
	#[inline(always)]
	fn glProgramUniform1i(&self, program: GLuint, location: GLint, v0: GLint) {
		(self.programuniform1i)(program, location, v0)
	}
	#[inline(always)]
	fn glProgramUniform1iv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLint) {
		(self.programuniform1iv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniform1f(&self, program: GLuint, location: GLint, v0: GLfloat) {
		(self.programuniform1f)(program, location, v0)
	}
	#[inline(always)]
	fn glProgramUniform1fv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) {
		(self.programuniform1fv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniform1d(&self, program: GLuint, location: GLint, v0: GLdouble) {
		(self.programuniform1d)(program, location, v0)
	}
	#[inline(always)]
	fn glProgramUniform1dv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) {
		(self.programuniform1dv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniform1ui(&self, program: GLuint, location: GLint, v0: GLuint) {
		(self.programuniform1ui)(program, location, v0)
	}
	#[inline(always)]
	fn glProgramUniform1uiv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) {
		(self.programuniform1uiv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniform2i(&self, program: GLuint, location: GLint, v0: GLint, v1: GLint) {
		(self.programuniform2i)(program, location, v0, v1)
	}
	#[inline(always)]
	fn glProgramUniform2iv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLint) {
		(self.programuniform2iv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniform2f(&self, program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat) {
		(self.programuniform2f)(program, location, v0, v1)
	}
	#[inline(always)]
	fn glProgramUniform2fv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) {
		(self.programuniform2fv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniform2d(&self, program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble) {
		(self.programuniform2d)(program, location, v0, v1)
	}
	#[inline(always)]
	fn glProgramUniform2dv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) {
		(self.programuniform2dv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniform2ui(&self, program: GLuint, location: GLint, v0: GLuint, v1: GLuint) {
		(self.programuniform2ui)(program, location, v0, v1)
	}
	#[inline(always)]
	fn glProgramUniform2uiv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) {
		(self.programuniform2uiv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniform3i(&self, program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint) {
		(self.programuniform3i)(program, location, v0, v1, v2)
	}
	#[inline(always)]
	fn glProgramUniform3iv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLint) {
		(self.programuniform3iv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniform3f(&self, program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) {
		(self.programuniform3f)(program, location, v0, v1, v2)
	}
	#[inline(always)]
	fn glProgramUniform3fv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) {
		(self.programuniform3fv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniform3d(&self, program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble) {
		(self.programuniform3d)(program, location, v0, v1, v2)
	}
	#[inline(always)]
	fn glProgramUniform3dv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) {
		(self.programuniform3dv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniform3ui(&self, program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) {
		(self.programuniform3ui)(program, location, v0, v1, v2)
	}
	#[inline(always)]
	fn glProgramUniform3uiv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) {
		(self.programuniform3uiv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniform4i(&self, program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) {
		(self.programuniform4i)(program, location, v0, v1, v2, v3)
	}
	#[inline(always)]
	fn glProgramUniform4iv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLint) {
		(self.programuniform4iv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniform4f(&self, program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) {
		(self.programuniform4f)(program, location, v0, v1, v2, v3)
	}
	#[inline(always)]
	fn glProgramUniform4fv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) {
		(self.programuniform4fv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniform4d(&self, program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble, v3: GLdouble) {
		(self.programuniform4d)(program, location, v0, v1, v2, v3)
	}
	#[inline(always)]
	fn glProgramUniform4dv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) {
		(self.programuniform4dv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniform4ui(&self, program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) {
		(self.programuniform4ui)(program, location, v0, v1, v2, v3)
	}
	#[inline(always)]
	fn glProgramUniform4uiv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) {
		(self.programuniform4uiv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix2fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.programuniformmatrix2fv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix3fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.programuniformmatrix3fv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix4fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.programuniformmatrix4fv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix2dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.programuniformmatrix2dv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix3dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.programuniformmatrix3dv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix4dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.programuniformmatrix4dv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix2x3fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.programuniformmatrix2x3fv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix3x2fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.programuniformmatrix3x2fv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix2x4fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.programuniformmatrix2x4fv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix4x2fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.programuniformmatrix4x2fv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix3x4fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.programuniformmatrix3x4fv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix4x3fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.programuniformmatrix4x3fv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix2x3dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.programuniformmatrix2x3dv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix3x2dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.programuniformmatrix3x2dv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix2x4dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.programuniformmatrix2x4dv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix4x2dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.programuniformmatrix4x2dv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix3x4dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.programuniformmatrix3x4dv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix4x3dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.programuniformmatrix4x3dv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glValidateProgramPipeline(&self, pipeline: GLuint) {
		(self.validateprogrampipeline)(pipeline)
	}
	#[inline(always)]
	fn glGetProgramPipelineInfoLog(&self, pipeline: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) {
		(self.getprogrampipelineinfolog)(pipeline, bufSize, length, infoLog)
	}
	#[inline(always)]
	fn glVertexAttribL1d(&self, index: GLuint, x: GLdouble) {
		(self.vertexattribl1d)(index, x)
	}
	#[inline(always)]
	fn glVertexAttribL2d(&self, index: GLuint, x: GLdouble, y: GLdouble) {
		(self.vertexattribl2d)(index, x, y)
	}
	#[inline(always)]
	fn glVertexAttribL3d(&self, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) {
		(self.vertexattribl3d)(index, x, y, z)
	}
	#[inline(always)]
	fn glVertexAttribL4d(&self, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) {
		(self.vertexattribl4d)(index, x, y, z, w)
	}
	#[inline(always)]
	fn glVertexAttribL1dv(&self, index: GLuint, v: *const GLdouble) {
		(self.vertexattribl1dv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribL2dv(&self, index: GLuint, v: *const GLdouble) {
		(self.vertexattribl2dv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribL3dv(&self, index: GLuint, v: *const GLdouble) {
		(self.vertexattribl3dv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribL4dv(&self, index: GLuint, v: *const GLdouble) {
		(self.vertexattribl4dv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribLPointer(&self, index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void) {
		(self.vertexattriblpointer)(index, size, type_, stride, pointer)
	}
	#[inline(always)]
	fn glGetVertexAttribLdv(&self, index: GLuint, pname: GLenum, params: *mut GLdouble) {
		(self.getvertexattribldv)(index, pname, params)
	}
	#[inline(always)]
	fn glViewportArrayv(&self, first: GLuint, count: GLsizei, v: *const GLfloat) {
		(self.viewportarrayv)(first, count, v)
	}
	#[inline(always)]
	fn glViewportIndexedf(&self, index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat) {
		(self.viewportindexedf)(index, x, y, w, h)
	}
	#[inline(always)]
	fn glViewportIndexedfv(&self, index: GLuint, v: *const GLfloat) {
		(self.viewportindexedfv)(index, v)
	}
	#[inline(always)]
	fn glScissorArrayv(&self, first: GLuint, count: GLsizei, v: *const GLint) {
		(self.scissorarrayv)(first, count, v)
	}
	#[inline(always)]
	fn glScissorIndexed(&self, index: GLuint, left: GLint, bottom: GLint, width: GLsizei, height: GLsizei) {
		(self.scissorindexed)(index, left, bottom, width, height)
	}
	#[inline(always)]
	fn glScissorIndexedv(&self, index: GLuint, v: *const GLint) {
		(self.scissorindexedv)(index, v)
	}
	#[inline(always)]
	fn glDepthRangeArrayv(&self, first: GLuint, count: GLsizei, v: *const GLdouble) {
		(self.depthrangearrayv)(first, count, v)
	}
	#[inline(always)]
	fn glDepthRangeIndexed(&self, index: GLuint, n: GLdouble, f: GLdouble) {
		(self.depthrangeindexed)(index, n, f)
	}
	#[inline(always)]
	fn glGetFloati_v(&self, target: GLenum, index: GLuint, data: *mut GLfloat) {
		(self.getfloati_v)(target, index, data)
	}
	#[inline(always)]
	fn glGetDoublei_v(&self, target: GLenum, index: GLuint, data: *mut GLdouble) {
		(self.getdoublei_v)(target, index, data)
	}
}

impl Version41 {
	pub fn new(base: impl GL_1_0, get_proc_address: &impl Fn(&'static str) -> *const c_void) -> Self {
		let (spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 40100 {
			return Self::default();
		}
		Self {
			available: true,
			releaseshadercompiler: unsafe{transmute(get_proc_address("glReleaseShaderCompiler"))},
			shaderbinary: unsafe{transmute(get_proc_address("glShaderBinary"))},
			getshaderprecisionformat: unsafe{transmute(get_proc_address("glGetShaderPrecisionFormat"))},
			depthrangef: unsafe{transmute(get_proc_address("glDepthRangef"))},
			cleardepthf: unsafe{transmute(get_proc_address("glClearDepthf"))},
			getprogrambinary: unsafe{transmute(get_proc_address("glGetProgramBinary"))},
			programbinary: unsafe{transmute(get_proc_address("glProgramBinary"))},
			programparameteri: unsafe{transmute(get_proc_address("glProgramParameteri"))},
			useprogramstages: unsafe{transmute(get_proc_address("glUseProgramStages"))},
			activeshaderprogram: unsafe{transmute(get_proc_address("glActiveShaderProgram"))},
			createshaderprogramv: unsafe{transmute(get_proc_address("glCreateShaderProgramv"))},
			bindprogrampipeline: unsafe{transmute(get_proc_address("glBindProgramPipeline"))},
			deleteprogrampipelines: unsafe{transmute(get_proc_address("glDeleteProgramPipelines"))},
			genprogrampipelines: unsafe{transmute(get_proc_address("glGenProgramPipelines"))},
			isprogrampipeline: unsafe{transmute(get_proc_address("glIsProgramPipeline"))},
			getprogrampipelineiv: unsafe{transmute(get_proc_address("glGetProgramPipelineiv"))},
			programuniform1i: unsafe{transmute(get_proc_address("glProgramUniform1i"))},
			programuniform1iv: unsafe{transmute(get_proc_address("glProgramUniform1iv"))},
			programuniform1f: unsafe{transmute(get_proc_address("glProgramUniform1f"))},
			programuniform1fv: unsafe{transmute(get_proc_address("glProgramUniform1fv"))},
			programuniform1d: unsafe{transmute(get_proc_address("glProgramUniform1d"))},
			programuniform1dv: unsafe{transmute(get_proc_address("glProgramUniform1dv"))},
			programuniform1ui: unsafe{transmute(get_proc_address("glProgramUniform1ui"))},
			programuniform1uiv: unsafe{transmute(get_proc_address("glProgramUniform1uiv"))},
			programuniform2i: unsafe{transmute(get_proc_address("glProgramUniform2i"))},
			programuniform2iv: unsafe{transmute(get_proc_address("glProgramUniform2iv"))},
			programuniform2f: unsafe{transmute(get_proc_address("glProgramUniform2f"))},
			programuniform2fv: unsafe{transmute(get_proc_address("glProgramUniform2fv"))},
			programuniform2d: unsafe{transmute(get_proc_address("glProgramUniform2d"))},
			programuniform2dv: unsafe{transmute(get_proc_address("glProgramUniform2dv"))},
			programuniform2ui: unsafe{transmute(get_proc_address("glProgramUniform2ui"))},
			programuniform2uiv: unsafe{transmute(get_proc_address("glProgramUniform2uiv"))},
			programuniform3i: unsafe{transmute(get_proc_address("glProgramUniform3i"))},
			programuniform3iv: unsafe{transmute(get_proc_address("glProgramUniform3iv"))},
			programuniform3f: unsafe{transmute(get_proc_address("glProgramUniform3f"))},
			programuniform3fv: unsafe{transmute(get_proc_address("glProgramUniform3fv"))},
			programuniform3d: unsafe{transmute(get_proc_address("glProgramUniform3d"))},
			programuniform3dv: unsafe{transmute(get_proc_address("glProgramUniform3dv"))},
			programuniform3ui: unsafe{transmute(get_proc_address("glProgramUniform3ui"))},
			programuniform3uiv: unsafe{transmute(get_proc_address("glProgramUniform3uiv"))},
			programuniform4i: unsafe{transmute(get_proc_address("glProgramUniform4i"))},
			programuniform4iv: unsafe{transmute(get_proc_address("glProgramUniform4iv"))},
			programuniform4f: unsafe{transmute(get_proc_address("glProgramUniform4f"))},
			programuniform4fv: unsafe{transmute(get_proc_address("glProgramUniform4fv"))},
			programuniform4d: unsafe{transmute(get_proc_address("glProgramUniform4d"))},
			programuniform4dv: unsafe{transmute(get_proc_address("glProgramUniform4dv"))},
			programuniform4ui: unsafe{transmute(get_proc_address("glProgramUniform4ui"))},
			programuniform4uiv: unsafe{transmute(get_proc_address("glProgramUniform4uiv"))},
			programuniformmatrix2fv: unsafe{transmute(get_proc_address("glProgramUniformMatrix2fv"))},
			programuniformmatrix3fv: unsafe{transmute(get_proc_address("glProgramUniformMatrix3fv"))},
			programuniformmatrix4fv: unsafe{transmute(get_proc_address("glProgramUniformMatrix4fv"))},
			programuniformmatrix2dv: unsafe{transmute(get_proc_address("glProgramUniformMatrix2dv"))},
			programuniformmatrix3dv: unsafe{transmute(get_proc_address("glProgramUniformMatrix3dv"))},
			programuniformmatrix4dv: unsafe{transmute(get_proc_address("glProgramUniformMatrix4dv"))},
			programuniformmatrix2x3fv: unsafe{transmute(get_proc_address("glProgramUniformMatrix2x3fv"))},
			programuniformmatrix3x2fv: unsafe{transmute(get_proc_address("glProgramUniformMatrix3x2fv"))},
			programuniformmatrix2x4fv: unsafe{transmute(get_proc_address("glProgramUniformMatrix2x4fv"))},
			programuniformmatrix4x2fv: unsafe{transmute(get_proc_address("glProgramUniformMatrix4x2fv"))},
			programuniformmatrix3x4fv: unsafe{transmute(get_proc_address("glProgramUniformMatrix3x4fv"))},
			programuniformmatrix4x3fv: unsafe{transmute(get_proc_address("glProgramUniformMatrix4x3fv"))},
			programuniformmatrix2x3dv: unsafe{transmute(get_proc_address("glProgramUniformMatrix2x3dv"))},
			programuniformmatrix3x2dv: unsafe{transmute(get_proc_address("glProgramUniformMatrix3x2dv"))},
			programuniformmatrix2x4dv: unsafe{transmute(get_proc_address("glProgramUniformMatrix2x4dv"))},
			programuniformmatrix4x2dv: unsafe{transmute(get_proc_address("glProgramUniformMatrix4x2dv"))},
			programuniformmatrix3x4dv: unsafe{transmute(get_proc_address("glProgramUniformMatrix3x4dv"))},
			programuniformmatrix4x3dv: unsafe{transmute(get_proc_address("glProgramUniformMatrix4x3dv"))},
			validateprogrampipeline: unsafe{transmute(get_proc_address("glValidateProgramPipeline"))},
			getprogrampipelineinfolog: unsafe{transmute(get_proc_address("glGetProgramPipelineInfoLog"))},
			vertexattribl1d: unsafe{transmute(get_proc_address("glVertexAttribL1d"))},
			vertexattribl2d: unsafe{transmute(get_proc_address("glVertexAttribL2d"))},
			vertexattribl3d: unsafe{transmute(get_proc_address("glVertexAttribL3d"))},
			vertexattribl4d: unsafe{transmute(get_proc_address("glVertexAttribL4d"))},
			vertexattribl1dv: unsafe{transmute(get_proc_address("glVertexAttribL1dv"))},
			vertexattribl2dv: unsafe{transmute(get_proc_address("glVertexAttribL2dv"))},
			vertexattribl3dv: unsafe{transmute(get_proc_address("glVertexAttribL3dv"))},
			vertexattribl4dv: unsafe{transmute(get_proc_address("glVertexAttribL4dv"))},
			vertexattriblpointer: unsafe{transmute(get_proc_address("glVertexAttribLPointer"))},
			getvertexattribldv: unsafe{transmute(get_proc_address("glGetVertexAttribLdv"))},
			viewportarrayv: unsafe{transmute(get_proc_address("glViewportArrayv"))},
			viewportindexedf: unsafe{transmute(get_proc_address("glViewportIndexedf"))},
			viewportindexedfv: unsafe{transmute(get_proc_address("glViewportIndexedfv"))},
			scissorarrayv: unsafe{transmute(get_proc_address("glScissorArrayv"))},
			scissorindexed: unsafe{transmute(get_proc_address("glScissorIndexed"))},
			scissorindexedv: unsafe{transmute(get_proc_address("glScissorIndexedv"))},
			depthrangearrayv: unsafe{transmute(get_proc_address("glDepthRangeArrayv"))},
			depthrangeindexed: unsafe{transmute(get_proc_address("glDepthRangeIndexed"))},
			getfloati_v: unsafe{transmute(get_proc_address("glGetFloati_v"))},
			getdoublei_v: unsafe{transmute(get_proc_address("glGetDoublei_v"))},
		}
	}
	#[inline(always)]
	pub fn get_available(&self) -> bool {
		self.available
	}
}

impl Default for Version41 {
	fn default() -> Self {
		Self {
			available: false,
			releaseshadercompiler: null(),
			shaderbinary: null(),
			getshaderprecisionformat: null(),
			depthrangef: null(),
			cleardepthf: null(),
			getprogrambinary: null(),
			programbinary: null(),
			programparameteri: null(),
			useprogramstages: null(),
			activeshaderprogram: null(),
			createshaderprogramv: null(),
			bindprogrampipeline: null(),
			deleteprogrampipelines: null(),
			genprogrampipelines: null(),
			isprogrampipeline: null(),
			getprogrampipelineiv: null(),
			programuniform1i: null(),
			programuniform1iv: null(),
			programuniform1f: null(),
			programuniform1fv: null(),
			programuniform1d: null(),
			programuniform1dv: null(),
			programuniform1ui: null(),
			programuniform1uiv: null(),
			programuniform2i: null(),
			programuniform2iv: null(),
			programuniform2f: null(),
			programuniform2fv: null(),
			programuniform2d: null(),
			programuniform2dv: null(),
			programuniform2ui: null(),
			programuniform2uiv: null(),
			programuniform3i: null(),
			programuniform3iv: null(),
			programuniform3f: null(),
			programuniform3fv: null(),
			programuniform3d: null(),
			programuniform3dv: null(),
			programuniform3ui: null(),
			programuniform3uiv: null(),
			programuniform4i: null(),
			programuniform4iv: null(),
			programuniform4f: null(),
			programuniform4fv: null(),
			programuniform4d: null(),
			programuniform4dv: null(),
			programuniform4ui: null(),
			programuniform4uiv: null(),
			programuniformmatrix2fv: null(),
			programuniformmatrix3fv: null(),
			programuniformmatrix4fv: null(),
			programuniformmatrix2dv: null(),
			programuniformmatrix3dv: null(),
			programuniformmatrix4dv: null(),
			programuniformmatrix2x3fv: null(),
			programuniformmatrix3x2fv: null(),
			programuniformmatrix2x4fv: null(),
			programuniformmatrix4x2fv: null(),
			programuniformmatrix3x4fv: null(),
			programuniformmatrix4x3fv: null(),
			programuniformmatrix2x3dv: null(),
			programuniformmatrix3x2dv: null(),
			programuniformmatrix2x4dv: null(),
			programuniformmatrix4x2dv: null(),
			programuniformmatrix3x4dv: null(),
			programuniformmatrix4x3dv: null(),
			validateprogrampipeline: null(),
			getprogrampipelineinfolog: null(),
			vertexattribl1d: null(),
			vertexattribl2d: null(),
			vertexattribl3d: null(),
			vertexattribl4d: null(),
			vertexattribl1dv: null(),
			vertexattribl2dv: null(),
			vertexattribl3dv: null(),
			vertexattribl4dv: null(),
			vertexattriblpointer: null(),
			getvertexattribldv: null(),
			viewportarrayv: null(),
			viewportindexedf: null(),
			viewportindexedfv: null(),
			scissorarrayv: null(),
			scissorindexed: null(),
			scissorindexedv: null(),
			depthrangearrayv: null(),
			depthrangeindexed: null(),
			getfloati_v: null(),
			getdoublei_v: null(),
		}
	}
}

type PFNGLDRAWARRAYSINSTANCEDBASEINSTANCEPROC = extern "system" fn(GLenum, GLint, GLsizei, GLsizei, GLuint);
type PFNGLDRAWELEMENTSINSTANCEDBASEINSTANCEPROC = extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei, GLuint);
type PFNGLDRAWELEMENTSINSTANCEDBASEVERTEXBASEINSTANCEPROC = extern "system" fn(GLenum, GLsizei, GLenum, *const c_void, GLsizei, GLint, GLuint);
type PFNGLGETINTERNALFORMATIVPROC = extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut GLint);
type PFNGLGETACTIVEATOMICCOUNTERBUFFERIVPROC = extern "system" fn(GLuint, GLuint, GLenum, *mut GLint);
type PFNGLBINDIMAGETEXTUREPROC = extern "system" fn(GLuint, GLuint, GLint, GLboolean, GLint, GLenum, GLenum);
type PFNGLMEMORYBARRIERPROC = extern "system" fn(GLbitfield);
type PFNGLTEXSTORAGE1DPROC = extern "system" fn(GLenum, GLsizei, GLenum, GLsizei);
type PFNGLTEXSTORAGE2DPROC = extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei);
type PFNGLTEXSTORAGE3DPROC = extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei);
type PFNGLDRAWTRANSFORMFEEDBACKINSTANCEDPROC = extern "system" fn(GLenum, GLuint, GLsizei);
type PFNGLDRAWTRANSFORMFEEDBACKSTREAMINSTANCEDPROC = extern "system" fn(GLenum, GLuint, GLuint, GLsizei);
const COPY_READ_BUFFER_BINDING: GLenum = 0x8F36;
const COPY_WRITE_BUFFER_BINDING: GLenum = 0x8F37;
const TRANSFORM_FEEDBACK_ACTIVE: GLenum = 0x8E24;
const TRANSFORM_FEEDBACK_PAUSED: GLenum = 0x8E23;
const UNPACK_COMPRESSED_BLOCK_WIDTH: GLenum = 0x9127;
const UNPACK_COMPRESSED_BLOCK_HEIGHT: GLenum = 0x9128;
const UNPACK_COMPRESSED_BLOCK_DEPTH: GLenum = 0x9129;
const UNPACK_COMPRESSED_BLOCK_SIZE: GLenum = 0x912A;
const PACK_COMPRESSED_BLOCK_WIDTH: GLenum = 0x912B;
const PACK_COMPRESSED_BLOCK_HEIGHT: GLenum = 0x912C;
const PACK_COMPRESSED_BLOCK_DEPTH: GLenum = 0x912D;
const PACK_COMPRESSED_BLOCK_SIZE: GLenum = 0x912E;
const NUM_SAMPLE_COUNTS: GLenum = 0x9380;
const MIN_MAP_BUFFER_ALIGNMENT: GLenum = 0x90BC;
const ATOMIC_COUNTER_BUFFER: GLenum = 0x92C0;
const ATOMIC_COUNTER_BUFFER_BINDING: GLenum = 0x92C1;
const ATOMIC_COUNTER_BUFFER_START: GLenum = 0x92C2;
const ATOMIC_COUNTER_BUFFER_SIZE: GLenum = 0x92C3;
const ATOMIC_COUNTER_BUFFER_DATA_SIZE: GLenum = 0x92C4;
const ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS: GLenum = 0x92C5;
const ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES: GLenum = 0x92C6;
const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x92C7;
const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x92C8;
const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x92C9;
const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x92CA;
const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x92CB;
const MAX_VERTEX_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CC;
const MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CD;
const MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CE;
const MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CF;
const MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D0;
const MAX_COMBINED_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D1;
const MAX_VERTEX_ATOMIC_COUNTERS: GLenum = 0x92D2;
const MAX_TESS_CONTROL_ATOMIC_COUNTERS: GLenum = 0x92D3;
const MAX_TESS_EVALUATION_ATOMIC_COUNTERS: GLenum = 0x92D4;
const MAX_GEOMETRY_ATOMIC_COUNTERS: GLenum = 0x92D5;
const MAX_FRAGMENT_ATOMIC_COUNTERS: GLenum = 0x92D6;
const MAX_COMBINED_ATOMIC_COUNTERS: GLenum = 0x92D7;
const MAX_ATOMIC_COUNTER_BUFFER_SIZE: GLenum = 0x92D8;
const MAX_ATOMIC_COUNTER_BUFFER_BINDINGS: GLenum = 0x92DC;
const ACTIVE_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D9;
const UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX: GLenum = 0x92DA;
const UNSIGNED_INT_ATOMIC_COUNTER: GLenum = 0x92DB;
const VERTEX_ATTRIB_ARRAY_BARRIER_BIT: GLbitfield = 0x00000001;
const ELEMENT_ARRAY_BARRIER_BIT: GLbitfield = 0x00000002;
const UNIFORM_BARRIER_BIT: GLbitfield = 0x00000004;
const TEXTURE_FETCH_BARRIER_BIT: GLbitfield = 0x00000008;
const SHADER_IMAGE_ACCESS_BARRIER_BIT: GLbitfield = 0x00000020;
const COMMAND_BARRIER_BIT: GLbitfield = 0x00000040;
const PIXEL_BUFFER_BARRIER_BIT: GLbitfield = 0x00000080;
const TEXTURE_UPDATE_BARRIER_BIT: GLbitfield = 0x00000100;
const BUFFER_UPDATE_BARRIER_BIT: GLbitfield = 0x00000200;
const FRAMEBUFFER_BARRIER_BIT: GLbitfield = 0x00000400;
const TRANSFORM_FEEDBACK_BARRIER_BIT: GLbitfield = 0x00000800;
const ATOMIC_COUNTER_BARRIER_BIT: GLbitfield = 0x00001000;
const ALL_BARRIER_BITS: GLbitfield = 0xFFFFFFFF;
const MAX_IMAGE_UNITS: GLenum = 0x8F38;
const MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS: GLenum = 0x8F39;
const IMAGE_BINDING_NAME: GLenum = 0x8F3A;
const IMAGE_BINDING_LEVEL: GLenum = 0x8F3B;
const IMAGE_BINDING_LAYERED: GLenum = 0x8F3C;
const IMAGE_BINDING_LAYER: GLenum = 0x8F3D;
const IMAGE_BINDING_ACCESS: GLenum = 0x8F3E;
const IMAGE_1D: GLenum = 0x904C;
const IMAGE_2D: GLenum = 0x904D;
const IMAGE_3D: GLenum = 0x904E;
const IMAGE_2D_RECT: GLenum = 0x904F;
const IMAGE_CUBE: GLenum = 0x9050;
const IMAGE_BUFFER: GLenum = 0x9051;
const IMAGE_1D_ARRAY: GLenum = 0x9052;
const IMAGE_2D_ARRAY: GLenum = 0x9053;
const IMAGE_CUBE_MAP_ARRAY: GLenum = 0x9054;
const IMAGE_2D_MULTISAMPLE: GLenum = 0x9055;
const IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9056;
const INT_IMAGE_1D: GLenum = 0x9057;
const INT_IMAGE_2D: GLenum = 0x9058;
const INT_IMAGE_3D: GLenum = 0x9059;
const INT_IMAGE_2D_RECT: GLenum = 0x905A;
const INT_IMAGE_CUBE: GLenum = 0x905B;
const INT_IMAGE_BUFFER: GLenum = 0x905C;
const INT_IMAGE_1D_ARRAY: GLenum = 0x905D;
const INT_IMAGE_2D_ARRAY: GLenum = 0x905E;
const INT_IMAGE_CUBE_MAP_ARRAY: GLenum = 0x905F;
const INT_IMAGE_2D_MULTISAMPLE: GLenum = 0x9060;
const INT_IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9061;
const UNSIGNED_INT_IMAGE_1D: GLenum = 0x9062;
const UNSIGNED_INT_IMAGE_2D: GLenum = 0x9063;
const UNSIGNED_INT_IMAGE_3D: GLenum = 0x9064;
const UNSIGNED_INT_IMAGE_2D_RECT: GLenum = 0x9065;
const UNSIGNED_INT_IMAGE_CUBE: GLenum = 0x9066;
const UNSIGNED_INT_IMAGE_BUFFER: GLenum = 0x9067;
const UNSIGNED_INT_IMAGE_1D_ARRAY: GLenum = 0x9068;
const UNSIGNED_INT_IMAGE_2D_ARRAY: GLenum = 0x9069;
const UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY: GLenum = 0x906A;
const UNSIGNED_INT_IMAGE_2D_MULTISAMPLE: GLenum = 0x906B;
const UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 0x906C;
const MAX_IMAGE_SAMPLES: GLenum = 0x906D;
const IMAGE_BINDING_FORMAT: GLenum = 0x906E;
const IMAGE_FORMAT_COMPATIBILITY_TYPE: GLenum = 0x90C7;
const IMAGE_FORMAT_COMPATIBILITY_BY_SIZE: GLenum = 0x90C8;
const IMAGE_FORMAT_COMPATIBILITY_BY_CLASS: GLenum = 0x90C9;
const MAX_VERTEX_IMAGE_UNIFORMS: GLenum = 0x90CA;
const MAX_TESS_CONTROL_IMAGE_UNIFORMS: GLenum = 0x90CB;
const MAX_TESS_EVALUATION_IMAGE_UNIFORMS: GLenum = 0x90CC;
const MAX_GEOMETRY_IMAGE_UNIFORMS: GLenum = 0x90CD;
const MAX_FRAGMENT_IMAGE_UNIFORMS: GLenum = 0x90CE;
const MAX_COMBINED_IMAGE_UNIFORMS: GLenum = 0x90CF;
const COMPRESSED_RGBA_BPTC_UNORM: GLenum = 0x8E8C;
const COMPRESSED_SRGB_ALPHA_BPTC_UNORM: GLenum = 0x8E8D;
const COMPRESSED_RGB_BPTC_SIGNED_FLOAT: GLenum = 0x8E8E;
const COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT: GLenum = 0x8E8F;
const TEXTURE_IMMUTABLE_FORMAT: GLenum = 0x912F;

pub trait GL_4_2 {
	fn glDrawArraysInstancedBaseInstance(&self, _: GLenum, _: GLint, _: GLsizei, _: GLsizei, _: GLuint) {
		panic!("OpenGL function pointer of `glDrawArraysInstancedBaseInstance` is NULL");
	}
	fn glDrawElementsInstancedBaseInstance(&self, _: GLenum, _: GLsizei, _: GLenum, _: *const c_void, _: GLsizei, _: GLuint) {
		panic!("OpenGL function pointer of `glDrawElementsInstancedBaseInstance` is NULL");
	}
	fn glDrawElementsInstancedBaseVertexBaseInstance(&self, _: GLenum, _: GLsizei, _: GLenum, _: *const c_void, _: GLsizei, _: GLint, _: GLuint) {
		panic!("OpenGL function pointer of `glDrawElementsInstancedBaseVertexBaseInstance` is NULL");
	}
	fn glGetInternalformativ(&self, _: GLenum, _: GLenum, _: GLenum, _: GLsizei, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetInternalformativ` is NULL");
	}
	fn glGetActiveAtomicCounterBufferiv(&self, _: GLuint, _: GLuint, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetActiveAtomicCounterBufferiv` is NULL");
	}
	fn glBindImageTexture(&self, _: GLuint, _: GLuint, _: GLint, _: GLboolean, _: GLint, _: GLenum, _: GLenum) {
		panic!("OpenGL function pointer of `glBindImageTexture` is NULL");
	}
	fn glMemoryBarrier(&self, _: GLbitfield) {
		panic!("OpenGL function pointer of `glMemoryBarrier` is NULL");
	}
	fn glTexStorage1D(&self, _: GLenum, _: GLsizei, _: GLenum, _: GLsizei) {
		panic!("OpenGL function pointer of `glTexStorage1D` is NULL");
	}
	fn glTexStorage2D(&self, _: GLenum, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei) {
		panic!("OpenGL function pointer of `glTexStorage2D` is NULL");
	}
	fn glTexStorage3D(&self, _: GLenum, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei, _: GLsizei) {
		panic!("OpenGL function pointer of `glTexStorage3D` is NULL");
	}
	fn glDrawTransformFeedbackInstanced(&self, _: GLenum, _: GLuint, _: GLsizei) {
		panic!("OpenGL function pointer of `glDrawTransformFeedbackInstanced` is NULL");
	}
	fn glDrawTransformFeedbackStreamInstanced(&self, _: GLenum, _: GLuint, _: GLuint, _: GLsizei) {
		panic!("OpenGL function pointer of `glDrawTransformFeedbackStreamInstanced` is NULL");
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Version42 {
	available: bool,
	drawarraysinstancedbaseinstance: PFNGLDRAWARRAYSINSTANCEDBASEINSTANCEPROC,
	drawelementsinstancedbaseinstance: PFNGLDRAWELEMENTSINSTANCEDBASEINSTANCEPROC,
	drawelementsinstancedbasevertexbaseinstance: PFNGLDRAWELEMENTSINSTANCEDBASEVERTEXBASEINSTANCEPROC,
	getinternalformativ: PFNGLGETINTERNALFORMATIVPROC,
	getactiveatomiccounterbufferiv: PFNGLGETACTIVEATOMICCOUNTERBUFFERIVPROC,
	bindimagetexture: PFNGLBINDIMAGETEXTUREPROC,
	memorybarrier: PFNGLMEMORYBARRIERPROC,
	texstorage1d: PFNGLTEXSTORAGE1DPROC,
	texstorage2d: PFNGLTEXSTORAGE2DPROC,
	texstorage3d: PFNGLTEXSTORAGE3DPROC,
	drawtransformfeedbackinstanced: PFNGLDRAWTRANSFORMFEEDBACKINSTANCEDPROC,
	drawtransformfeedbackstreaminstanced: PFNGLDRAWTRANSFORMFEEDBACKSTREAMINSTANCEDPROC,
}

impl GL_4_2 for Version42 {
	#[inline(always)]
	fn glDrawArraysInstancedBaseInstance(&self, mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei, baseinstance: GLuint) {
		(self.drawarraysinstancedbaseinstance)(mode, first, count, instancecount, baseinstance)
	}
	#[inline(always)]
	fn glDrawElementsInstancedBaseInstance(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei, baseinstance: GLuint) {
		(self.drawelementsinstancedbaseinstance)(mode, count, type_, indices, instancecount, baseinstance)
	}
	#[inline(always)]
	fn glDrawElementsInstancedBaseVertexBaseInstance(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei, basevertex: GLint, baseinstance: GLuint) {
		(self.drawelementsinstancedbasevertexbaseinstance)(mode, count, type_, indices, instancecount, basevertex, baseinstance)
	}
	#[inline(always)]
	fn glGetInternalformativ(&self, target: GLenum, internalformat: GLenum, pname: GLenum, count: GLsizei, params: *mut GLint) {
		(self.getinternalformativ)(target, internalformat, pname, count, params)
	}
	#[inline(always)]
	fn glGetActiveAtomicCounterBufferiv(&self, program: GLuint, bufferIndex: GLuint, pname: GLenum, params: *mut GLint) {
		(self.getactiveatomiccounterbufferiv)(program, bufferIndex, pname, params)
	}
	#[inline(always)]
	fn glBindImageTexture(&self, unit: GLuint, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, access: GLenum, format: GLenum) {
		(self.bindimagetexture)(unit, texture, level, layered, layer, access, format)
	}
	#[inline(always)]
	fn glMemoryBarrier(&self, barriers: GLbitfield) {
		(self.memorybarrier)(barriers)
	}
	#[inline(always)]
	fn glTexStorage1D(&self, target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei) {
		(self.texstorage1d)(target, levels, internalformat, width)
	}
	#[inline(always)]
	fn glTexStorage2D(&self, target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) {
		(self.texstorage2d)(target, levels, internalformat, width, height)
	}
	#[inline(always)]
	fn glTexStorage3D(&self, target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei) {
		(self.texstorage3d)(target, levels, internalformat, width, height, depth)
	}
	#[inline(always)]
	fn glDrawTransformFeedbackInstanced(&self, mode: GLenum, id: GLuint, instancecount: GLsizei) {
		(self.drawtransformfeedbackinstanced)(mode, id, instancecount)
	}
	#[inline(always)]
	fn glDrawTransformFeedbackStreamInstanced(&self, mode: GLenum, id: GLuint, stream: GLuint, instancecount: GLsizei) {
		(self.drawtransformfeedbackstreaminstanced)(mode, id, stream, instancecount)
	}
}

impl Version42 {
	pub fn new(base: impl GL_1_0, get_proc_address: &impl Fn(&'static str) -> *const c_void) -> Self {
		let (spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 40200 {
			return Self::default();
		}
		Self {
			available: true,
			drawarraysinstancedbaseinstance: unsafe{transmute(get_proc_address("glDrawArraysInstancedBaseInstance"))},
			drawelementsinstancedbaseinstance: unsafe{transmute(get_proc_address("glDrawElementsInstancedBaseInstance"))},
			drawelementsinstancedbasevertexbaseinstance: unsafe{transmute(get_proc_address("glDrawElementsInstancedBaseVertexBaseInstance"))},
			getinternalformativ: unsafe{transmute(get_proc_address("glGetInternalformativ"))},
			getactiveatomiccounterbufferiv: unsafe{transmute(get_proc_address("glGetActiveAtomicCounterBufferiv"))},
			bindimagetexture: unsafe{transmute(get_proc_address("glBindImageTexture"))},
			memorybarrier: unsafe{transmute(get_proc_address("glMemoryBarrier"))},
			texstorage1d: unsafe{transmute(get_proc_address("glTexStorage1D"))},
			texstorage2d: unsafe{transmute(get_proc_address("glTexStorage2D"))},
			texstorage3d: unsafe{transmute(get_proc_address("glTexStorage3D"))},
			drawtransformfeedbackinstanced: unsafe{transmute(get_proc_address("glDrawTransformFeedbackInstanced"))},
			drawtransformfeedbackstreaminstanced: unsafe{transmute(get_proc_address("glDrawTransformFeedbackStreamInstanced"))},
		}
	}
	#[inline(always)]
	pub fn get_available(&self) -> bool {
		self.available
	}
}

impl Default for Version42 {
	fn default() -> Self {
		Self {
			available: false,
			drawarraysinstancedbaseinstance: null(),
			drawelementsinstancedbaseinstance: null(),
			drawelementsinstancedbasevertexbaseinstance: null(),
			getinternalformativ: null(),
			getactiveatomiccounterbufferiv: null(),
			bindimagetexture: null(),
			memorybarrier: null(),
			texstorage1d: null(),
			texstorage2d: null(),
			texstorage3d: null(),
			drawtransformfeedbackinstanced: null(),
			drawtransformfeedbackstreaminstanced: null(),
		}
	}
}

type PFNGLCLEARBUFFERDATAPROC = extern "system" fn(GLenum, GLenum, GLenum, GLenum, *const c_void);
type PFNGLCLEARBUFFERSUBDATAPROC = extern "system" fn(GLenum, GLenum, GLintptr, GLsizeiptr, GLenum, GLenum, *const c_void);
type PFNGLDISPATCHCOMPUTEPROC = extern "system" fn(GLuint, GLuint, GLuint);
type PFNGLDISPATCHCOMPUTEINDIRECTPROC = extern "system" fn(GLintptr);
type PFNGLCOPYIMAGESUBDATAPROC = extern "system" fn(GLuint, GLenum, GLint, GLint, GLint, GLint, GLuint, GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei);
type PFNGLFRAMEBUFFERPARAMETERIPROC = extern "system" fn(GLenum, GLenum, GLint);
type PFNGLGETFRAMEBUFFERPARAMETERIVPROC = extern "system" fn(GLenum, GLenum, *mut GLint);
type PFNGLGETINTERNALFORMATI64VPROC = extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut GLint64);
type PFNGLINVALIDATETEXSUBIMAGEPROC = extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei);
type PFNGLINVALIDATETEXIMAGEPROC = extern "system" fn(GLuint, GLint);
type PFNGLINVALIDATEBUFFERSUBDATAPROC = extern "system" fn(GLuint, GLintptr, GLsizeiptr);
type PFNGLINVALIDATEBUFFERDATAPROC = extern "system" fn(GLuint);
type PFNGLINVALIDATEFRAMEBUFFERPROC = extern "system" fn(GLenum, GLsizei, *const GLenum);
type PFNGLINVALIDATESUBFRAMEBUFFERPROC = extern "system" fn(GLenum, GLsizei, *const GLenum, GLint, GLint, GLsizei, GLsizei);
type PFNGLMULTIDRAWARRAYSINDIRECTPROC = extern "system" fn(GLenum, *const c_void, GLsizei, GLsizei);
type PFNGLMULTIDRAWELEMENTSINDIRECTPROC = extern "system" fn(GLenum, GLenum, *const c_void, GLsizei, GLsizei);
type PFNGLGETPROGRAMINTERFACEIVPROC = extern "system" fn(GLuint, GLenum, GLenum, *mut GLint);
type PFNGLGETPROGRAMRESOURCEINDEXPROC = extern "system" fn(GLuint, GLenum, *const GLchar) -> GLuint;
type PFNGLGETPROGRAMRESOURCENAMEPROC = extern "system" fn(GLuint, GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar);
type PFNGLGETPROGRAMRESOURCEIVPROC = extern "system" fn(GLuint, GLenum, GLuint, GLsizei, *const GLenum, GLsizei, *mut GLsizei, *mut GLint);
type PFNGLGETPROGRAMRESOURCELOCATIONPROC = extern "system" fn(GLuint, GLenum, *const GLchar) -> GLint;
type PFNGLGETPROGRAMRESOURCELOCATIONINDEXPROC = extern "system" fn(GLuint, GLenum, *const GLchar) -> GLint;
type PFNGLSHADERSTORAGEBLOCKBINDINGPROC = extern "system" fn(GLuint, GLuint, GLuint);
type PFNGLTEXBUFFERRANGEPROC = extern "system" fn(GLenum, GLenum, GLuint, GLintptr, GLsizeiptr);
type PFNGLTEXSTORAGE2DMULTISAMPLEPROC = extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLboolean);
type PFNGLTEXSTORAGE3DMULTISAMPLEPROC = extern "system" fn(GLenum, GLsizei, GLenum, GLsizei, GLsizei, GLsizei, GLboolean);
type PFNGLTEXTUREVIEWPROC = extern "system" fn(GLuint, GLenum, GLuint, GLenum, GLuint, GLuint, GLuint, GLuint);
type PFNGLBINDVERTEXBUFFERPROC = extern "system" fn(GLuint, GLuint, GLintptr, GLsizei);
type PFNGLVERTEXATTRIBFORMATPROC = extern "system" fn(GLuint, GLint, GLenum, GLboolean, GLuint);
type PFNGLVERTEXATTRIBIFORMATPROC = extern "system" fn(GLuint, GLint, GLenum, GLuint);
type PFNGLVERTEXATTRIBLFORMATPROC = extern "system" fn(GLuint, GLint, GLenum, GLuint);
type PFNGLVERTEXATTRIBBINDINGPROC = extern "system" fn(GLuint, GLuint);
type PFNGLVERTEXBINDINGDIVISORPROC = extern "system" fn(GLuint, GLuint);
type PFNGLDEBUGMESSAGECONTROLPROC = extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *const GLuint, GLboolean);
type PFNGLDEBUGMESSAGEINSERTPROC = extern "system" fn(GLenum, GLenum, GLuint, GLenum, GLsizei, *const GLchar);
type PFNGLDEBUGMESSAGECALLBACKPROC = extern "system" fn(GLDEBUGPROC, *const c_void);
type PFNGLGETDEBUGMESSAGELOGPROC = extern "system" fn(GLuint, GLsizei, *mut GLenum, *mut GLenum, *mut GLuint, *mut GLenum, *mut GLsizei, *mut GLchar) -> GLuint;
type PFNGLPUSHDEBUGGROUPPROC = extern "system" fn(GLenum, GLuint, GLsizei, *const GLchar);
type PFNGLPOPDEBUGGROUPPROC = extern "system" fn();
type PFNGLOBJECTLABELPROC = extern "system" fn(GLenum, GLuint, GLsizei, *const GLchar);
type PFNGLGETOBJECTLABELPROC = extern "system" fn(GLenum, GLuint, GLsizei, *mut GLsizei, *mut GLchar);
type PFNGLOBJECTPTRLABELPROC = extern "system" fn(*const c_void, GLsizei, *const GLchar);
type PFNGLGETOBJECTPTRLABELPROC = extern "system" fn(*const c_void, GLsizei, *mut GLsizei, *mut GLchar);
const NUM_SHADING_LANGUAGE_VERSIONS: GLenum = 0x82E9;
const VERTEX_ATTRIB_ARRAY_LONG: GLenum = 0x874E;
const COMPRESSED_RGB8_ETC2: GLenum = 0x9274;
const COMPRESSED_SRGB8_ETC2: GLenum = 0x9275;
const COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = 0x9276;
const COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = 0x9277;
const COMPRESSED_RGBA8_ETC2_EAC: GLenum = 0x9278;
const COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: GLenum = 0x9279;
const COMPRESSED_R11_EAC: GLenum = 0x9270;
const COMPRESSED_SIGNED_R11_EAC: GLenum = 0x9271;
const COMPRESSED_RG11_EAC: GLenum = 0x9272;
const COMPRESSED_SIGNED_RG11_EAC: GLenum = 0x9273;
const PRIMITIVE_RESTART_FIXED_INDEX: GLenum = 0x8D69;
const ANY_SAMPLES_PASSED_CONSERVATIVE: GLenum = 0x8D6A;
const MAX_ELEMENT_INDEX: GLenum = 0x8D6B;
const COMPUTE_SHADER: GLenum = 0x91B9;
const MAX_COMPUTE_UNIFORM_BLOCKS: GLenum = 0x91BB;
const MAX_COMPUTE_TEXTURE_IMAGE_UNITS: GLenum = 0x91BC;
const MAX_COMPUTE_IMAGE_UNIFORMS: GLenum = 0x91BD;
const MAX_COMPUTE_SHARED_MEMORY_SIZE: GLenum = 0x8262;
const MAX_COMPUTE_UNIFORM_COMPONENTS: GLenum = 0x8263;
const MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS: GLenum = 0x8264;
const MAX_COMPUTE_ATOMIC_COUNTERS: GLenum = 0x8265;
const MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS: GLenum = 0x8266;
const MAX_COMPUTE_WORK_GROUP_INVOCATIONS: GLenum = 0x90EB;
const MAX_COMPUTE_WORK_GROUP_COUNT: GLenum = 0x91BE;
const MAX_COMPUTE_WORK_GROUP_SIZE: GLenum = 0x91BF;
const COMPUTE_WORK_GROUP_SIZE: GLenum = 0x8267;
const UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x90EC;
const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x90ED;
const DISPATCH_INDIRECT_BUFFER: GLenum = 0x90EE;
const DISPATCH_INDIRECT_BUFFER_BINDING: GLenum = 0x90EF;
const COMPUTE_SHADER_BIT: GLbitfield = 0x00000020;
const DEBUG_OUTPUT_SYNCHRONOUS: GLenum = 0x8242;
const DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: GLenum = 0x8243;
const DEBUG_CALLBACK_FUNCTION: GLenum = 0x8244;
const DEBUG_CALLBACK_USER_PARAM: GLenum = 0x8245;
const DEBUG_SOURCE_API: GLenum = 0x8246;
const DEBUG_SOURCE_WINDOW_SYSTEM: GLenum = 0x8247;
const DEBUG_SOURCE_SHADER_COMPILER: GLenum = 0x8248;
const DEBUG_SOURCE_THIRD_PARTY: GLenum = 0x8249;
const DEBUG_SOURCE_APPLICATION: GLenum = 0x824A;
const DEBUG_SOURCE_OTHER: GLenum = 0x824B;
const DEBUG_TYPE_ERROR: GLenum = 0x824C;
const DEBUG_TYPE_DEPRECATED_BEHAVIOR: GLenum = 0x824D;
const DEBUG_TYPE_UNDEFINED_BEHAVIOR: GLenum = 0x824E;
const DEBUG_TYPE_PORTABILITY: GLenum = 0x824F;
const DEBUG_TYPE_PERFORMANCE: GLenum = 0x8250;
const DEBUG_TYPE_OTHER: GLenum = 0x8251;
const MAX_DEBUG_MESSAGE_LENGTH: GLenum = 0x9143;
const MAX_DEBUG_LOGGED_MESSAGES: GLenum = 0x9144;
const DEBUG_LOGGED_MESSAGES: GLenum = 0x9145;
const DEBUG_SEVERITY_HIGH: GLenum = 0x9146;
const DEBUG_SEVERITY_MEDIUM: GLenum = 0x9147;
const DEBUG_SEVERITY_LOW: GLenum = 0x9148;
const DEBUG_TYPE_MARKER: GLenum = 0x8268;
const DEBUG_TYPE_PUSH_GROUP: GLenum = 0x8269;
const DEBUG_TYPE_POP_GROUP: GLenum = 0x826A;
const DEBUG_SEVERITY_NOTIFICATION: GLenum = 0x826B;
const MAX_DEBUG_GROUP_STACK_DEPTH: GLenum = 0x826C;
const DEBUG_GROUP_STACK_DEPTH: GLenum = 0x826D;
const BUFFER: GLenum = 0x82E0;
const SHADER: GLenum = 0x82E1;
const PROGRAM: GLenum = 0x82E2;
const QUERY: GLenum = 0x82E3;
const PROGRAM_PIPELINE: GLenum = 0x82E4;
const SAMPLER: GLenum = 0x82E6;
const MAX_LABEL_LENGTH: GLenum = 0x82E8;
const DEBUG_OUTPUT: GLenum = 0x92E0;
const CONTEXT_FLAG_DEBUG_BIT: GLbitfield = 0x00000002;
const MAX_UNIFORM_LOCATIONS: GLenum = 0x826E;
const FRAMEBUFFER_DEFAULT_WIDTH: GLenum = 0x9310;
const FRAMEBUFFER_DEFAULT_HEIGHT: GLenum = 0x9311;
const FRAMEBUFFER_DEFAULT_LAYERS: GLenum = 0x9312;
const FRAMEBUFFER_DEFAULT_SAMPLES: GLenum = 0x9313;
const FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: GLenum = 0x9314;
const MAX_FRAMEBUFFER_WIDTH: GLenum = 0x9315;
const MAX_FRAMEBUFFER_HEIGHT: GLenum = 0x9316;
const MAX_FRAMEBUFFER_LAYERS: GLenum = 0x9317;
const MAX_FRAMEBUFFER_SAMPLES: GLenum = 0x9318;
const INTERNALFORMAT_SUPPORTED: GLenum = 0x826F;
const INTERNALFORMAT_PREFERRED: GLenum = 0x8270;
const INTERNALFORMAT_RED_SIZE: GLenum = 0x8271;
const INTERNALFORMAT_GREEN_SIZE: GLenum = 0x8272;
const INTERNALFORMAT_BLUE_SIZE: GLenum = 0x8273;
const INTERNALFORMAT_ALPHA_SIZE: GLenum = 0x8274;
const INTERNALFORMAT_DEPTH_SIZE: GLenum = 0x8275;
const INTERNALFORMAT_STENCIL_SIZE: GLenum = 0x8276;
const INTERNALFORMAT_SHARED_SIZE: GLenum = 0x8277;
const INTERNALFORMAT_RED_TYPE: GLenum = 0x8278;
const INTERNALFORMAT_GREEN_TYPE: GLenum = 0x8279;
const INTERNALFORMAT_BLUE_TYPE: GLenum = 0x827A;
const INTERNALFORMAT_ALPHA_TYPE: GLenum = 0x827B;
const INTERNALFORMAT_DEPTH_TYPE: GLenum = 0x827C;
const INTERNALFORMAT_STENCIL_TYPE: GLenum = 0x827D;
const MAX_WIDTH: GLenum = 0x827E;
const MAX_HEIGHT: GLenum = 0x827F;
const MAX_DEPTH: GLenum = 0x8280;
const MAX_LAYERS: GLenum = 0x8281;
const MAX_COMBINED_DIMENSIONS: GLenum = 0x8282;
const COLOR_COMPONENTS: GLenum = 0x8283;
const DEPTH_COMPONENTS: GLenum = 0x8284;
const STENCIL_COMPONENTS: GLenum = 0x8285;
const COLOR_RENDERABLE: GLenum = 0x8286;
const DEPTH_RENDERABLE: GLenum = 0x8287;
const STENCIL_RENDERABLE: GLenum = 0x8288;
const FRAMEBUFFER_RENDERABLE: GLenum = 0x8289;
const FRAMEBUFFER_RENDERABLE_LAYERED: GLenum = 0x828A;
const FRAMEBUFFER_BLEND: GLenum = 0x828B;
const READ_PIXELS: GLenum = 0x828C;
const READ_PIXELS_FORMAT: GLenum = 0x828D;
const READ_PIXELS_TYPE: GLenum = 0x828E;
const TEXTURE_IMAGE_FORMAT: GLenum = 0x828F;
const TEXTURE_IMAGE_TYPE: GLenum = 0x8290;
const GET_TEXTURE_IMAGE_FORMAT: GLenum = 0x8291;
const GET_TEXTURE_IMAGE_TYPE: GLenum = 0x8292;
const MIPMAP: GLenum = 0x8293;
const MANUAL_GENERATE_MIPMAP: GLenum = 0x8294;
const AUTO_GENERATE_MIPMAP: GLenum = 0x8295;
const COLOR_ENCODING: GLenum = 0x8296;
const SRGB_READ: GLenum = 0x8297;
const SRGB_WRITE: GLenum = 0x8298;
const FILTER: GLenum = 0x829A;
const VERTEX_TEXTURE: GLenum = 0x829B;
const TESS_CONTROL_TEXTURE: GLenum = 0x829C;
const TESS_EVALUATION_TEXTURE: GLenum = 0x829D;
const GEOMETRY_TEXTURE: GLenum = 0x829E;
const FRAGMENT_TEXTURE: GLenum = 0x829F;
const COMPUTE_TEXTURE: GLenum = 0x82A0;
const TEXTURE_SHADOW: GLenum = 0x82A1;
const TEXTURE_GATHER: GLenum = 0x82A2;
const TEXTURE_GATHER_SHADOW: GLenum = 0x82A3;
const SHADER_IMAGE_LOAD: GLenum = 0x82A4;
const SHADER_IMAGE_STORE: GLenum = 0x82A5;
const SHADER_IMAGE_ATOMIC: GLenum = 0x82A6;
const IMAGE_TEXEL_SIZE: GLenum = 0x82A7;
const IMAGE_COMPATIBILITY_CLASS: GLenum = 0x82A8;
const IMAGE_PIXEL_FORMAT: GLenum = 0x82A9;
const IMAGE_PIXEL_TYPE: GLenum = 0x82AA;
const SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST: GLenum = 0x82AC;
const SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST: GLenum = 0x82AD;
const SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE: GLenum = 0x82AE;
const SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE: GLenum = 0x82AF;
const TEXTURE_COMPRESSED_BLOCK_WIDTH: GLenum = 0x82B1;
const TEXTURE_COMPRESSED_BLOCK_HEIGHT: GLenum = 0x82B2;
const TEXTURE_COMPRESSED_BLOCK_SIZE: GLenum = 0x82B3;
const CLEAR_BUFFER: GLenum = 0x82B4;
const TEXTURE_VIEW: GLenum = 0x82B5;
const VIEW_COMPATIBILITY_CLASS: GLenum = 0x82B6;
const FULL_SUPPORT: GLenum = 0x82B7;
const CAVEAT_SUPPORT: GLenum = 0x82B8;
const IMAGE_CLASS_4_X_32: GLenum = 0x82B9;
const IMAGE_CLASS_2_X_32: GLenum = 0x82BA;
const IMAGE_CLASS_1_X_32: GLenum = 0x82BB;
const IMAGE_CLASS_4_X_16: GLenum = 0x82BC;
const IMAGE_CLASS_2_X_16: GLenum = 0x82BD;
const IMAGE_CLASS_1_X_16: GLenum = 0x82BE;
const IMAGE_CLASS_4_X_8: GLenum = 0x82BF;
const IMAGE_CLASS_2_X_8: GLenum = 0x82C0;
const IMAGE_CLASS_1_X_8: GLenum = 0x82C1;
const IMAGE_CLASS_11_11_10: GLenum = 0x82C2;
const IMAGE_CLASS_10_10_10_2: GLenum = 0x82C3;
const VIEW_CLASS_128_BITS: GLenum = 0x82C4;
const VIEW_CLASS_96_BITS: GLenum = 0x82C5;
const VIEW_CLASS_64_BITS: GLenum = 0x82C6;
const VIEW_CLASS_48_BITS: GLenum = 0x82C7;
const VIEW_CLASS_32_BITS: GLenum = 0x82C8;
const VIEW_CLASS_24_BITS: GLenum = 0x82C9;
const VIEW_CLASS_16_BITS: GLenum = 0x82CA;
const VIEW_CLASS_8_BITS: GLenum = 0x82CB;
const VIEW_CLASS_S3TC_DXT1_RGB: GLenum = 0x82CC;
const VIEW_CLASS_S3TC_DXT1_RGBA: GLenum = 0x82CD;
const VIEW_CLASS_S3TC_DXT3_RGBA: GLenum = 0x82CE;
const VIEW_CLASS_S3TC_DXT5_RGBA: GLenum = 0x82CF;
const VIEW_CLASS_RGTC1_RED: GLenum = 0x82D0;
const VIEW_CLASS_RGTC2_RG: GLenum = 0x82D1;
const VIEW_CLASS_BPTC_UNORM: GLenum = 0x82D2;
const VIEW_CLASS_BPTC_FLOAT: GLenum = 0x82D3;
const UNIFORM: GLenum = 0x92E1;
const UNIFORM_BLOCK: GLenum = 0x92E2;
const PROGRAM_INPUT: GLenum = 0x92E3;
const PROGRAM_OUTPUT: GLenum = 0x92E4;
const BUFFER_VARIABLE: GLenum = 0x92E5;
const SHADER_STORAGE_BLOCK: GLenum = 0x92E6;
const VERTEX_SUBROUTINE: GLenum = 0x92E8;
const TESS_CONTROL_SUBROUTINE: GLenum = 0x92E9;
const TESS_EVALUATION_SUBROUTINE: GLenum = 0x92EA;
const GEOMETRY_SUBROUTINE: GLenum = 0x92EB;
const FRAGMENT_SUBROUTINE: GLenum = 0x92EC;
const COMPUTE_SUBROUTINE: GLenum = 0x92ED;
const VERTEX_SUBROUTINE_UNIFORM: GLenum = 0x92EE;
const TESS_CONTROL_SUBROUTINE_UNIFORM: GLenum = 0x92EF;
const TESS_EVALUATION_SUBROUTINE_UNIFORM: GLenum = 0x92F0;
const GEOMETRY_SUBROUTINE_UNIFORM: GLenum = 0x92F1;
const FRAGMENT_SUBROUTINE_UNIFORM: GLenum = 0x92F2;
const COMPUTE_SUBROUTINE_UNIFORM: GLenum = 0x92F3;
const TRANSFORM_FEEDBACK_VARYING: GLenum = 0x92F4;
const ACTIVE_RESOURCES: GLenum = 0x92F5;
const MAX_NAME_LENGTH: GLenum = 0x92F6;
const MAX_NUM_ACTIVE_VARIABLES: GLenum = 0x92F7;
const MAX_NUM_COMPATIBLE_SUBROUTINES: GLenum = 0x92F8;
const NAME_LENGTH: GLenum = 0x92F9;
const TYPE: GLenum = 0x92FA;
const ARRAY_SIZE: GLenum = 0x92FB;
const OFFSET: GLenum = 0x92FC;
const BLOCK_INDEX: GLenum = 0x92FD;
const ARRAY_STRIDE: GLenum = 0x92FE;
const MATRIX_STRIDE: GLenum = 0x92FF;
const IS_ROW_MAJOR: GLenum = 0x9300;
const ATOMIC_COUNTER_BUFFER_INDEX: GLenum = 0x9301;
const BUFFER_BINDING: GLenum = 0x9302;
const BUFFER_DATA_SIZE: GLenum = 0x9303;
const NUM_ACTIVE_VARIABLES: GLenum = 0x9304;
const ACTIVE_VARIABLES: GLenum = 0x9305;
const REFERENCED_BY_VERTEX_SHADER: GLenum = 0x9306;
const REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x9307;
const REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x9308;
const REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x9309;
const REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x930A;
const REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x930B;
const TOP_LEVEL_ARRAY_SIZE: GLenum = 0x930C;
const TOP_LEVEL_ARRAY_STRIDE: GLenum = 0x930D;
const LOCATION: GLenum = 0x930E;
const LOCATION_INDEX: GLenum = 0x930F;
const IS_PER_PATCH: GLenum = 0x92E7;
const SHADER_STORAGE_BUFFER: GLenum = 0x90D2;
const SHADER_STORAGE_BUFFER_BINDING: GLenum = 0x90D3;
const SHADER_STORAGE_BUFFER_START: GLenum = 0x90D4;
const SHADER_STORAGE_BUFFER_SIZE: GLenum = 0x90D5;
const MAX_VERTEX_SHADER_STORAGE_BLOCKS: GLenum = 0x90D6;
const MAX_GEOMETRY_SHADER_STORAGE_BLOCKS: GLenum = 0x90D7;
const MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS: GLenum = 0x90D8;
const MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS: GLenum = 0x90D9;
const MAX_FRAGMENT_SHADER_STORAGE_BLOCKS: GLenum = 0x90DA;
const MAX_COMPUTE_SHADER_STORAGE_BLOCKS: GLenum = 0x90DB;
const MAX_COMBINED_SHADER_STORAGE_BLOCKS: GLenum = 0x90DC;
const MAX_SHADER_STORAGE_BUFFER_BINDINGS: GLenum = 0x90DD;
const MAX_SHADER_STORAGE_BLOCK_SIZE: GLenum = 0x90DE;
const SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x90DF;
const SHADER_STORAGE_BARRIER_BIT: GLbitfield = 0x00002000;
const MAX_COMBINED_SHADER_OUTPUT_RESOURCES: GLenum = 0x8F39;
const DEPTH_STENCIL_TEXTURE_MODE: GLenum = 0x90EA;
const TEXTURE_BUFFER_OFFSET: GLenum = 0x919D;
const TEXTURE_BUFFER_SIZE: GLenum = 0x919E;
const TEXTURE_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x919F;
const TEXTURE_VIEW_MIN_LEVEL: GLenum = 0x82DB;
const TEXTURE_VIEW_NUM_LEVELS: GLenum = 0x82DC;
const TEXTURE_VIEW_MIN_LAYER: GLenum = 0x82DD;
const TEXTURE_VIEW_NUM_LAYERS: GLenum = 0x82DE;
const TEXTURE_IMMUTABLE_LEVELS: GLenum = 0x82DF;
const VERTEX_ATTRIB_BINDING: GLenum = 0x82D4;
const VERTEX_ATTRIB_RELATIVE_OFFSET: GLenum = 0x82D5;
const VERTEX_BINDING_DIVISOR: GLenum = 0x82D6;
const VERTEX_BINDING_OFFSET: GLenum = 0x82D7;
const VERTEX_BINDING_STRIDE: GLenum = 0x82D8;
const MAX_VERTEX_ATTRIB_RELATIVE_OFFSET: GLenum = 0x82D9;
const MAX_VERTEX_ATTRIB_BINDINGS: GLenum = 0x82DA;
const VERTEX_BINDING_BUFFER: GLenum = 0x8F4F;
const DISPLAY_LIST: GLenum = 0x82E7;

pub trait GL_4_3 {
	fn glClearBufferData(&self, _: GLenum, _: GLenum, _: GLenum, _: GLenum, _: *const c_void) {
		panic!("OpenGL function pointer of `glClearBufferData` is NULL");
	}
	fn glClearBufferSubData(&self, _: GLenum, _: GLenum, _: GLintptr, _: GLsizeiptr, _: GLenum, _: GLenum, _: *const c_void) {
		panic!("OpenGL function pointer of `glClearBufferSubData` is NULL");
	}
	fn glDispatchCompute(&self, _: GLuint, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glDispatchCompute` is NULL");
	}
	fn glDispatchComputeIndirect(&self, _: GLintptr) {
		panic!("OpenGL function pointer of `glDispatchComputeIndirect` is NULL");
	}
	fn glCopyImageSubData(&self, _: GLuint, _: GLenum, _: GLint, _: GLint, _: GLint, _: GLint, _: GLuint, _: GLenum, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLsizei) {
		panic!("OpenGL function pointer of `glCopyImageSubData` is NULL");
	}
	fn glFramebufferParameteri(&self, _: GLenum, _: GLenum, _: GLint) {
		panic!("OpenGL function pointer of `glFramebufferParameteri` is NULL");
	}
	fn glGetFramebufferParameteriv(&self, _: GLenum, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetFramebufferParameteriv` is NULL");
	}
	fn glGetInternalformati64v(&self, _: GLenum, _: GLenum, _: GLenum, _: GLsizei, _: *mut GLint64) {
		panic!("OpenGL function pointer of `glGetInternalformati64v` is NULL");
	}
	fn glInvalidateTexSubImage(&self, _: GLuint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLsizei) {
		panic!("OpenGL function pointer of `glInvalidateTexSubImage` is NULL");
	}
	fn glInvalidateTexImage(&self, _: GLuint, _: GLint) {
		panic!("OpenGL function pointer of `glInvalidateTexImage` is NULL");
	}
	fn glInvalidateBufferSubData(&self, _: GLuint, _: GLintptr, _: GLsizeiptr) {
		panic!("OpenGL function pointer of `glInvalidateBufferSubData` is NULL");
	}
	fn glInvalidateBufferData(&self, _: GLuint) {
		panic!("OpenGL function pointer of `glInvalidateBufferData` is NULL");
	}
	fn glInvalidateFramebuffer(&self, _: GLenum, _: GLsizei, _: *const GLenum) {
		panic!("OpenGL function pointer of `glInvalidateFramebuffer` is NULL");
	}
	fn glInvalidateSubFramebuffer(&self, _: GLenum, _: GLsizei, _: *const GLenum, _: GLint, _: GLint, _: GLsizei, _: GLsizei) {
		panic!("OpenGL function pointer of `glInvalidateSubFramebuffer` is NULL");
	}
	fn glMultiDrawArraysIndirect(&self, _: GLenum, _: *const c_void, _: GLsizei, _: GLsizei) {
		panic!("OpenGL function pointer of `glMultiDrawArraysIndirect` is NULL");
	}
	fn glMultiDrawElementsIndirect(&self, _: GLenum, _: GLenum, _: *const c_void, _: GLsizei, _: GLsizei) {
		panic!("OpenGL function pointer of `glMultiDrawElementsIndirect` is NULL");
	}
	fn glGetProgramInterfaceiv(&self, _: GLuint, _: GLenum, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetProgramInterfaceiv` is NULL");
	}
	fn glGetProgramResourceIndex(&self, _: GLuint, _: GLenum, _: *const GLchar) -> GLuint {
		panic!("OpenGL function pointer of `glGetProgramResourceIndex` is NULL");
	}
	fn glGetProgramResourceName(&self, _: GLuint, _: GLenum, _: GLuint, _: GLsizei, _: *mut GLsizei, _: *mut GLchar) {
		panic!("OpenGL function pointer of `glGetProgramResourceName` is NULL");
	}
	fn glGetProgramResourceiv(&self, _: GLuint, _: GLenum, _: GLuint, _: GLsizei, _: *const GLenum, _: GLsizei, _: *mut GLsizei, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetProgramResourceiv` is NULL");
	}
	fn glGetProgramResourceLocation(&self, _: GLuint, _: GLenum, _: *const GLchar) -> GLint {
		panic!("OpenGL function pointer of `glGetProgramResourceLocation` is NULL");
	}
	fn glGetProgramResourceLocationIndex(&self, _: GLuint, _: GLenum, _: *const GLchar) -> GLint {
		panic!("OpenGL function pointer of `glGetProgramResourceLocationIndex` is NULL");
	}
	fn glShaderStorageBlockBinding(&self, _: GLuint, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glShaderStorageBlockBinding` is NULL");
	}
	fn glTexBufferRange(&self, _: GLenum, _: GLenum, _: GLuint, _: GLintptr, _: GLsizeiptr) {
		panic!("OpenGL function pointer of `glTexBufferRange` is NULL");
	}
	fn glTexStorage2DMultisample(&self, _: GLenum, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei, _: GLboolean) {
		panic!("OpenGL function pointer of `glTexStorage2DMultisample` is NULL");
	}
	fn glTexStorage3DMultisample(&self, _: GLenum, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei, _: GLsizei, _: GLboolean) {
		panic!("OpenGL function pointer of `glTexStorage3DMultisample` is NULL");
	}
	fn glTextureView(&self, _: GLuint, _: GLenum, _: GLuint, _: GLenum, _: GLuint, _: GLuint, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glTextureView` is NULL");
	}
	fn glBindVertexBuffer(&self, _: GLuint, _: GLuint, _: GLintptr, _: GLsizei) {
		panic!("OpenGL function pointer of `glBindVertexBuffer` is NULL");
	}
	fn glVertexAttribFormat(&self, _: GLuint, _: GLint, _: GLenum, _: GLboolean, _: GLuint) {
		panic!("OpenGL function pointer of `glVertexAttribFormat` is NULL");
	}
	fn glVertexAttribIFormat(&self, _: GLuint, _: GLint, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glVertexAttribIFormat` is NULL");
	}
	fn glVertexAttribLFormat(&self, _: GLuint, _: GLint, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glVertexAttribLFormat` is NULL");
	}
	fn glVertexAttribBinding(&self, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glVertexAttribBinding` is NULL");
	}
	fn glVertexBindingDivisor(&self, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glVertexBindingDivisor` is NULL");
	}
	fn glDebugMessageControl(&self, _: GLenum, _: GLenum, _: GLenum, _: GLsizei, _: *const GLuint, _: GLboolean) {
		panic!("OpenGL function pointer of `glDebugMessageControl` is NULL");
	}
	fn glDebugMessageInsert(&self, _: GLenum, _: GLenum, _: GLuint, _: GLenum, _: GLsizei, _: *const GLchar) {
		panic!("OpenGL function pointer of `glDebugMessageInsert` is NULL");
	}
	fn glDebugMessageCallback(&self, _: GLDEBUGPROC, _: *const c_void) {
		panic!("OpenGL function pointer of `glDebugMessageCallback` is NULL");
	}
	fn glGetDebugMessageLog(&self, _: GLuint, _: GLsizei, _: *mut GLenum, _: *mut GLenum, _: *mut GLuint, _: *mut GLenum, _: *mut GLsizei, _: *mut GLchar) -> GLuint {
		panic!("OpenGL function pointer of `glGetDebugMessageLog` is NULL");
	}
	fn glPushDebugGroup(&self, _: GLenum, _: GLuint, _: GLsizei, _: *const GLchar) {
		panic!("OpenGL function pointer of `glPushDebugGroup` is NULL");
	}
	fn glPopDebugGroup(&self) {
		panic!("OpenGL function pointer of `glPopDebugGroup` is NULL");
	}
	fn glObjectLabel(&self, _: GLenum, _: GLuint, _: GLsizei, _: *const GLchar) {
		panic!("OpenGL function pointer of `glObjectLabel` is NULL");
	}
	fn glGetObjectLabel(&self, _: GLenum, _: GLuint, _: GLsizei, _: *mut GLsizei, _: *mut GLchar) {
		panic!("OpenGL function pointer of `glGetObjectLabel` is NULL");
	}
	fn glObjectPtrLabel(&self, _: *const c_void, _: GLsizei, _: *const GLchar) {
		panic!("OpenGL function pointer of `glObjectPtrLabel` is NULL");
	}
	fn glGetObjectPtrLabel(&self, _: *const c_void, _: GLsizei, _: *mut GLsizei, _: *mut GLchar) {
		panic!("OpenGL function pointer of `glGetObjectPtrLabel` is NULL");
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Version43 {
	available: bool,
	clearbufferdata: PFNGLCLEARBUFFERDATAPROC,
	clearbuffersubdata: PFNGLCLEARBUFFERSUBDATAPROC,
	dispatchcompute: PFNGLDISPATCHCOMPUTEPROC,
	dispatchcomputeindirect: PFNGLDISPATCHCOMPUTEINDIRECTPROC,
	copyimagesubdata: PFNGLCOPYIMAGESUBDATAPROC,
	framebufferparameteri: PFNGLFRAMEBUFFERPARAMETERIPROC,
	getframebufferparameteriv: PFNGLGETFRAMEBUFFERPARAMETERIVPROC,
	getinternalformati64v: PFNGLGETINTERNALFORMATI64VPROC,
	invalidatetexsubimage: PFNGLINVALIDATETEXSUBIMAGEPROC,
	invalidateteximage: PFNGLINVALIDATETEXIMAGEPROC,
	invalidatebuffersubdata: PFNGLINVALIDATEBUFFERSUBDATAPROC,
	invalidatebufferdata: PFNGLINVALIDATEBUFFERDATAPROC,
	invalidateframebuffer: PFNGLINVALIDATEFRAMEBUFFERPROC,
	invalidatesubframebuffer: PFNGLINVALIDATESUBFRAMEBUFFERPROC,
	multidrawarraysindirect: PFNGLMULTIDRAWARRAYSINDIRECTPROC,
	multidrawelementsindirect: PFNGLMULTIDRAWELEMENTSINDIRECTPROC,
	getprograminterfaceiv: PFNGLGETPROGRAMINTERFACEIVPROC,
	getprogramresourceindex: PFNGLGETPROGRAMRESOURCEINDEXPROC,
	getprogramresourcename: PFNGLGETPROGRAMRESOURCENAMEPROC,
	getprogramresourceiv: PFNGLGETPROGRAMRESOURCEIVPROC,
	getprogramresourcelocation: PFNGLGETPROGRAMRESOURCELOCATIONPROC,
	getprogramresourcelocationindex: PFNGLGETPROGRAMRESOURCELOCATIONINDEXPROC,
	shaderstorageblockbinding: PFNGLSHADERSTORAGEBLOCKBINDINGPROC,
	texbufferrange: PFNGLTEXBUFFERRANGEPROC,
	texstorage2dmultisample: PFNGLTEXSTORAGE2DMULTISAMPLEPROC,
	texstorage3dmultisample: PFNGLTEXSTORAGE3DMULTISAMPLEPROC,
	textureview: PFNGLTEXTUREVIEWPROC,
	bindvertexbuffer: PFNGLBINDVERTEXBUFFERPROC,
	vertexattribformat: PFNGLVERTEXATTRIBFORMATPROC,
	vertexattribiformat: PFNGLVERTEXATTRIBIFORMATPROC,
	vertexattriblformat: PFNGLVERTEXATTRIBLFORMATPROC,
	vertexattribbinding: PFNGLVERTEXATTRIBBINDINGPROC,
	vertexbindingdivisor: PFNGLVERTEXBINDINGDIVISORPROC,
	debugmessagecontrol: PFNGLDEBUGMESSAGECONTROLPROC,
	debugmessageinsert: PFNGLDEBUGMESSAGEINSERTPROC,
	debugmessagecallback: PFNGLDEBUGMESSAGECALLBACKPROC,
	getdebugmessagelog: PFNGLGETDEBUGMESSAGELOGPROC,
	pushdebuggroup: PFNGLPUSHDEBUGGROUPPROC,
	popdebuggroup: PFNGLPOPDEBUGGROUPPROC,
	objectlabel: PFNGLOBJECTLABELPROC,
	getobjectlabel: PFNGLGETOBJECTLABELPROC,
	objectptrlabel: PFNGLOBJECTPTRLABELPROC,
	getobjectptrlabel: PFNGLGETOBJECTPTRLABELPROC,
}

impl GL_4_3 for Version43 {
	#[inline(always)]
	fn glClearBufferData(&self, target: GLenum, internalformat: GLenum, format: GLenum, type_: GLenum, data: *const c_void) {
		(self.clearbufferdata)(target, internalformat, format, type_, data)
	}
	#[inline(always)]
	fn glClearBufferSubData(&self, target: GLenum, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *const c_void) {
		(self.clearbuffersubdata)(target, internalformat, offset, size, format, type_, data)
	}
	#[inline(always)]
	fn glDispatchCompute(&self, num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint) {
		(self.dispatchcompute)(num_groups_x, num_groups_y, num_groups_z)
	}
	#[inline(always)]
	fn glDispatchComputeIndirect(&self, indirect: GLintptr) {
		(self.dispatchcomputeindirect)(indirect)
	}
	#[inline(always)]
	fn glCopyImageSubData(&self, srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei) {
		(self.copyimagesubdata)(srcName, srcTarget, srcLevel, srcX, srcY, srcZ, dstName, dstTarget, dstLevel, dstX, dstY, dstZ, srcWidth, srcHeight, srcDepth)
	}
	#[inline(always)]
	fn glFramebufferParameteri(&self, target: GLenum, pname: GLenum, param: GLint) {
		(self.framebufferparameteri)(target, pname, param)
	}
	#[inline(always)]
	fn glGetFramebufferParameteriv(&self, target: GLenum, pname: GLenum, params: *mut GLint) {
		(self.getframebufferparameteriv)(target, pname, params)
	}
	#[inline(always)]
	fn glGetInternalformati64v(&self, target: GLenum, internalformat: GLenum, pname: GLenum, count: GLsizei, params: *mut GLint64) {
		(self.getinternalformati64v)(target, internalformat, pname, count, params)
	}
	#[inline(always)]
	fn glInvalidateTexSubImage(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei) {
		(self.invalidatetexsubimage)(texture, level, xoffset, yoffset, zoffset, width, height, depth)
	}
	#[inline(always)]
	fn glInvalidateTexImage(&self, texture: GLuint, level: GLint) {
		(self.invalidateteximage)(texture, level)
	}
	#[inline(always)]
	fn glInvalidateBufferSubData(&self, buffer: GLuint, offset: GLintptr, length: GLsizeiptr) {
		(self.invalidatebuffersubdata)(buffer, offset, length)
	}
	#[inline(always)]
	fn glInvalidateBufferData(&self, buffer: GLuint) {
		(self.invalidatebufferdata)(buffer)
	}
	#[inline(always)]
	fn glInvalidateFramebuffer(&self, target: GLenum, numAttachments: GLsizei, attachments: *const GLenum) {
		(self.invalidateframebuffer)(target, numAttachments, attachments)
	}
	#[inline(always)]
	fn glInvalidateSubFramebuffer(&self, target: GLenum, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
		(self.invalidatesubframebuffer)(target, numAttachments, attachments, x, y, width, height)
	}
	#[inline(always)]
	fn glMultiDrawArraysIndirect(&self, mode: GLenum, indirect: *const c_void, drawcount: GLsizei, stride: GLsizei) {
		(self.multidrawarraysindirect)(mode, indirect, drawcount, stride)
	}
	#[inline(always)]
	fn glMultiDrawElementsIndirect(&self, mode: GLenum, type_: GLenum, indirect: *const c_void, drawcount: GLsizei, stride: GLsizei) {
		(self.multidrawelementsindirect)(mode, type_, indirect, drawcount, stride)
	}
	#[inline(always)]
	fn glGetProgramInterfaceiv(&self, program: GLuint, programInterface: GLenum, pname: GLenum, params: *mut GLint) {
		(self.getprograminterfaceiv)(program, programInterface, pname, params)
	}
	#[inline(always)]
	fn glGetProgramResourceIndex(&self, program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLuint {
		(self.getprogramresourceindex)(program, programInterface, name)
	}
	#[inline(always)]
	fn glGetProgramResourceName(&self, program: GLuint, programInterface: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar) {
		(self.getprogramresourcename)(program, programInterface, index, bufSize, length, name)
	}
	#[inline(always)]
	fn glGetProgramResourceiv(&self, program: GLuint, programInterface: GLenum, index: GLuint, propCount: GLsizei, props: *const GLenum, count: GLsizei, length: *mut GLsizei, params: *mut GLint) {
		(self.getprogramresourceiv)(program, programInterface, index, propCount, props, count, length, params)
	}
	#[inline(always)]
	fn glGetProgramResourceLocation(&self, program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLint {
		(self.getprogramresourcelocation)(program, programInterface, name)
	}
	#[inline(always)]
	fn glGetProgramResourceLocationIndex(&self, program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLint {
		(self.getprogramresourcelocationindex)(program, programInterface, name)
	}
	#[inline(always)]
	fn glShaderStorageBlockBinding(&self, program: GLuint, storageBlockIndex: GLuint, storageBlockBinding: GLuint) {
		(self.shaderstorageblockbinding)(program, storageBlockIndex, storageBlockBinding)
	}
	#[inline(always)]
	fn glTexBufferRange(&self, target: GLenum, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) {
		(self.texbufferrange)(target, internalformat, buffer, offset, size)
	}
	#[inline(always)]
	fn glTexStorage2DMultisample(&self, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) {
		(self.texstorage2dmultisample)(target, samples, internalformat, width, height, fixedsamplelocations)
	}
	#[inline(always)]
	fn glTexStorage3DMultisample(&self, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) {
		(self.texstorage3dmultisample)(target, samples, internalformat, width, height, depth, fixedsamplelocations)
	}
	#[inline(always)]
	fn glTextureView(&self, texture: GLuint, target: GLenum, origtexture: GLuint, internalformat: GLenum, minlevel: GLuint, numlevels: GLuint, minlayer: GLuint, numlayers: GLuint) {
		(self.textureview)(texture, target, origtexture, internalformat, minlevel, numlevels, minlayer, numlayers)
	}
	#[inline(always)]
	fn glBindVertexBuffer(&self, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei) {
		(self.bindvertexbuffer)(bindingindex, buffer, offset, stride)
	}
	#[inline(always)]
	fn glVertexAttribFormat(&self, attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint) {
		(self.vertexattribformat)(attribindex, size, type_, normalized, relativeoffset)
	}
	#[inline(always)]
	fn glVertexAttribIFormat(&self, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) {
		(self.vertexattribiformat)(attribindex, size, type_, relativeoffset)
	}
	#[inline(always)]
	fn glVertexAttribLFormat(&self, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) {
		(self.vertexattriblformat)(attribindex, size, type_, relativeoffset)
	}
	#[inline(always)]
	fn glVertexAttribBinding(&self, attribindex: GLuint, bindingindex: GLuint) {
		(self.vertexattribbinding)(attribindex, bindingindex)
	}
	#[inline(always)]
	fn glVertexBindingDivisor(&self, bindingindex: GLuint, divisor: GLuint) {
		(self.vertexbindingdivisor)(bindingindex, divisor)
	}
	#[inline(always)]
	fn glDebugMessageControl(&self, source: GLenum, type_: GLenum, severity: GLenum, count: GLsizei, ids: *const GLuint, enabled: GLboolean) {
		(self.debugmessagecontrol)(source, type_, severity, count, ids, enabled)
	}
	#[inline(always)]
	fn glDebugMessageInsert(&self, source: GLenum, type_: GLenum, id: GLuint, severity: GLenum, length: GLsizei, buf: *const GLchar) {
		(self.debugmessageinsert)(source, type_, id, severity, length, buf)
	}
	#[inline(always)]
	fn glDebugMessageCallback(&self, callback: GLDEBUGPROC, userParam: *const c_void) {
		(self.debugmessagecallback)(callback, userParam)
	}
	#[inline(always)]
	fn glGetDebugMessageLog(&self, count: GLuint, bufSize: GLsizei, sources: *mut GLenum, types: *mut GLenum, ids: *mut GLuint, severities: *mut GLenum, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint {
		(self.getdebugmessagelog)(count, bufSize, sources, types, ids, severities, lengths, messageLog)
	}
	#[inline(always)]
	fn glPushDebugGroup(&self, source: GLenum, id: GLuint, length: GLsizei, message: *const GLchar) {
		(self.pushdebuggroup)(source, id, length, message)
	}
	#[inline(always)]
	fn glPopDebugGroup(&self) {
		(self.popdebuggroup)()
	}
	#[inline(always)]
	fn glObjectLabel(&self, identifier: GLenum, name: GLuint, length: GLsizei, label: *const GLchar) {
		(self.objectlabel)(identifier, name, length, label)
	}
	#[inline(always)]
	fn glGetObjectLabel(&self, identifier: GLenum, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) {
		(self.getobjectlabel)(identifier, name, bufSize, length, label)
	}
	#[inline(always)]
	fn glObjectPtrLabel(&self, ptr: *const c_void, length: GLsizei, label: *const GLchar) {
		(self.objectptrlabel)(ptr, length, label)
	}
	#[inline(always)]
	fn glGetObjectPtrLabel(&self, ptr: *const c_void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) {
		(self.getobjectptrlabel)(ptr, bufSize, length, label)
	}
}

impl Version43 {
	pub fn new(base: impl GL_1_0, get_proc_address: &impl Fn(&'static str) -> *const c_void) -> Self {
		let (spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 40300 {
			return Self::default();
		}
		Self {
			available: true,
			clearbufferdata: unsafe{transmute(get_proc_address("glClearBufferData"))},
			clearbuffersubdata: unsafe{transmute(get_proc_address("glClearBufferSubData"))},
			dispatchcompute: unsafe{transmute(get_proc_address("glDispatchCompute"))},
			dispatchcomputeindirect: unsafe{transmute(get_proc_address("glDispatchComputeIndirect"))},
			copyimagesubdata: unsafe{transmute(get_proc_address("glCopyImageSubData"))},
			framebufferparameteri: unsafe{transmute(get_proc_address("glFramebufferParameteri"))},
			getframebufferparameteriv: unsafe{transmute(get_proc_address("glGetFramebufferParameteriv"))},
			getinternalformati64v: unsafe{transmute(get_proc_address("glGetInternalformati64v"))},
			invalidatetexsubimage: unsafe{transmute(get_proc_address("glInvalidateTexSubImage"))},
			invalidateteximage: unsafe{transmute(get_proc_address("glInvalidateTexImage"))},
			invalidatebuffersubdata: unsafe{transmute(get_proc_address("glInvalidateBufferSubData"))},
			invalidatebufferdata: unsafe{transmute(get_proc_address("glInvalidateBufferData"))},
			invalidateframebuffer: unsafe{transmute(get_proc_address("glInvalidateFramebuffer"))},
			invalidatesubframebuffer: unsafe{transmute(get_proc_address("glInvalidateSubFramebuffer"))},
			multidrawarraysindirect: unsafe{transmute(get_proc_address("glMultiDrawArraysIndirect"))},
			multidrawelementsindirect: unsafe{transmute(get_proc_address("glMultiDrawElementsIndirect"))},
			getprograminterfaceiv: unsafe{transmute(get_proc_address("glGetProgramInterfaceiv"))},
			getprogramresourceindex: unsafe{transmute(get_proc_address("glGetProgramResourceIndex"))},
			getprogramresourcename: unsafe{transmute(get_proc_address("glGetProgramResourceName"))},
			getprogramresourceiv: unsafe{transmute(get_proc_address("glGetProgramResourceiv"))},
			getprogramresourcelocation: unsafe{transmute(get_proc_address("glGetProgramResourceLocation"))},
			getprogramresourcelocationindex: unsafe{transmute(get_proc_address("glGetProgramResourceLocationIndex"))},
			shaderstorageblockbinding: unsafe{transmute(get_proc_address("glShaderStorageBlockBinding"))},
			texbufferrange: unsafe{transmute(get_proc_address("glTexBufferRange"))},
			texstorage2dmultisample: unsafe{transmute(get_proc_address("glTexStorage2DMultisample"))},
			texstorage3dmultisample: unsafe{transmute(get_proc_address("glTexStorage3DMultisample"))},
			textureview: unsafe{transmute(get_proc_address("glTextureView"))},
			bindvertexbuffer: unsafe{transmute(get_proc_address("glBindVertexBuffer"))},
			vertexattribformat: unsafe{transmute(get_proc_address("glVertexAttribFormat"))},
			vertexattribiformat: unsafe{transmute(get_proc_address("glVertexAttribIFormat"))},
			vertexattriblformat: unsafe{transmute(get_proc_address("glVertexAttribLFormat"))},
			vertexattribbinding: unsafe{transmute(get_proc_address("glVertexAttribBinding"))},
			vertexbindingdivisor: unsafe{transmute(get_proc_address("glVertexBindingDivisor"))},
			debugmessagecontrol: unsafe{transmute(get_proc_address("glDebugMessageControl"))},
			debugmessageinsert: unsafe{transmute(get_proc_address("glDebugMessageInsert"))},
			debugmessagecallback: unsafe{transmute(get_proc_address("glDebugMessageCallback"))},
			getdebugmessagelog: unsafe{transmute(get_proc_address("glGetDebugMessageLog"))},
			pushdebuggroup: unsafe{transmute(get_proc_address("glPushDebugGroup"))},
			popdebuggroup: unsafe{transmute(get_proc_address("glPopDebugGroup"))},
			objectlabel: unsafe{transmute(get_proc_address("glObjectLabel"))},
			getobjectlabel: unsafe{transmute(get_proc_address("glGetObjectLabel"))},
			objectptrlabel: unsafe{transmute(get_proc_address("glObjectPtrLabel"))},
			getobjectptrlabel: unsafe{transmute(get_proc_address("glGetObjectPtrLabel"))},
		}
	}
	#[inline(always)]
	pub fn get_available(&self) -> bool {
		self.available
	}
}

impl Default for Version43 {
	fn default() -> Self {
		Self {
			available: false,
			clearbufferdata: null(),
			clearbuffersubdata: null(),
			dispatchcompute: null(),
			dispatchcomputeindirect: null(),
			copyimagesubdata: null(),
			framebufferparameteri: null(),
			getframebufferparameteriv: null(),
			getinternalformati64v: null(),
			invalidatetexsubimage: null(),
			invalidateteximage: null(),
			invalidatebuffersubdata: null(),
			invalidatebufferdata: null(),
			invalidateframebuffer: null(),
			invalidatesubframebuffer: null(),
			multidrawarraysindirect: null(),
			multidrawelementsindirect: null(),
			getprograminterfaceiv: null(),
			getprogramresourceindex: null(),
			getprogramresourcename: null(),
			getprogramresourceiv: null(),
			getprogramresourcelocation: null(),
			getprogramresourcelocationindex: null(),
			shaderstorageblockbinding: null(),
			texbufferrange: null(),
			texstorage2dmultisample: null(),
			texstorage3dmultisample: null(),
			textureview: null(),
			bindvertexbuffer: null(),
			vertexattribformat: null(),
			vertexattribiformat: null(),
			vertexattriblformat: null(),
			vertexattribbinding: null(),
			vertexbindingdivisor: null(),
			debugmessagecontrol: null(),
			debugmessageinsert: null(),
			debugmessagecallback: null(),
			getdebugmessagelog: null(),
			pushdebuggroup: null(),
			popdebuggroup: null(),
			objectlabel: null(),
			getobjectlabel: null(),
			objectptrlabel: null(),
			getobjectptrlabel: null(),
		}
	}
}

type PFNGLBUFFERSTORAGEPROC = extern "system" fn(GLenum, GLsizeiptr, *const c_void, GLbitfield);
type PFNGLCLEARTEXIMAGEPROC = extern "system" fn(GLuint, GLint, GLenum, GLenum, *const c_void);
type PFNGLCLEARTEXSUBIMAGEPROC = extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLenum, *const c_void);
type PFNGLBINDBUFFERSBASEPROC = extern "system" fn(GLenum, GLuint, GLsizei, *const GLuint);
type PFNGLBINDBUFFERSRANGEPROC = extern "system" fn(GLenum, GLuint, GLsizei, *const GLuint, *const GLintptr, *const GLsizeiptr);
type PFNGLBINDTEXTURESPROC = extern "system" fn(GLuint, GLsizei, *const GLuint);
type PFNGLBINDSAMPLERSPROC = extern "system" fn(GLuint, GLsizei, *const GLuint);
type PFNGLBINDIMAGETEXTURESPROC = extern "system" fn(GLuint, GLsizei, *const GLuint);
type PFNGLBINDVERTEXBUFFERSPROC = extern "system" fn(GLuint, GLsizei, *const GLuint, *const GLintptr, *const GLsizei);
const MAX_VERTEX_ATTRIB_STRIDE: GLenum = 0x82E5;
const PRIMITIVE_RESTART_FOR_PATCHES_SUPPORTED: GLenum = 0x8221;
const TEXTURE_BUFFER_BINDING: GLenum = 0x8C2A;
const MAP_PERSISTENT_BIT: GLbitfield = 0x0040;
const MAP_COHERENT_BIT: GLbitfield = 0x0080;
const DYNAMIC_STORAGE_BIT: GLbitfield = 0x0100;
const CLIENT_STORAGE_BIT: GLbitfield = 0x0200;
const CLIENT_MAPPED_BUFFER_BARRIER_BIT: GLbitfield = 0x00004000;
const BUFFER_IMMUTABLE_STORAGE: GLenum = 0x821F;
const BUFFER_STORAGE_FLAGS: GLenum = 0x8220;
const CLEAR_TEXTURE: GLenum = 0x9365;
const LOCATION_COMPONENT: GLenum = 0x934A;
const TRANSFORM_FEEDBACK_BUFFER_INDEX: GLenum = 0x934B;
const TRANSFORM_FEEDBACK_BUFFER_STRIDE: GLenum = 0x934C;
const QUERY_BUFFER: GLenum = 0x9192;
const QUERY_BUFFER_BARRIER_BIT: GLbitfield = 0x00008000;
const QUERY_BUFFER_BINDING: GLenum = 0x9193;
const QUERY_RESULT_NO_WAIT: GLenum = 0x9194;
const MIRROR_CLAMP_TO_EDGE: GLenum = 0x8743;

pub trait GL_4_4 {
	fn glBufferStorage(&self, _: GLenum, _: GLsizeiptr, _: *const c_void, _: GLbitfield) {
		panic!("OpenGL function pointer of `glBufferStorage` is NULL");
	}
	fn glClearTexImage(&self, _: GLuint, _: GLint, _: GLenum, _: GLenum, _: *const c_void) {
		panic!("OpenGL function pointer of `glClearTexImage` is NULL");
	}
	fn glClearTexSubImage(&self, _: GLuint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLsizei, _: GLenum, _: GLenum, _: *const c_void) {
		panic!("OpenGL function pointer of `glClearTexSubImage` is NULL");
	}
	fn glBindBuffersBase(&self, _: GLenum, _: GLuint, _: GLsizei, _: *const GLuint) {
		panic!("OpenGL function pointer of `glBindBuffersBase` is NULL");
	}
	fn glBindBuffersRange(&self, _: GLenum, _: GLuint, _: GLsizei, _: *const GLuint, _: *const GLintptr, _: *const GLsizeiptr) {
		panic!("OpenGL function pointer of `glBindBuffersRange` is NULL");
	}
	fn glBindTextures(&self, _: GLuint, _: GLsizei, _: *const GLuint) {
		panic!("OpenGL function pointer of `glBindTextures` is NULL");
	}
	fn glBindSamplers(&self, _: GLuint, _: GLsizei, _: *const GLuint) {
		panic!("OpenGL function pointer of `glBindSamplers` is NULL");
	}
	fn glBindImageTextures(&self, _: GLuint, _: GLsizei, _: *const GLuint) {
		panic!("OpenGL function pointer of `glBindImageTextures` is NULL");
	}
	fn glBindVertexBuffers(&self, _: GLuint, _: GLsizei, _: *const GLuint, _: *const GLintptr, _: *const GLsizei) {
		panic!("OpenGL function pointer of `glBindVertexBuffers` is NULL");
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Version44 {
	available: bool,
	bufferstorage: PFNGLBUFFERSTORAGEPROC,
	clearteximage: PFNGLCLEARTEXIMAGEPROC,
	cleartexsubimage: PFNGLCLEARTEXSUBIMAGEPROC,
	bindbuffersbase: PFNGLBINDBUFFERSBASEPROC,
	bindbuffersrange: PFNGLBINDBUFFERSRANGEPROC,
	bindtextures: PFNGLBINDTEXTURESPROC,
	bindsamplers: PFNGLBINDSAMPLERSPROC,
	bindimagetextures: PFNGLBINDIMAGETEXTURESPROC,
	bindvertexbuffers: PFNGLBINDVERTEXBUFFERSPROC,
}

impl GL_4_4 for Version44 {
	#[inline(always)]
	fn glBufferStorage(&self, target: GLenum, size: GLsizeiptr, data: *const c_void, flags: GLbitfield) {
		(self.bufferstorage)(target, size, data, flags)
	}
	#[inline(always)]
	fn glClearTexImage(&self, texture: GLuint, level: GLint, format: GLenum, type_: GLenum, data: *const c_void) {
		(self.clearteximage)(texture, level, format, type_, data)
	}
	#[inline(always)]
	fn glClearTexSubImage(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, data: *const c_void) {
		(self.cleartexsubimage)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, data)
	}
	#[inline(always)]
	fn glBindBuffersBase(&self, target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint) {
		(self.bindbuffersbase)(target, first, count, buffers)
	}
	#[inline(always)]
	fn glBindBuffersRange(&self, target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, sizes: *const GLsizeiptr) {
		(self.bindbuffersrange)(target, first, count, buffers, offsets, sizes)
	}
	#[inline(always)]
	fn glBindTextures(&self, first: GLuint, count: GLsizei, textures: *const GLuint) {
		(self.bindtextures)(first, count, textures)
	}
	#[inline(always)]
	fn glBindSamplers(&self, first: GLuint, count: GLsizei, samplers: *const GLuint) {
		(self.bindsamplers)(first, count, samplers)
	}
	#[inline(always)]
	fn glBindImageTextures(&self, first: GLuint, count: GLsizei, textures: *const GLuint) {
		(self.bindimagetextures)(first, count, textures)
	}
	#[inline(always)]
	fn glBindVertexBuffers(&self, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei) {
		(self.bindvertexbuffers)(first, count, buffers, offsets, strides)
	}
}

impl Version44 {
	pub fn new(base: impl GL_1_0, get_proc_address: &impl Fn(&'static str) -> *const c_void) -> Self {
		let (spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 40400 {
			return Self::default();
		}
		Self {
			available: true,
			bufferstorage: unsafe{transmute(get_proc_address("glBufferStorage"))},
			clearteximage: unsafe{transmute(get_proc_address("glClearTexImage"))},
			cleartexsubimage: unsafe{transmute(get_proc_address("glClearTexSubImage"))},
			bindbuffersbase: unsafe{transmute(get_proc_address("glBindBuffersBase"))},
			bindbuffersrange: unsafe{transmute(get_proc_address("glBindBuffersRange"))},
			bindtextures: unsafe{transmute(get_proc_address("glBindTextures"))},
			bindsamplers: unsafe{transmute(get_proc_address("glBindSamplers"))},
			bindimagetextures: unsafe{transmute(get_proc_address("glBindImageTextures"))},
			bindvertexbuffers: unsafe{transmute(get_proc_address("glBindVertexBuffers"))},
		}
	}
	#[inline(always)]
	pub fn get_available(&self) -> bool {
		self.available
	}
}

impl Default for Version44 {
	fn default() -> Self {
		Self {
			available: false,
			bufferstorage: null(),
			clearteximage: null(),
			cleartexsubimage: null(),
			bindbuffersbase: null(),
			bindbuffersrange: null(),
			bindtextures: null(),
			bindsamplers: null(),
			bindimagetextures: null(),
			bindvertexbuffers: null(),
		}
	}
}

type PFNGLCLIPCONTROLPROC = extern "system" fn(GLenum, GLenum);
type PFNGLCREATETRANSFORMFEEDBACKSPROC = extern "system" fn(GLsizei, *mut GLuint);
type PFNGLTRANSFORMFEEDBACKBUFFERBASEPROC = extern "system" fn(GLuint, GLuint, GLuint);
type PFNGLTRANSFORMFEEDBACKBUFFERRANGEPROC = extern "system" fn(GLuint, GLuint, GLuint, GLintptr, GLsizeiptr);
type PFNGLGETTRANSFORMFEEDBACKIVPROC = extern "system" fn(GLuint, GLenum, *mut GLint);
type PFNGLGETTRANSFORMFEEDBACKI_VPROC = extern "system" fn(GLuint, GLenum, GLuint, *mut GLint);
type PFNGLGETTRANSFORMFEEDBACKI64_VPROC = extern "system" fn(GLuint, GLenum, GLuint, *mut GLint64);
type PFNGLCREATEBUFFERSPROC = extern "system" fn(GLsizei, *mut GLuint);
type PFNGLNAMEDBUFFERSTORAGEPROC = extern "system" fn(GLuint, GLsizeiptr, *const c_void, GLbitfield);
type PFNGLNAMEDBUFFERDATAPROC = extern "system" fn(GLuint, GLsizeiptr, *const c_void, GLenum);
type PFNGLNAMEDBUFFERSUBDATAPROC = extern "system" fn(GLuint, GLintptr, GLsizeiptr, *const c_void);
type PFNGLCOPYNAMEDBUFFERSUBDATAPROC = extern "system" fn(GLuint, GLuint, GLintptr, GLintptr, GLsizeiptr);
type PFNGLCLEARNAMEDBUFFERDATAPROC = extern "system" fn(GLuint, GLenum, GLenum, GLenum, *const c_void);
type PFNGLCLEARNAMEDBUFFERSUBDATAPROC = extern "system" fn(GLuint, GLenum, GLintptr, GLsizeiptr, GLenum, GLenum, *const c_void);
type PFNGLMAPNAMEDBUFFERPROC = extern "system" fn(GLuint, GLenum) -> *mut c_void;
type PFNGLMAPNAMEDBUFFERRANGEPROC = extern "system" fn(GLuint, GLintptr, GLsizeiptr, GLbitfield) -> *mut c_void;
type PFNGLUNMAPNAMEDBUFFERPROC = extern "system" fn(GLuint) -> GLboolean;
type PFNGLFLUSHMAPPEDNAMEDBUFFERRANGEPROC = extern "system" fn(GLuint, GLintptr, GLsizeiptr);
type PFNGLGETNAMEDBUFFERPARAMETERIVPROC = extern "system" fn(GLuint, GLenum, *mut GLint);
type PFNGLGETNAMEDBUFFERPARAMETERI64VPROC = extern "system" fn(GLuint, GLenum, *mut GLint64);
type PFNGLGETNAMEDBUFFERPOINTERVPROC = extern "system" fn(GLuint, GLenum, *mut *mut c_void);
type PFNGLGETNAMEDBUFFERSUBDATAPROC = extern "system" fn(GLuint, GLintptr, GLsizeiptr, *mut c_void);
type PFNGLCREATEFRAMEBUFFERSPROC = extern "system" fn(GLsizei, *mut GLuint);
type PFNGLNAMEDFRAMEBUFFERRENDERBUFFERPROC = extern "system" fn(GLuint, GLenum, GLenum, GLuint);
type PFNGLNAMEDFRAMEBUFFERPARAMETERIPROC = extern "system" fn(GLuint, GLenum, GLint);
type PFNGLNAMEDFRAMEBUFFERTEXTUREPROC = extern "system" fn(GLuint, GLenum, GLuint, GLint);
type PFNGLNAMEDFRAMEBUFFERTEXTURELAYERPROC = extern "system" fn(GLuint, GLenum, GLuint, GLint, GLint);
type PFNGLNAMEDFRAMEBUFFERDRAWBUFFERPROC = extern "system" fn(GLuint, GLenum);
type PFNGLNAMEDFRAMEBUFFERDRAWBUFFERSPROC = extern "system" fn(GLuint, GLsizei, *const GLenum);
type PFNGLNAMEDFRAMEBUFFERREADBUFFERPROC = extern "system" fn(GLuint, GLenum);
type PFNGLINVALIDATENAMEDFRAMEBUFFERDATAPROC = extern "system" fn(GLuint, GLsizei, *const GLenum);
type PFNGLINVALIDATENAMEDFRAMEBUFFERSUBDATAPROC = extern "system" fn(GLuint, GLsizei, *const GLenum, GLint, GLint, GLsizei, GLsizei);
type PFNGLCLEARNAMEDFRAMEBUFFERIVPROC = extern "system" fn(GLuint, GLenum, GLint, *const GLint);
type PFNGLCLEARNAMEDFRAMEBUFFERUIVPROC = extern "system" fn(GLuint, GLenum, GLint, *const GLuint);
type PFNGLCLEARNAMEDFRAMEBUFFERFVPROC = extern "system" fn(GLuint, GLenum, GLint, *const GLfloat);
type PFNGLCLEARNAMEDFRAMEBUFFERFIPROC = extern "system" fn(GLuint, GLenum, GLint, GLfloat, GLint);
type PFNGLBLITNAMEDFRAMEBUFFERPROC = extern "system" fn(GLuint, GLuint, GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLint, GLbitfield, GLenum);
type PFNGLCHECKNAMEDFRAMEBUFFERSTATUSPROC = extern "system" fn(GLuint, GLenum) -> GLenum;
type PFNGLGETNAMEDFRAMEBUFFERPARAMETERIVPROC = extern "system" fn(GLuint, GLenum, *mut GLint);
type PFNGLGETNAMEDFRAMEBUFFERATTACHMENTPARAMETERIVPROC = extern "system" fn(GLuint, GLenum, GLenum, *mut GLint);
type PFNGLCREATERENDERBUFFERSPROC = extern "system" fn(GLsizei, *mut GLuint);
type PFNGLNAMEDRENDERBUFFERSTORAGEPROC = extern "system" fn(GLuint, GLenum, GLsizei, GLsizei);
type PFNGLNAMEDRENDERBUFFERSTORAGEMULTISAMPLEPROC = extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei);
type PFNGLGETNAMEDRENDERBUFFERPARAMETERIVPROC = extern "system" fn(GLuint, GLenum, *mut GLint);
type PFNGLCREATETEXTURESPROC = extern "system" fn(GLenum, GLsizei, *mut GLuint);
type PFNGLTEXTUREBUFFERPROC = extern "system" fn(GLuint, GLenum, GLuint);
type PFNGLTEXTUREBUFFERRANGEPROC = extern "system" fn(GLuint, GLenum, GLuint, GLintptr, GLsizeiptr);
type PFNGLTEXTURESTORAGE1DPROC = extern "system" fn(GLuint, GLsizei, GLenum, GLsizei);
type PFNGLTEXTURESTORAGE2DPROC = extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei);
type PFNGLTEXTURESTORAGE3DPROC = extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei, GLsizei);
type PFNGLTEXTURESTORAGE2DMULTISAMPLEPROC = extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei, GLboolean);
type PFNGLTEXTURESTORAGE3DMULTISAMPLEPROC = extern "system" fn(GLuint, GLsizei, GLenum, GLsizei, GLsizei, GLsizei, GLboolean);
type PFNGLTEXTURESUBIMAGE1DPROC = extern "system" fn(GLuint, GLint, GLint, GLsizei, GLenum, GLenum, *const c_void);
type PFNGLTEXTURESUBIMAGE2DPROC = extern "system" fn(GLuint, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *const c_void);
type PFNGLTEXTURESUBIMAGE3DPROC = extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLenum, *const c_void);
type PFNGLCOMPRESSEDTEXTURESUBIMAGE1DPROC = extern "system" fn(GLuint, GLint, GLint, GLsizei, GLenum, GLsizei, *const c_void);
type PFNGLCOMPRESSEDTEXTURESUBIMAGE2DPROC = extern "system" fn(GLuint, GLint, GLint, GLint, GLsizei, GLsizei, GLenum, GLsizei, *const c_void);
type PFNGLCOMPRESSEDTEXTURESUBIMAGE3DPROC = extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLsizei, *const c_void);
type PFNGLCOPYTEXTURESUBIMAGE1DPROC = extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei);
type PFNGLCOPYTEXTURESUBIMAGE2DPROC = extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei);
type PFNGLCOPYTEXTURESUBIMAGE3DPROC = extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei);
type PFNGLTEXTUREPARAMETERFPROC = extern "system" fn(GLuint, GLenum, GLfloat);
type PFNGLTEXTUREPARAMETERFVPROC = extern "system" fn(GLuint, GLenum, *const GLfloat);
type PFNGLTEXTUREPARAMETERIPROC = extern "system" fn(GLuint, GLenum, GLint);
type PFNGLTEXTUREPARAMETERIIVPROC = extern "system" fn(GLuint, GLenum, *const GLint);
type PFNGLTEXTUREPARAMETERIUIVPROC = extern "system" fn(GLuint, GLenum, *const GLuint);
type PFNGLTEXTUREPARAMETERIVPROC = extern "system" fn(GLuint, GLenum, *const GLint);
type PFNGLGENERATETEXTUREMIPMAPPROC = extern "system" fn(GLuint);
type PFNGLBINDTEXTUREUNITPROC = extern "system" fn(GLuint, GLuint);
type PFNGLGETTEXTUREIMAGEPROC = extern "system" fn(GLuint, GLint, GLenum, GLenum, GLsizei, *mut c_void);
type PFNGLGETCOMPRESSEDTEXTUREIMAGEPROC = extern "system" fn(GLuint, GLint, GLsizei, *mut c_void);
type PFNGLGETTEXTURELEVELPARAMETERFVPROC = extern "system" fn(GLuint, GLint, GLenum, *mut GLfloat);
type PFNGLGETTEXTURELEVELPARAMETERIVPROC = extern "system" fn(GLuint, GLint, GLenum, *mut GLint);
type PFNGLGETTEXTUREPARAMETERFVPROC = extern "system" fn(GLuint, GLenum, *mut GLfloat);
type PFNGLGETTEXTUREPARAMETERIIVPROC = extern "system" fn(GLuint, GLenum, *mut GLint);
type PFNGLGETTEXTUREPARAMETERIUIVPROC = extern "system" fn(GLuint, GLenum, *mut GLuint);
type PFNGLGETTEXTUREPARAMETERIVPROC = extern "system" fn(GLuint, GLenum, *mut GLint);
type PFNGLCREATEVERTEXARRAYSPROC = extern "system" fn(GLsizei, *mut GLuint);
type PFNGLDISABLEVERTEXARRAYATTRIBPROC = extern "system" fn(GLuint, GLuint);
type PFNGLENABLEVERTEXARRAYATTRIBPROC = extern "system" fn(GLuint, GLuint);
type PFNGLVERTEXARRAYELEMENTBUFFERPROC = extern "system" fn(GLuint, GLuint);
type PFNGLVERTEXARRAYVERTEXBUFFERPROC = extern "system" fn(GLuint, GLuint, GLuint, GLintptr, GLsizei);
type PFNGLVERTEXARRAYVERTEXBUFFERSPROC = extern "system" fn(GLuint, GLuint, GLsizei, *const GLuint, *const GLintptr, *const GLsizei);
type PFNGLVERTEXARRAYATTRIBBINDINGPROC = extern "system" fn(GLuint, GLuint, GLuint);
type PFNGLVERTEXARRAYATTRIBFORMATPROC = extern "system" fn(GLuint, GLuint, GLint, GLenum, GLboolean, GLuint);
type PFNGLVERTEXARRAYATTRIBIFORMATPROC = extern "system" fn(GLuint, GLuint, GLint, GLenum, GLuint);
type PFNGLVERTEXARRAYATTRIBLFORMATPROC = extern "system" fn(GLuint, GLuint, GLint, GLenum, GLuint);
type PFNGLVERTEXARRAYBINDINGDIVISORPROC = extern "system" fn(GLuint, GLuint, GLuint);
type PFNGLGETVERTEXARRAYIVPROC = extern "system" fn(GLuint, GLenum, *mut GLint);
type PFNGLGETVERTEXARRAYINDEXEDIVPROC = extern "system" fn(GLuint, GLuint, GLenum, *mut GLint);
type PFNGLGETVERTEXARRAYINDEXED64IVPROC = extern "system" fn(GLuint, GLuint, GLenum, *mut GLint64);
type PFNGLCREATESAMPLERSPROC = extern "system" fn(GLsizei, *mut GLuint);
type PFNGLCREATEPROGRAMPIPELINESPROC = extern "system" fn(GLsizei, *mut GLuint);
type PFNGLCREATEQUERIESPROC = extern "system" fn(GLenum, GLsizei, *mut GLuint);
type PFNGLGETQUERYBUFFEROBJECTI64VPROC = extern "system" fn(GLuint, GLuint, GLenum, GLintptr);
type PFNGLGETQUERYBUFFEROBJECTIVPROC = extern "system" fn(GLuint, GLuint, GLenum, GLintptr);
type PFNGLGETQUERYBUFFEROBJECTUI64VPROC = extern "system" fn(GLuint, GLuint, GLenum, GLintptr);
type PFNGLGETQUERYBUFFEROBJECTUIVPROC = extern "system" fn(GLuint, GLuint, GLenum, GLintptr);
type PFNGLMEMORYBARRIERBYREGIONPROC = extern "system" fn(GLbitfield);
type PFNGLGETTEXTURESUBIMAGEPROC = extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLenum, GLsizei, *mut c_void);
type PFNGLGETCOMPRESSEDTEXTURESUBIMAGEPROC = extern "system" fn(GLuint, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLsizei, *mut c_void);
type PFNGLGETGRAPHICSRESETSTATUSPROC = extern "system" fn() -> GLenum;
type PFNGLGETNCOMPRESSEDTEXIMAGEPROC = extern "system" fn(GLenum, GLint, GLsizei, *mut c_void);
type PFNGLGETNTEXIMAGEPROC = extern "system" fn(GLenum, GLint, GLenum, GLenum, GLsizei, *mut c_void);
type PFNGLGETNUNIFORMDVPROC = extern "system" fn(GLuint, GLint, GLsizei, *mut GLdouble);
type PFNGLGETNUNIFORMFVPROC = extern "system" fn(GLuint, GLint, GLsizei, *mut GLfloat);
type PFNGLGETNUNIFORMIVPROC = extern "system" fn(GLuint, GLint, GLsizei, *mut GLint);
type PFNGLGETNUNIFORMUIVPROC = extern "system" fn(GLuint, GLint, GLsizei, *mut GLuint);
type PFNGLREADNPIXELSPROC = extern "system" fn(GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, GLsizei, *mut c_void);
type PFNGLGETNMAPDVPROC = extern "system" fn(GLenum, GLenum, GLsizei, *mut GLdouble);
type PFNGLGETNMAPFVPROC = extern "system" fn(GLenum, GLenum, GLsizei, *mut GLfloat);
type PFNGLGETNMAPIVPROC = extern "system" fn(GLenum, GLenum, GLsizei, *mut GLint);
type PFNGLGETNPIXELMAPFVPROC = extern "system" fn(GLenum, GLsizei, *mut GLfloat);
type PFNGLGETNPIXELMAPUIVPROC = extern "system" fn(GLenum, GLsizei, *mut GLuint);
type PFNGLGETNPIXELMAPUSVPROC = extern "system" fn(GLenum, GLsizei, *mut GLushort);
type PFNGLGETNPOLYGONSTIPPLEPROC = extern "system" fn(GLsizei, *mut GLubyte);
type PFNGLGETNCOLORTABLEPROC = extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut c_void);
type PFNGLGETNCONVOLUTIONFILTERPROC = extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut c_void);
type PFNGLGETNSEPARABLEFILTERPROC = extern "system" fn(GLenum, GLenum, GLenum, GLsizei, *mut c_void, GLsizei, *mut c_void, *mut c_void);
type PFNGLGETNHISTOGRAMPROC = extern "system" fn(GLenum, GLboolean, GLenum, GLenum, GLsizei, *mut c_void);
type PFNGLGETNMINMAXPROC = extern "system" fn(GLenum, GLboolean, GLenum, GLenum, GLsizei, *mut c_void);
type PFNGLTEXTUREBARRIERPROC = extern "system" fn();
const CONTEXT_LOST: GLenum = 0x0507;
const NEGATIVE_ONE_TO_ONE: GLenum = 0x935E;
const ZERO_TO_ONE: GLenum = 0x935F;
const CLIP_ORIGIN: GLenum = 0x935C;
const CLIP_DEPTH_MODE: GLenum = 0x935D;
const QUERY_WAIT_INVERTED: GLenum = 0x8E17;
const QUERY_NO_WAIT_INVERTED: GLenum = 0x8E18;
const QUERY_BY_REGION_WAIT_INVERTED: GLenum = 0x8E19;
const QUERY_BY_REGION_NO_WAIT_INVERTED: GLenum = 0x8E1A;
const MAX_CULL_DISTANCES: GLenum = 0x82F9;
const MAX_COMBINED_CLIP_AND_CULL_DISTANCES: GLenum = 0x82FA;
const TEXTURE_TARGET: GLenum = 0x1006;
const QUERY_TARGET: GLenum = 0x82EA;
const GUILTY_CONTEXT_RESET: GLenum = 0x8253;
const INNOCENT_CONTEXT_RESET: GLenum = 0x8254;
const UNKNOWN_CONTEXT_RESET: GLenum = 0x8255;
const RESET_NOTIFICATION_STRATEGY: GLenum = 0x8256;
const LOSE_CONTEXT_ON_RESET: GLenum = 0x8252;
const NO_RESET_NOTIFICATION: GLenum = 0x8261;
const CONTEXT_FLAG_ROBUST_ACCESS_BIT: GLbitfield = 0x00000004;
const COLOR_TABLE: GLenum = 0x80D0;
const POST_CONVOLUTION_COLOR_TABLE: GLenum = 0x80D1;
const POST_COLOR_MATRIX_COLOR_TABLE: GLenum = 0x80D2;
const PROXY_COLOR_TABLE: GLenum = 0x80D3;
const PROXY_POST_CONVOLUTION_COLOR_TABLE: GLenum = 0x80D4;
const PROXY_POST_COLOR_MATRIX_COLOR_TABLE: GLenum = 0x80D5;
const CONVOLUTION_1D: GLenum = 0x8010;
const CONVOLUTION_2D: GLenum = 0x8011;
const SEPARABLE_2D: GLenum = 0x8012;
const HISTOGRAM: GLenum = 0x8024;
const PROXY_HISTOGRAM: GLenum = 0x8025;
const MINMAX: GLenum = 0x802E;
const CONTEXT_RELEASE_BEHAVIOR: GLenum = 0x82FB;
const CONTEXT_RELEASE_BEHAVIOR_FLUSH: GLenum = 0x82FC;

pub trait GL_4_5 {
	fn glClipControl(&self, _: GLenum, _: GLenum) {
		panic!("OpenGL function pointer of `glClipControl` is NULL");
	}
	fn glCreateTransformFeedbacks(&self, _: GLsizei, _: *mut GLuint) {
		panic!("OpenGL function pointer of `glCreateTransformFeedbacks` is NULL");
	}
	fn glTransformFeedbackBufferBase(&self, _: GLuint, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glTransformFeedbackBufferBase` is NULL");
	}
	fn glTransformFeedbackBufferRange(&self, _: GLuint, _: GLuint, _: GLuint, _: GLintptr, _: GLsizeiptr) {
		panic!("OpenGL function pointer of `glTransformFeedbackBufferRange` is NULL");
	}
	fn glGetTransformFeedbackiv(&self, _: GLuint, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetTransformFeedbackiv` is NULL");
	}
	fn glGetTransformFeedbacki_v(&self, _: GLuint, _: GLenum, _: GLuint, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetTransformFeedbacki_v` is NULL");
	}
	fn glGetTransformFeedbacki64_v(&self, _: GLuint, _: GLenum, _: GLuint, _: *mut GLint64) {
		panic!("OpenGL function pointer of `glGetTransformFeedbacki64_v` is NULL");
	}
	fn glCreateBuffers(&self, _: GLsizei, _: *mut GLuint) {
		panic!("OpenGL function pointer of `glCreateBuffers` is NULL");
	}
	fn glNamedBufferStorage(&self, _: GLuint, _: GLsizeiptr, _: *const c_void, _: GLbitfield) {
		panic!("OpenGL function pointer of `glNamedBufferStorage` is NULL");
	}
	fn glNamedBufferData(&self, _: GLuint, _: GLsizeiptr, _: *const c_void, _: GLenum) {
		panic!("OpenGL function pointer of `glNamedBufferData` is NULL");
	}
	fn glNamedBufferSubData(&self, _: GLuint, _: GLintptr, _: GLsizeiptr, _: *const c_void) {
		panic!("OpenGL function pointer of `glNamedBufferSubData` is NULL");
	}
	fn glCopyNamedBufferSubData(&self, _: GLuint, _: GLuint, _: GLintptr, _: GLintptr, _: GLsizeiptr) {
		panic!("OpenGL function pointer of `glCopyNamedBufferSubData` is NULL");
	}
	fn glClearNamedBufferData(&self, _: GLuint, _: GLenum, _: GLenum, _: GLenum, _: *const c_void) {
		panic!("OpenGL function pointer of `glClearNamedBufferData` is NULL");
	}
	fn glClearNamedBufferSubData(&self, _: GLuint, _: GLenum, _: GLintptr, _: GLsizeiptr, _: GLenum, _: GLenum, _: *const c_void) {
		panic!("OpenGL function pointer of `glClearNamedBufferSubData` is NULL");
	}
	fn glMapNamedBuffer(&self, _: GLuint, _: GLenum) -> *mut c_void {
		panic!("OpenGL function pointer of `glMapNamedBuffer` is NULL");
	}
	fn glMapNamedBufferRange(&self, _: GLuint, _: GLintptr, _: GLsizeiptr, _: GLbitfield) -> *mut c_void {
		panic!("OpenGL function pointer of `glMapNamedBufferRange` is NULL");
	}
	fn glUnmapNamedBuffer(&self, _: GLuint) -> GLboolean {
		panic!("OpenGL function pointer of `glUnmapNamedBuffer` is NULL");
	}
	fn glFlushMappedNamedBufferRange(&self, _: GLuint, _: GLintptr, _: GLsizeiptr) {
		panic!("OpenGL function pointer of `glFlushMappedNamedBufferRange` is NULL");
	}
	fn glGetNamedBufferParameteriv(&self, _: GLuint, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetNamedBufferParameteriv` is NULL");
	}
	fn glGetNamedBufferParameteri64v(&self, _: GLuint, _: GLenum, _: *mut GLint64) {
		panic!("OpenGL function pointer of `glGetNamedBufferParameteri64v` is NULL");
	}
	fn glGetNamedBufferPointerv(&self, _: GLuint, _: GLenum, _: *mut *mut c_void) {
		panic!("OpenGL function pointer of `glGetNamedBufferPointerv` is NULL");
	}
	fn glGetNamedBufferSubData(&self, _: GLuint, _: GLintptr, _: GLsizeiptr, _: *mut c_void) {
		panic!("OpenGL function pointer of `glGetNamedBufferSubData` is NULL");
	}
	fn glCreateFramebuffers(&self, _: GLsizei, _: *mut GLuint) {
		panic!("OpenGL function pointer of `glCreateFramebuffers` is NULL");
	}
	fn glNamedFramebufferRenderbuffer(&self, _: GLuint, _: GLenum, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glNamedFramebufferRenderbuffer` is NULL");
	}
	fn glNamedFramebufferParameteri(&self, _: GLuint, _: GLenum, _: GLint) {
		panic!("OpenGL function pointer of `glNamedFramebufferParameteri` is NULL");
	}
	fn glNamedFramebufferTexture(&self, _: GLuint, _: GLenum, _: GLuint, _: GLint) {
		panic!("OpenGL function pointer of `glNamedFramebufferTexture` is NULL");
	}
	fn glNamedFramebufferTextureLayer(&self, _: GLuint, _: GLenum, _: GLuint, _: GLint, _: GLint) {
		panic!("OpenGL function pointer of `glNamedFramebufferTextureLayer` is NULL");
	}
	fn glNamedFramebufferDrawBuffer(&self, _: GLuint, _: GLenum) {
		panic!("OpenGL function pointer of `glNamedFramebufferDrawBuffer` is NULL");
	}
	fn glNamedFramebufferDrawBuffers(&self, _: GLuint, _: GLsizei, _: *const GLenum) {
		panic!("OpenGL function pointer of `glNamedFramebufferDrawBuffers` is NULL");
	}
	fn glNamedFramebufferReadBuffer(&self, _: GLuint, _: GLenum) {
		panic!("OpenGL function pointer of `glNamedFramebufferReadBuffer` is NULL");
	}
	fn glInvalidateNamedFramebufferData(&self, _: GLuint, _: GLsizei, _: *const GLenum) {
		panic!("OpenGL function pointer of `glInvalidateNamedFramebufferData` is NULL");
	}
	fn glInvalidateNamedFramebufferSubData(&self, _: GLuint, _: GLsizei, _: *const GLenum, _: GLint, _: GLint, _: GLsizei, _: GLsizei) {
		panic!("OpenGL function pointer of `glInvalidateNamedFramebufferSubData` is NULL");
	}
	fn glClearNamedFramebufferiv(&self, _: GLuint, _: GLenum, _: GLint, _: *const GLint) {
		panic!("OpenGL function pointer of `glClearNamedFramebufferiv` is NULL");
	}
	fn glClearNamedFramebufferuiv(&self, _: GLuint, _: GLenum, _: GLint, _: *const GLuint) {
		panic!("OpenGL function pointer of `glClearNamedFramebufferuiv` is NULL");
	}
	fn glClearNamedFramebufferfv(&self, _: GLuint, _: GLenum, _: GLint, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glClearNamedFramebufferfv` is NULL");
	}
	fn glClearNamedFramebufferfi(&self, _: GLuint, _: GLenum, _: GLint, _: GLfloat, _: GLint) {
		panic!("OpenGL function pointer of `glClearNamedFramebufferfi` is NULL");
	}
	fn glBlitNamedFramebuffer(&self, _: GLuint, _: GLuint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLbitfield, _: GLenum) {
		panic!("OpenGL function pointer of `glBlitNamedFramebuffer` is NULL");
	}
	fn glCheckNamedFramebufferStatus(&self, _: GLuint, _: GLenum) -> GLenum {
		panic!("OpenGL function pointer of `glCheckNamedFramebufferStatus` is NULL");
	}
	fn glGetNamedFramebufferParameteriv(&self, _: GLuint, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetNamedFramebufferParameteriv` is NULL");
	}
	fn glGetNamedFramebufferAttachmentParameteriv(&self, _: GLuint, _: GLenum, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetNamedFramebufferAttachmentParameteriv` is NULL");
	}
	fn glCreateRenderbuffers(&self, _: GLsizei, _: *mut GLuint) {
		panic!("OpenGL function pointer of `glCreateRenderbuffers` is NULL");
	}
	fn glNamedRenderbufferStorage(&self, _: GLuint, _: GLenum, _: GLsizei, _: GLsizei) {
		panic!("OpenGL function pointer of `glNamedRenderbufferStorage` is NULL");
	}
	fn glNamedRenderbufferStorageMultisample(&self, _: GLuint, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei) {
		panic!("OpenGL function pointer of `glNamedRenderbufferStorageMultisample` is NULL");
	}
	fn glGetNamedRenderbufferParameteriv(&self, _: GLuint, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetNamedRenderbufferParameteriv` is NULL");
	}
	fn glCreateTextures(&self, _: GLenum, _: GLsizei, _: *mut GLuint) {
		panic!("OpenGL function pointer of `glCreateTextures` is NULL");
	}
	fn glTextureBuffer(&self, _: GLuint, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glTextureBuffer` is NULL");
	}
	fn glTextureBufferRange(&self, _: GLuint, _: GLenum, _: GLuint, _: GLintptr, _: GLsizeiptr) {
		panic!("OpenGL function pointer of `glTextureBufferRange` is NULL");
	}
	fn glTextureStorage1D(&self, _: GLuint, _: GLsizei, _: GLenum, _: GLsizei) {
		panic!("OpenGL function pointer of `glTextureStorage1D` is NULL");
	}
	fn glTextureStorage2D(&self, _: GLuint, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei) {
		panic!("OpenGL function pointer of `glTextureStorage2D` is NULL");
	}
	fn glTextureStorage3D(&self, _: GLuint, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei, _: GLsizei) {
		panic!("OpenGL function pointer of `glTextureStorage3D` is NULL");
	}
	fn glTextureStorage2DMultisample(&self, _: GLuint, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei, _: GLboolean) {
		panic!("OpenGL function pointer of `glTextureStorage2DMultisample` is NULL");
	}
	fn glTextureStorage3DMultisample(&self, _: GLuint, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei, _: GLsizei, _: GLboolean) {
		panic!("OpenGL function pointer of `glTextureStorage3DMultisample` is NULL");
	}
	fn glTextureSubImage1D(&self, _: GLuint, _: GLint, _: GLint, _: GLsizei, _: GLenum, _: GLenum, _: *const c_void) {
		panic!("OpenGL function pointer of `glTextureSubImage1D` is NULL");
	}
	fn glTextureSubImage2D(&self, _: GLuint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLenum, _: GLenum, _: *const c_void) {
		panic!("OpenGL function pointer of `glTextureSubImage2D` is NULL");
	}
	fn glTextureSubImage3D(&self, _: GLuint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLsizei, _: GLenum, _: GLenum, _: *const c_void) {
		panic!("OpenGL function pointer of `glTextureSubImage3D` is NULL");
	}
	fn glCompressedTextureSubImage1D(&self, _: GLuint, _: GLint, _: GLint, _: GLsizei, _: GLenum, _: GLsizei, _: *const c_void) {
		panic!("OpenGL function pointer of `glCompressedTextureSubImage1D` is NULL");
	}
	fn glCompressedTextureSubImage2D(&self, _: GLuint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLenum, _: GLsizei, _: *const c_void) {
		panic!("OpenGL function pointer of `glCompressedTextureSubImage2D` is NULL");
	}
	fn glCompressedTextureSubImage3D(&self, _: GLuint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLsizei, _: GLenum, _: GLsizei, _: *const c_void) {
		panic!("OpenGL function pointer of `glCompressedTextureSubImage3D` is NULL");
	}
	fn glCopyTextureSubImage1D(&self, _: GLuint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei) {
		panic!("OpenGL function pointer of `glCopyTextureSubImage1D` is NULL");
	}
	fn glCopyTextureSubImage2D(&self, _: GLuint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei) {
		panic!("OpenGL function pointer of `glCopyTextureSubImage2D` is NULL");
	}
	fn glCopyTextureSubImage3D(&self, _: GLuint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei) {
		panic!("OpenGL function pointer of `glCopyTextureSubImage3D` is NULL");
	}
	fn glTextureParameterf(&self, _: GLuint, _: GLenum, _: GLfloat) {
		panic!("OpenGL function pointer of `glTextureParameterf` is NULL");
	}
	fn glTextureParameterfv(&self, _: GLuint, _: GLenum, _: *const GLfloat) {
		panic!("OpenGL function pointer of `glTextureParameterfv` is NULL");
	}
	fn glTextureParameteri(&self, _: GLuint, _: GLenum, _: GLint) {
		panic!("OpenGL function pointer of `glTextureParameteri` is NULL");
	}
	fn glTextureParameterIiv(&self, _: GLuint, _: GLenum, _: *const GLint) {
		panic!("OpenGL function pointer of `glTextureParameterIiv` is NULL");
	}
	fn glTextureParameterIuiv(&self, _: GLuint, _: GLenum, _: *const GLuint) {
		panic!("OpenGL function pointer of `glTextureParameterIuiv` is NULL");
	}
	fn glTextureParameteriv(&self, _: GLuint, _: GLenum, _: *const GLint) {
		panic!("OpenGL function pointer of `glTextureParameteriv` is NULL");
	}
	fn glGenerateTextureMipmap(&self, _: GLuint) {
		panic!("OpenGL function pointer of `glGenerateTextureMipmap` is NULL");
	}
	fn glBindTextureUnit(&self, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glBindTextureUnit` is NULL");
	}
	fn glGetTextureImage(&self, _: GLuint, _: GLint, _: GLenum, _: GLenum, _: GLsizei, _: *mut c_void) {
		panic!("OpenGL function pointer of `glGetTextureImage` is NULL");
	}
	fn glGetCompressedTextureImage(&self, _: GLuint, _: GLint, _: GLsizei, _: *mut c_void) {
		panic!("OpenGL function pointer of `glGetCompressedTextureImage` is NULL");
	}
	fn glGetTextureLevelParameterfv(&self, _: GLuint, _: GLint, _: GLenum, _: *mut GLfloat) {
		panic!("OpenGL function pointer of `glGetTextureLevelParameterfv` is NULL");
	}
	fn glGetTextureLevelParameteriv(&self, _: GLuint, _: GLint, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetTextureLevelParameteriv` is NULL");
	}
	fn glGetTextureParameterfv(&self, _: GLuint, _: GLenum, _: *mut GLfloat) {
		panic!("OpenGL function pointer of `glGetTextureParameterfv` is NULL");
	}
	fn glGetTextureParameterIiv(&self, _: GLuint, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetTextureParameterIiv` is NULL");
	}
	fn glGetTextureParameterIuiv(&self, _: GLuint, _: GLenum, _: *mut GLuint) {
		panic!("OpenGL function pointer of `glGetTextureParameterIuiv` is NULL");
	}
	fn glGetTextureParameteriv(&self, _: GLuint, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetTextureParameteriv` is NULL");
	}
	fn glCreateVertexArrays(&self, _: GLsizei, _: *mut GLuint) {
		panic!("OpenGL function pointer of `glCreateVertexArrays` is NULL");
	}
	fn glDisableVertexArrayAttrib(&self, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glDisableVertexArrayAttrib` is NULL");
	}
	fn glEnableVertexArrayAttrib(&self, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glEnableVertexArrayAttrib` is NULL");
	}
	fn glVertexArrayElementBuffer(&self, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glVertexArrayElementBuffer` is NULL");
	}
	fn glVertexArrayVertexBuffer(&self, _: GLuint, _: GLuint, _: GLuint, _: GLintptr, _: GLsizei) {
		panic!("OpenGL function pointer of `glVertexArrayVertexBuffer` is NULL");
	}
	fn glVertexArrayVertexBuffers(&self, _: GLuint, _: GLuint, _: GLsizei, _: *const GLuint, _: *const GLintptr, _: *const GLsizei) {
		panic!("OpenGL function pointer of `glVertexArrayVertexBuffers` is NULL");
	}
	fn glVertexArrayAttribBinding(&self, _: GLuint, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glVertexArrayAttribBinding` is NULL");
	}
	fn glVertexArrayAttribFormat(&self, _: GLuint, _: GLuint, _: GLint, _: GLenum, _: GLboolean, _: GLuint) {
		panic!("OpenGL function pointer of `glVertexArrayAttribFormat` is NULL");
	}
	fn glVertexArrayAttribIFormat(&self, _: GLuint, _: GLuint, _: GLint, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glVertexArrayAttribIFormat` is NULL");
	}
	fn glVertexArrayAttribLFormat(&self, _: GLuint, _: GLuint, _: GLint, _: GLenum, _: GLuint) {
		panic!("OpenGL function pointer of `glVertexArrayAttribLFormat` is NULL");
	}
	fn glVertexArrayBindingDivisor(&self, _: GLuint, _: GLuint, _: GLuint) {
		panic!("OpenGL function pointer of `glVertexArrayBindingDivisor` is NULL");
	}
	fn glGetVertexArrayiv(&self, _: GLuint, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetVertexArrayiv` is NULL");
	}
	fn glGetVertexArrayIndexediv(&self, _: GLuint, _: GLuint, _: GLenum, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetVertexArrayIndexediv` is NULL");
	}
	fn glGetVertexArrayIndexed64iv(&self, _: GLuint, _: GLuint, _: GLenum, _: *mut GLint64) {
		panic!("OpenGL function pointer of `glGetVertexArrayIndexed64iv` is NULL");
	}
	fn glCreateSamplers(&self, _: GLsizei, _: *mut GLuint) {
		panic!("OpenGL function pointer of `glCreateSamplers` is NULL");
	}
	fn glCreateProgramPipelines(&self, _: GLsizei, _: *mut GLuint) {
		panic!("OpenGL function pointer of `glCreateProgramPipelines` is NULL");
	}
	fn glCreateQueries(&self, _: GLenum, _: GLsizei, _: *mut GLuint) {
		panic!("OpenGL function pointer of `glCreateQueries` is NULL");
	}
	fn glGetQueryBufferObjecti64v(&self, _: GLuint, _: GLuint, _: GLenum, _: GLintptr) {
		panic!("OpenGL function pointer of `glGetQueryBufferObjecti64v` is NULL");
	}
	fn glGetQueryBufferObjectiv(&self, _: GLuint, _: GLuint, _: GLenum, _: GLintptr) {
		panic!("OpenGL function pointer of `glGetQueryBufferObjectiv` is NULL");
	}
	fn glGetQueryBufferObjectui64v(&self, _: GLuint, _: GLuint, _: GLenum, _: GLintptr) {
		panic!("OpenGL function pointer of `glGetQueryBufferObjectui64v` is NULL");
	}
	fn glGetQueryBufferObjectuiv(&self, _: GLuint, _: GLuint, _: GLenum, _: GLintptr) {
		panic!("OpenGL function pointer of `glGetQueryBufferObjectuiv` is NULL");
	}
	fn glMemoryBarrierByRegion(&self, _: GLbitfield) {
		panic!("OpenGL function pointer of `glMemoryBarrierByRegion` is NULL");
	}
	fn glGetTextureSubImage(&self, _: GLuint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLsizei, _: GLenum, _: GLenum, _: GLsizei, _: *mut c_void) {
		panic!("OpenGL function pointer of `glGetTextureSubImage` is NULL");
	}
	fn glGetCompressedTextureSubImage(&self, _: GLuint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLsizei, _: GLsizei, _: *mut c_void) {
		panic!("OpenGL function pointer of `glGetCompressedTextureSubImage` is NULL");
	}
	fn glGetGraphicsResetStatus(&self) -> GLenum {
		panic!("OpenGL function pointer of `glGetGraphicsResetStatus` is NULL");
	}
	fn glGetnCompressedTexImage(&self, _: GLenum, _: GLint, _: GLsizei, _: *mut c_void) {
		panic!("OpenGL function pointer of `glGetnCompressedTexImage` is NULL");
	}
	fn glGetnTexImage(&self, _: GLenum, _: GLint, _: GLenum, _: GLenum, _: GLsizei, _: *mut c_void) {
		panic!("OpenGL function pointer of `glGetnTexImage` is NULL");
	}
	fn glGetnUniformdv(&self, _: GLuint, _: GLint, _: GLsizei, _: *mut GLdouble) {
		panic!("OpenGL function pointer of `glGetnUniformdv` is NULL");
	}
	fn glGetnUniformfv(&self, _: GLuint, _: GLint, _: GLsizei, _: *mut GLfloat) {
		panic!("OpenGL function pointer of `glGetnUniformfv` is NULL");
	}
	fn glGetnUniformiv(&self, _: GLuint, _: GLint, _: GLsizei, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetnUniformiv` is NULL");
	}
	fn glGetnUniformuiv(&self, _: GLuint, _: GLint, _: GLsizei, _: *mut GLuint) {
		panic!("OpenGL function pointer of `glGetnUniformuiv` is NULL");
	}
	fn glReadnPixels(&self, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLenum, _: GLenum, _: GLsizei, _: *mut c_void) {
		panic!("OpenGL function pointer of `glReadnPixels` is NULL");
	}
	fn glGetnMapdv(&self, _: GLenum, _: GLenum, _: GLsizei, _: *mut GLdouble) {
		panic!("OpenGL function pointer of `glGetnMapdv` is NULL");
	}
	fn glGetnMapfv(&self, _: GLenum, _: GLenum, _: GLsizei, _: *mut GLfloat) {
		panic!("OpenGL function pointer of `glGetnMapfv` is NULL");
	}
	fn glGetnMapiv(&self, _: GLenum, _: GLenum, _: GLsizei, _: *mut GLint) {
		panic!("OpenGL function pointer of `glGetnMapiv` is NULL");
	}
	fn glGetnPixelMapfv(&self, _: GLenum, _: GLsizei, _: *mut GLfloat) {
		panic!("OpenGL function pointer of `glGetnPixelMapfv` is NULL");
	}
	fn glGetnPixelMapuiv(&self, _: GLenum, _: GLsizei, _: *mut GLuint) {
		panic!("OpenGL function pointer of `glGetnPixelMapuiv` is NULL");
	}
	fn glGetnPixelMapusv(&self, _: GLenum, _: GLsizei, _: *mut GLushort) {
		panic!("OpenGL function pointer of `glGetnPixelMapusv` is NULL");
	}
	fn glGetnPolygonStipple(&self, _: GLsizei, _: *mut GLubyte) {
		panic!("OpenGL function pointer of `glGetnPolygonStipple` is NULL");
	}
	fn glGetnColorTable(&self, _: GLenum, _: GLenum, _: GLenum, _: GLsizei, _: *mut c_void) {
		panic!("OpenGL function pointer of `glGetnColorTable` is NULL");
	}
	fn glGetnConvolutionFilter(&self, _: GLenum, _: GLenum, _: GLenum, _: GLsizei, _: *mut c_void) {
		panic!("OpenGL function pointer of `glGetnConvolutionFilter` is NULL");
	}
	fn glGetnSeparableFilter(&self, _: GLenum, _: GLenum, _: GLenum, _: GLsizei, _: *mut c_void, _: GLsizei, _: *mut c_void, _: *mut c_void) {
		panic!("OpenGL function pointer of `glGetnSeparableFilter` is NULL");
	}
	fn glGetnHistogram(&self, _: GLenum, _: GLboolean, _: GLenum, _: GLenum, _: GLsizei, _: *mut c_void) {
		panic!("OpenGL function pointer of `glGetnHistogram` is NULL");
	}
	fn glGetnMinmax(&self, _: GLenum, _: GLboolean, _: GLenum, _: GLenum, _: GLsizei, _: *mut c_void) {
		panic!("OpenGL function pointer of `glGetnMinmax` is NULL");
	}
	fn glTextureBarrier(&self) {
		panic!("OpenGL function pointer of `glTextureBarrier` is NULL");
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Version45 {
	available: bool,
	clipcontrol: PFNGLCLIPCONTROLPROC,
	createtransformfeedbacks: PFNGLCREATETRANSFORMFEEDBACKSPROC,
	transformfeedbackbufferbase: PFNGLTRANSFORMFEEDBACKBUFFERBASEPROC,
	transformfeedbackbufferrange: PFNGLTRANSFORMFEEDBACKBUFFERRANGEPROC,
	gettransformfeedbackiv: PFNGLGETTRANSFORMFEEDBACKIVPROC,
	gettransformfeedbacki_v: PFNGLGETTRANSFORMFEEDBACKI_VPROC,
	gettransformfeedbacki64_v: PFNGLGETTRANSFORMFEEDBACKI64_VPROC,
	createbuffers: PFNGLCREATEBUFFERSPROC,
	namedbufferstorage: PFNGLNAMEDBUFFERSTORAGEPROC,
	namedbufferdata: PFNGLNAMEDBUFFERDATAPROC,
	namedbuffersubdata: PFNGLNAMEDBUFFERSUBDATAPROC,
	copynamedbuffersubdata: PFNGLCOPYNAMEDBUFFERSUBDATAPROC,
	clearnamedbufferdata: PFNGLCLEARNAMEDBUFFERDATAPROC,
	clearnamedbuffersubdata: PFNGLCLEARNAMEDBUFFERSUBDATAPROC,
	mapnamedbuffer: PFNGLMAPNAMEDBUFFERPROC,
	mapnamedbufferrange: PFNGLMAPNAMEDBUFFERRANGEPROC,
	unmapnamedbuffer: PFNGLUNMAPNAMEDBUFFERPROC,
	flushmappednamedbufferrange: PFNGLFLUSHMAPPEDNAMEDBUFFERRANGEPROC,
	getnamedbufferparameteriv: PFNGLGETNAMEDBUFFERPARAMETERIVPROC,
	getnamedbufferparameteri64v: PFNGLGETNAMEDBUFFERPARAMETERI64VPROC,
	getnamedbufferpointerv: PFNGLGETNAMEDBUFFERPOINTERVPROC,
	getnamedbuffersubdata: PFNGLGETNAMEDBUFFERSUBDATAPROC,
	createframebuffers: PFNGLCREATEFRAMEBUFFERSPROC,
	namedframebufferrenderbuffer: PFNGLNAMEDFRAMEBUFFERRENDERBUFFERPROC,
	namedframebufferparameteri: PFNGLNAMEDFRAMEBUFFERPARAMETERIPROC,
	namedframebuffertexture: PFNGLNAMEDFRAMEBUFFERTEXTUREPROC,
	namedframebuffertexturelayer: PFNGLNAMEDFRAMEBUFFERTEXTURELAYERPROC,
	namedframebufferdrawbuffer: PFNGLNAMEDFRAMEBUFFERDRAWBUFFERPROC,
	namedframebufferdrawbuffers: PFNGLNAMEDFRAMEBUFFERDRAWBUFFERSPROC,
	namedframebufferreadbuffer: PFNGLNAMEDFRAMEBUFFERREADBUFFERPROC,
	invalidatenamedframebufferdata: PFNGLINVALIDATENAMEDFRAMEBUFFERDATAPROC,
	invalidatenamedframebuffersubdata: PFNGLINVALIDATENAMEDFRAMEBUFFERSUBDATAPROC,
	clearnamedframebufferiv: PFNGLCLEARNAMEDFRAMEBUFFERIVPROC,
	clearnamedframebufferuiv: PFNGLCLEARNAMEDFRAMEBUFFERUIVPROC,
	clearnamedframebufferfv: PFNGLCLEARNAMEDFRAMEBUFFERFVPROC,
	clearnamedframebufferfi: PFNGLCLEARNAMEDFRAMEBUFFERFIPROC,
	blitnamedframebuffer: PFNGLBLITNAMEDFRAMEBUFFERPROC,
	checknamedframebufferstatus: PFNGLCHECKNAMEDFRAMEBUFFERSTATUSPROC,
	getnamedframebufferparameteriv: PFNGLGETNAMEDFRAMEBUFFERPARAMETERIVPROC,
	getnamedframebufferattachmentparameteriv: PFNGLGETNAMEDFRAMEBUFFERATTACHMENTPARAMETERIVPROC,
	createrenderbuffers: PFNGLCREATERENDERBUFFERSPROC,
	namedrenderbufferstorage: PFNGLNAMEDRENDERBUFFERSTORAGEPROC,
	namedrenderbufferstoragemultisample: PFNGLNAMEDRENDERBUFFERSTORAGEMULTISAMPLEPROC,
	getnamedrenderbufferparameteriv: PFNGLGETNAMEDRENDERBUFFERPARAMETERIVPROC,
	createtextures: PFNGLCREATETEXTURESPROC,
	texturebuffer: PFNGLTEXTUREBUFFERPROC,
	texturebufferrange: PFNGLTEXTUREBUFFERRANGEPROC,
	texturestorage1d: PFNGLTEXTURESTORAGE1DPROC,
	texturestorage2d: PFNGLTEXTURESTORAGE2DPROC,
	texturestorage3d: PFNGLTEXTURESTORAGE3DPROC,
	texturestorage2dmultisample: PFNGLTEXTURESTORAGE2DMULTISAMPLEPROC,
	texturestorage3dmultisample: PFNGLTEXTURESTORAGE3DMULTISAMPLEPROC,
	texturesubimage1d: PFNGLTEXTURESUBIMAGE1DPROC,
	texturesubimage2d: PFNGLTEXTURESUBIMAGE2DPROC,
	texturesubimage3d: PFNGLTEXTURESUBIMAGE3DPROC,
	compressedtexturesubimage1d: PFNGLCOMPRESSEDTEXTURESUBIMAGE1DPROC,
	compressedtexturesubimage2d: PFNGLCOMPRESSEDTEXTURESUBIMAGE2DPROC,
	compressedtexturesubimage3d: PFNGLCOMPRESSEDTEXTURESUBIMAGE3DPROC,
	copytexturesubimage1d: PFNGLCOPYTEXTURESUBIMAGE1DPROC,
	copytexturesubimage2d: PFNGLCOPYTEXTURESUBIMAGE2DPROC,
	copytexturesubimage3d: PFNGLCOPYTEXTURESUBIMAGE3DPROC,
	textureparameterf: PFNGLTEXTUREPARAMETERFPROC,
	textureparameterfv: PFNGLTEXTUREPARAMETERFVPROC,
	textureparameteri: PFNGLTEXTUREPARAMETERIPROC,
	textureparameteriiv: PFNGLTEXTUREPARAMETERIIVPROC,
	textureparameteriuiv: PFNGLTEXTUREPARAMETERIUIVPROC,
	textureparameteriv: PFNGLTEXTUREPARAMETERIVPROC,
	generatetexturemipmap: PFNGLGENERATETEXTUREMIPMAPPROC,
	bindtextureunit: PFNGLBINDTEXTUREUNITPROC,
	gettextureimage: PFNGLGETTEXTUREIMAGEPROC,
	getcompressedtextureimage: PFNGLGETCOMPRESSEDTEXTUREIMAGEPROC,
	gettexturelevelparameterfv: PFNGLGETTEXTURELEVELPARAMETERFVPROC,
	gettexturelevelparameteriv: PFNGLGETTEXTURELEVELPARAMETERIVPROC,
	gettextureparameterfv: PFNGLGETTEXTUREPARAMETERFVPROC,
	gettextureparameteriiv: PFNGLGETTEXTUREPARAMETERIIVPROC,
	gettextureparameteriuiv: PFNGLGETTEXTUREPARAMETERIUIVPROC,
	gettextureparameteriv: PFNGLGETTEXTUREPARAMETERIVPROC,
	createvertexarrays: PFNGLCREATEVERTEXARRAYSPROC,
	disablevertexarrayattrib: PFNGLDISABLEVERTEXARRAYATTRIBPROC,
	enablevertexarrayattrib: PFNGLENABLEVERTEXARRAYATTRIBPROC,
	vertexarrayelementbuffer: PFNGLVERTEXARRAYELEMENTBUFFERPROC,
	vertexarrayvertexbuffer: PFNGLVERTEXARRAYVERTEXBUFFERPROC,
	vertexarrayvertexbuffers: PFNGLVERTEXARRAYVERTEXBUFFERSPROC,
	vertexarrayattribbinding: PFNGLVERTEXARRAYATTRIBBINDINGPROC,
	vertexarrayattribformat: PFNGLVERTEXARRAYATTRIBFORMATPROC,
	vertexarrayattribiformat: PFNGLVERTEXARRAYATTRIBIFORMATPROC,
	vertexarrayattriblformat: PFNGLVERTEXARRAYATTRIBLFORMATPROC,
	vertexarraybindingdivisor: PFNGLVERTEXARRAYBINDINGDIVISORPROC,
	getvertexarrayiv: PFNGLGETVERTEXARRAYIVPROC,
	getvertexarrayindexediv: PFNGLGETVERTEXARRAYINDEXEDIVPROC,
	getvertexarrayindexed64iv: PFNGLGETVERTEXARRAYINDEXED64IVPROC,
	createsamplers: PFNGLCREATESAMPLERSPROC,
	createprogrampipelines: PFNGLCREATEPROGRAMPIPELINESPROC,
	createqueries: PFNGLCREATEQUERIESPROC,
	getquerybufferobjecti64v: PFNGLGETQUERYBUFFEROBJECTI64VPROC,
	getquerybufferobjectiv: PFNGLGETQUERYBUFFEROBJECTIVPROC,
	getquerybufferobjectui64v: PFNGLGETQUERYBUFFEROBJECTUI64VPROC,
	getquerybufferobjectuiv: PFNGLGETQUERYBUFFEROBJECTUIVPROC,
	memorybarrierbyregion: PFNGLMEMORYBARRIERBYREGIONPROC,
	gettexturesubimage: PFNGLGETTEXTURESUBIMAGEPROC,
	getcompressedtexturesubimage: PFNGLGETCOMPRESSEDTEXTURESUBIMAGEPROC,
	getgraphicsresetstatus: PFNGLGETGRAPHICSRESETSTATUSPROC,
	getncompressedteximage: PFNGLGETNCOMPRESSEDTEXIMAGEPROC,
	getnteximage: PFNGLGETNTEXIMAGEPROC,
	getnuniformdv: PFNGLGETNUNIFORMDVPROC,
	getnuniformfv: PFNGLGETNUNIFORMFVPROC,
	getnuniformiv: PFNGLGETNUNIFORMIVPROC,
	getnuniformuiv: PFNGLGETNUNIFORMUIVPROC,
	readnpixels: PFNGLREADNPIXELSPROC,
	getnmapdv: PFNGLGETNMAPDVPROC,
	getnmapfv: PFNGLGETNMAPFVPROC,
	getnmapiv: PFNGLGETNMAPIVPROC,
	getnpixelmapfv: PFNGLGETNPIXELMAPFVPROC,
	getnpixelmapuiv: PFNGLGETNPIXELMAPUIVPROC,
	getnpixelmapusv: PFNGLGETNPIXELMAPUSVPROC,
	getnpolygonstipple: PFNGLGETNPOLYGONSTIPPLEPROC,
	getncolortable: PFNGLGETNCOLORTABLEPROC,
	getnconvolutionfilter: PFNGLGETNCONVOLUTIONFILTERPROC,
	getnseparablefilter: PFNGLGETNSEPARABLEFILTERPROC,
	getnhistogram: PFNGLGETNHISTOGRAMPROC,
	getnminmax: PFNGLGETNMINMAXPROC,
	texturebarrier: PFNGLTEXTUREBARRIERPROC,
}

impl GL_4_5 for Version45 {
	#[inline(always)]
	fn glClipControl(&self, origin: GLenum, depth: GLenum) {
		(self.clipcontrol)(origin, depth)
	}
	#[inline(always)]
	fn glCreateTransformFeedbacks(&self, n: GLsizei, ids: *mut GLuint) {
		(self.createtransformfeedbacks)(n, ids)
	}
	#[inline(always)]
	fn glTransformFeedbackBufferBase(&self, xfb: GLuint, index: GLuint, buffer: GLuint) {
		(self.transformfeedbackbufferbase)(xfb, index, buffer)
	}
	#[inline(always)]
	fn glTransformFeedbackBufferRange(&self, xfb: GLuint, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) {
		(self.transformfeedbackbufferrange)(xfb, index, buffer, offset, size)
	}
	#[inline(always)]
	fn glGetTransformFeedbackiv(&self, xfb: GLuint, pname: GLenum, param: *mut GLint) {
		(self.gettransformfeedbackiv)(xfb, pname, param)
	}
	#[inline(always)]
	fn glGetTransformFeedbacki_v(&self, xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint) {
		(self.gettransformfeedbacki_v)(xfb, pname, index, param)
	}
	#[inline(always)]
	fn glGetTransformFeedbacki64_v(&self, xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint64) {
		(self.gettransformfeedbacki64_v)(xfb, pname, index, param)
	}
	#[inline(always)]
	fn glCreateBuffers(&self, n: GLsizei, buffers: *mut GLuint) {
		(self.createbuffers)(n, buffers)
	}
	#[inline(always)]
	fn glNamedBufferStorage(&self, buffer: GLuint, size: GLsizeiptr, data: *const c_void, flags: GLbitfield) {
		(self.namedbufferstorage)(buffer, size, data, flags)
	}
	#[inline(always)]
	fn glNamedBufferData(&self, buffer: GLuint, size: GLsizeiptr, data: *const c_void, usage: GLenum) {
		(self.namedbufferdata)(buffer, size, data, usage)
	}
	#[inline(always)]
	fn glNamedBufferSubData(&self, buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const c_void) {
		(self.namedbuffersubdata)(buffer, offset, size, data)
	}
	#[inline(always)]
	fn glCopyNamedBufferSubData(&self, readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr) {
		(self.copynamedbuffersubdata)(readBuffer, writeBuffer, readOffset, writeOffset, size)
	}
	#[inline(always)]
	fn glClearNamedBufferData(&self, buffer: GLuint, internalformat: GLenum, format: GLenum, type_: GLenum, data: *const c_void) {
		(self.clearnamedbufferdata)(buffer, internalformat, format, type_, data)
	}
	#[inline(always)]
	fn glClearNamedBufferSubData(&self, buffer: GLuint, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *const c_void) {
		(self.clearnamedbuffersubdata)(buffer, internalformat, offset, size, format, type_, data)
	}
	#[inline(always)]
	fn glMapNamedBuffer(&self, buffer: GLuint, access: GLenum) -> *mut c_void {
		(self.mapnamedbuffer)(buffer, access)
	}
	#[inline(always)]
	fn glMapNamedBufferRange(&self, buffer: GLuint, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut c_void {
		(self.mapnamedbufferrange)(buffer, offset, length, access)
	}
	#[inline(always)]
	fn glUnmapNamedBuffer(&self, buffer: GLuint) -> GLboolean {
		(self.unmapnamedbuffer)(buffer)
	}
	#[inline(always)]
	fn glFlushMappedNamedBufferRange(&self, buffer: GLuint, offset: GLintptr, length: GLsizeiptr) {
		(self.flushmappednamedbufferrange)(buffer, offset, length)
	}
	#[inline(always)]
	fn glGetNamedBufferParameteriv(&self, buffer: GLuint, pname: GLenum, params: *mut GLint) {
		(self.getnamedbufferparameteriv)(buffer, pname, params)
	}
	#[inline(always)]
	fn glGetNamedBufferParameteri64v(&self, buffer: GLuint, pname: GLenum, params: *mut GLint64) {
		(self.getnamedbufferparameteri64v)(buffer, pname, params)
	}
	#[inline(always)]
	fn glGetNamedBufferPointerv(&self, buffer: GLuint, pname: GLenum, params: *mut *mut c_void) {
		(self.getnamedbufferpointerv)(buffer, pname, params)
	}
	#[inline(always)]
	fn glGetNamedBufferSubData(&self, buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *mut c_void) {
		(self.getnamedbuffersubdata)(buffer, offset, size, data)
	}
	#[inline(always)]
	fn glCreateFramebuffers(&self, n: GLsizei, framebuffers: *mut GLuint) {
		(self.createframebuffers)(n, framebuffers)
	}
	#[inline(always)]
	fn glNamedFramebufferRenderbuffer(&self, framebuffer: GLuint, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint) {
		(self.namedframebufferrenderbuffer)(framebuffer, attachment, renderbuffertarget, renderbuffer)
	}
	#[inline(always)]
	fn glNamedFramebufferParameteri(&self, framebuffer: GLuint, pname: GLenum, param: GLint) {
		(self.namedframebufferparameteri)(framebuffer, pname, param)
	}
	#[inline(always)]
	fn glNamedFramebufferTexture(&self, framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint) {
		(self.namedframebuffertexture)(framebuffer, attachment, texture, level)
	}
	#[inline(always)]
	fn glNamedFramebufferTextureLayer(&self, framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint) {
		(self.namedframebuffertexturelayer)(framebuffer, attachment, texture, level, layer)
	}
	#[inline(always)]
	fn glNamedFramebufferDrawBuffer(&self, framebuffer: GLuint, buf: GLenum) {
		(self.namedframebufferdrawbuffer)(framebuffer, buf)
	}
	#[inline(always)]
	fn glNamedFramebufferDrawBuffers(&self, framebuffer: GLuint, n: GLsizei, bufs: *const GLenum) {
		(self.namedframebufferdrawbuffers)(framebuffer, n, bufs)
	}
	#[inline(always)]
	fn glNamedFramebufferReadBuffer(&self, framebuffer: GLuint, src: GLenum) {
		(self.namedframebufferreadbuffer)(framebuffer, src)
	}
	#[inline(always)]
	fn glInvalidateNamedFramebufferData(&self, framebuffer: GLuint, numAttachments: GLsizei, attachments: *const GLenum) {
		(self.invalidatenamedframebufferdata)(framebuffer, numAttachments, attachments)
	}
	#[inline(always)]
	fn glInvalidateNamedFramebufferSubData(&self, framebuffer: GLuint, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
		(self.invalidatenamedframebuffersubdata)(framebuffer, numAttachments, attachments, x, y, width, height)
	}
	#[inline(always)]
	fn glClearNamedFramebufferiv(&self, framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLint) {
		(self.clearnamedframebufferiv)(framebuffer, buffer, drawbuffer, value)
	}
	#[inline(always)]
	fn glClearNamedFramebufferuiv(&self, framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLuint) {
		(self.clearnamedframebufferuiv)(framebuffer, buffer, drawbuffer, value)
	}
	#[inline(always)]
	fn glClearNamedFramebufferfv(&self, framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLfloat) {
		(self.clearnamedframebufferfv)(framebuffer, buffer, drawbuffer, value)
	}
	#[inline(always)]
	fn glClearNamedFramebufferfi(&self, framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint) {
		(self.clearnamedframebufferfi)(framebuffer, buffer, drawbuffer, depth, stencil)
	}
	#[inline(always)]
	fn glBlitNamedFramebuffer(&self, readFramebuffer: GLuint, drawFramebuffer: GLuint, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum) {
		(self.blitnamedframebuffer)(readFramebuffer, drawFramebuffer, srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter)
	}
	#[inline(always)]
	fn glCheckNamedFramebufferStatus(&self, framebuffer: GLuint, target: GLenum) -> GLenum {
		(self.checknamedframebufferstatus)(framebuffer, target)
	}
	#[inline(always)]
	fn glGetNamedFramebufferParameteriv(&self, framebuffer: GLuint, pname: GLenum, param: *mut GLint) {
		(self.getnamedframebufferparameteriv)(framebuffer, pname, param)
	}
	#[inline(always)]
	fn glGetNamedFramebufferAttachmentParameteriv(&self, framebuffer: GLuint, attachment: GLenum, pname: GLenum, params: *mut GLint) {
		(self.getnamedframebufferattachmentparameteriv)(framebuffer, attachment, pname, params)
	}
	#[inline(always)]
	fn glCreateRenderbuffers(&self, n: GLsizei, renderbuffers: *mut GLuint) {
		(self.createrenderbuffers)(n, renderbuffers)
	}
	#[inline(always)]
	fn glNamedRenderbufferStorage(&self, renderbuffer: GLuint, internalformat: GLenum, width: GLsizei, height: GLsizei) {
		(self.namedrenderbufferstorage)(renderbuffer, internalformat, width, height)
	}
	#[inline(always)]
	fn glNamedRenderbufferStorageMultisample(&self, renderbuffer: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) {
		(self.namedrenderbufferstoragemultisample)(renderbuffer, samples, internalformat, width, height)
	}
	#[inline(always)]
	fn glGetNamedRenderbufferParameteriv(&self, renderbuffer: GLuint, pname: GLenum, params: *mut GLint) {
		(self.getnamedrenderbufferparameteriv)(renderbuffer, pname, params)
	}
	#[inline(always)]
	fn glCreateTextures(&self, target: GLenum, n: GLsizei, textures: *mut GLuint) {
		(self.createtextures)(target, n, textures)
	}
	#[inline(always)]
	fn glTextureBuffer(&self, texture: GLuint, internalformat: GLenum, buffer: GLuint) {
		(self.texturebuffer)(texture, internalformat, buffer)
	}
	#[inline(always)]
	fn glTextureBufferRange(&self, texture: GLuint, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) {
		(self.texturebufferrange)(texture, internalformat, buffer, offset, size)
	}
	#[inline(always)]
	fn glTextureStorage1D(&self, texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei) {
		(self.texturestorage1d)(texture, levels, internalformat, width)
	}
	#[inline(always)]
	fn glTextureStorage2D(&self, texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) {
		(self.texturestorage2d)(texture, levels, internalformat, width, height)
	}
	#[inline(always)]
	fn glTextureStorage3D(&self, texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei) {
		(self.texturestorage3d)(texture, levels, internalformat, width, height, depth)
	}
	#[inline(always)]
	fn glTextureStorage2DMultisample(&self, texture: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) {
		(self.texturestorage2dmultisample)(texture, samples, internalformat, width, height, fixedsamplelocations)
	}
	#[inline(always)]
	fn glTextureStorage3DMultisample(&self, texture: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) {
		(self.texturestorage3dmultisample)(texture, samples, internalformat, width, height, depth, fixedsamplelocations)
	}
	#[inline(always)]
	fn glTextureSubImage1D(&self, texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) {
		(self.texturesubimage1d)(texture, level, xoffset, width, format, type_, pixels)
	}
	#[inline(always)]
	fn glTextureSubImage2D(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) {
		(self.texturesubimage2d)(texture, level, xoffset, yoffset, width, height, format, type_, pixels)
	}
	#[inline(always)]
	fn glTextureSubImage3D(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) {
		(self.texturesubimage3d)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels)
	}
	#[inline(always)]
	fn glCompressedTextureSubImage1D(&self, texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) {
		(self.compressedtexturesubimage1d)(texture, level, xoffset, width, format, imageSize, data)
	}
	#[inline(always)]
	fn glCompressedTextureSubImage2D(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) {
		(self.compressedtexturesubimage2d)(texture, level, xoffset, yoffset, width, height, format, imageSize, data)
	}
	#[inline(always)]
	fn glCompressedTextureSubImage3D(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) {
		(self.compressedtexturesubimage3d)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data)
	}
	#[inline(always)]
	fn glCopyTextureSubImage1D(&self, texture: GLuint, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) {
		(self.copytexturesubimage1d)(texture, level, xoffset, x, y, width)
	}
	#[inline(always)]
	fn glCopyTextureSubImage2D(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
		(self.copytexturesubimage2d)(texture, level, xoffset, yoffset, x, y, width, height)
	}
	#[inline(always)]
	fn glCopyTextureSubImage3D(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
		(self.copytexturesubimage3d)(texture, level, xoffset, yoffset, zoffset, x, y, width, height)
	}
	#[inline(always)]
	fn glTextureParameterf(&self, texture: GLuint, pname: GLenum, param: GLfloat) {
		(self.textureparameterf)(texture, pname, param)
	}
	#[inline(always)]
	fn glTextureParameterfv(&self, texture: GLuint, pname: GLenum, param: *const GLfloat) {
		(self.textureparameterfv)(texture, pname, param)
	}
	#[inline(always)]
	fn glTextureParameteri(&self, texture: GLuint, pname: GLenum, param: GLint) {
		(self.textureparameteri)(texture, pname, param)
	}
	#[inline(always)]
	fn glTextureParameterIiv(&self, texture: GLuint, pname: GLenum, params: *const GLint) {
		(self.textureparameteriiv)(texture, pname, params)
	}
	#[inline(always)]
	fn glTextureParameterIuiv(&self, texture: GLuint, pname: GLenum, params: *const GLuint) {
		(self.textureparameteriuiv)(texture, pname, params)
	}
	#[inline(always)]
	fn glTextureParameteriv(&self, texture: GLuint, pname: GLenum, param: *const GLint) {
		(self.textureparameteriv)(texture, pname, param)
	}
	#[inline(always)]
	fn glGenerateTextureMipmap(&self, texture: GLuint) {
		(self.generatetexturemipmap)(texture)
	}
	#[inline(always)]
	fn glBindTextureUnit(&self, unit: GLuint, texture: GLuint) {
		(self.bindtextureunit)(unit, texture)
	}
	#[inline(always)]
	fn glGetTextureImage(&self, texture: GLuint, level: GLint, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut c_void) {
		(self.gettextureimage)(texture, level, format, type_, bufSize, pixels)
	}
	#[inline(always)]
	fn glGetCompressedTextureImage(&self, texture: GLuint, level: GLint, bufSize: GLsizei, pixels: *mut c_void) {
		(self.getcompressedtextureimage)(texture, level, bufSize, pixels)
	}
	#[inline(always)]
	fn glGetTextureLevelParameterfv(&self, texture: GLuint, level: GLint, pname: GLenum, params: *mut GLfloat) {
		(self.gettexturelevelparameterfv)(texture, level, pname, params)
	}
	#[inline(always)]
	fn glGetTextureLevelParameteriv(&self, texture: GLuint, level: GLint, pname: GLenum, params: *mut GLint) {
		(self.gettexturelevelparameteriv)(texture, level, pname, params)
	}
	#[inline(always)]
	fn glGetTextureParameterfv(&self, texture: GLuint, pname: GLenum, params: *mut GLfloat) {
		(self.gettextureparameterfv)(texture, pname, params)
	}
	#[inline(always)]
	fn glGetTextureParameterIiv(&self, texture: GLuint, pname: GLenum, params: *mut GLint) {
		(self.gettextureparameteriiv)(texture, pname, params)
	}
	#[inline(always)]
	fn glGetTextureParameterIuiv(&self, texture: GLuint, pname: GLenum, params: *mut GLuint) {
		(self.gettextureparameteriuiv)(texture, pname, params)
	}
	#[inline(always)]
	fn glGetTextureParameteriv(&self, texture: GLuint, pname: GLenum, params: *mut GLint) {
		(self.gettextureparameteriv)(texture, pname, params)
	}
	#[inline(always)]
	fn glCreateVertexArrays(&self, n: GLsizei, arrays: *mut GLuint) {
		(self.createvertexarrays)(n, arrays)
	}
	#[inline(always)]
	fn glDisableVertexArrayAttrib(&self, vaobj: GLuint, index: GLuint) {
		(self.disablevertexarrayattrib)(vaobj, index)
	}
	#[inline(always)]
	fn glEnableVertexArrayAttrib(&self, vaobj: GLuint, index: GLuint) {
		(self.enablevertexarrayattrib)(vaobj, index)
	}
	#[inline(always)]
	fn glVertexArrayElementBuffer(&self, vaobj: GLuint, buffer: GLuint) {
		(self.vertexarrayelementbuffer)(vaobj, buffer)
	}
	#[inline(always)]
	fn glVertexArrayVertexBuffer(&self, vaobj: GLuint, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei) {
		(self.vertexarrayvertexbuffer)(vaobj, bindingindex, buffer, offset, stride)
	}
	#[inline(always)]
	fn glVertexArrayVertexBuffers(&self, vaobj: GLuint, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei) {
		(self.vertexarrayvertexbuffers)(vaobj, first, count, buffers, offsets, strides)
	}
	#[inline(always)]
	fn glVertexArrayAttribBinding(&self, vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint) {
		(self.vertexarrayattribbinding)(vaobj, attribindex, bindingindex)
	}
	#[inline(always)]
	fn glVertexArrayAttribFormat(&self, vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint) {
		(self.vertexarrayattribformat)(vaobj, attribindex, size, type_, normalized, relativeoffset)
	}
	#[inline(always)]
	fn glVertexArrayAttribIFormat(&self, vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) {
		(self.vertexarrayattribiformat)(vaobj, attribindex, size, type_, relativeoffset)
	}
	#[inline(always)]
	fn glVertexArrayAttribLFormat(&self, vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) {
		(self.vertexarrayattriblformat)(vaobj, attribindex, size, type_, relativeoffset)
	}
	#[inline(always)]
	fn glVertexArrayBindingDivisor(&self, vaobj: GLuint, bindingindex: GLuint, divisor: GLuint) {
		(self.vertexarraybindingdivisor)(vaobj, bindingindex, divisor)
	}
	#[inline(always)]
	fn glGetVertexArrayiv(&self, vaobj: GLuint, pname: GLenum, param: *mut GLint) {
		(self.getvertexarrayiv)(vaobj, pname, param)
	}
	#[inline(always)]
	fn glGetVertexArrayIndexediv(&self, vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint) {
		(self.getvertexarrayindexediv)(vaobj, index, pname, param)
	}
	#[inline(always)]
	fn glGetVertexArrayIndexed64iv(&self, vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint64) {
		(self.getvertexarrayindexed64iv)(vaobj, index, pname, param)
	}
	#[inline(always)]
	fn glCreateSamplers(&self, n: GLsizei, samplers: *mut GLuint) {
		(self.createsamplers)(n, samplers)
	}
	#[inline(always)]
	fn glCreateProgramPipelines(&self, n: GLsizei, pipelines: *mut GLuint) {
		(self.createprogrampipelines)(n, pipelines)
	}
	#[inline(always)]
	fn glCreateQueries(&self, target: GLenum, n: GLsizei, ids: *mut GLuint) {
		(self.createqueries)(target, n, ids)
	}
	#[inline(always)]
	fn glGetQueryBufferObjecti64v(&self, id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr) {
		(self.getquerybufferobjecti64v)(id, buffer, pname, offset)
	}
	#[inline(always)]
	fn glGetQueryBufferObjectiv(&self, id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr) {
		(self.getquerybufferobjectiv)(id, buffer, pname, offset)
	}
	#[inline(always)]
	fn glGetQueryBufferObjectui64v(&self, id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr) {
		(self.getquerybufferobjectui64v)(id, buffer, pname, offset)
	}
	#[inline(always)]
	fn glGetQueryBufferObjectuiv(&self, id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr) {
		(self.getquerybufferobjectuiv)(id, buffer, pname, offset)
	}
	#[inline(always)]
	fn glMemoryBarrierByRegion(&self, barriers: GLbitfield) {
		(self.memorybarrierbyregion)(barriers)
	}
	#[inline(always)]
	fn glGetTextureSubImage(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut c_void) {
		(self.gettexturesubimage)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, bufSize, pixels)
	}
	#[inline(always)]
	fn glGetCompressedTextureSubImage(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, bufSize: GLsizei, pixels: *mut c_void) {
		(self.getcompressedtexturesubimage)(texture, level, xoffset, yoffset, zoffset, width, height, depth, bufSize, pixels)
	}
	#[inline(always)]
	fn glGetGraphicsResetStatus(&self) -> GLenum {
		(self.getgraphicsresetstatus)()
	}
	#[inline(always)]
	fn glGetnCompressedTexImage(&self, target: GLenum, lod: GLint, bufSize: GLsizei, pixels: *mut c_void) {
		(self.getncompressedteximage)(target, lod, bufSize, pixels)
	}
	#[inline(always)]
	fn glGetnTexImage(&self, target: GLenum, level: GLint, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut c_void) {
		(self.getnteximage)(target, level, format, type_, bufSize, pixels)
	}
	#[inline(always)]
	fn glGetnUniformdv(&self, program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLdouble) {
		(self.getnuniformdv)(program, location, bufSize, params)
	}
	#[inline(always)]
	fn glGetnUniformfv(&self, program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLfloat) {
		(self.getnuniformfv)(program, location, bufSize, params)
	}
	#[inline(always)]
	fn glGetnUniformiv(&self, program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint) {
		(self.getnuniformiv)(program, location, bufSize, params)
	}
	#[inline(always)]
	fn glGetnUniformuiv(&self, program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint) {
		(self.getnuniformuiv)(program, location, bufSize, params)
	}
	#[inline(always)]
	fn glReadnPixels(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, bufSize: GLsizei, data: *mut c_void) {
		(self.readnpixels)(x, y, width, height, format, type_, bufSize, data)
	}
	#[inline(always)]
	fn glGetnMapdv(&self, target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLdouble) {
		(self.getnmapdv)(target, query, bufSize, v)
	}
	#[inline(always)]
	fn glGetnMapfv(&self, target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLfloat) {
		(self.getnmapfv)(target, query, bufSize, v)
	}
	#[inline(always)]
	fn glGetnMapiv(&self, target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLint) {
		(self.getnmapiv)(target, query, bufSize, v)
	}
	#[inline(always)]
	fn glGetnPixelMapfv(&self, map: GLenum, bufSize: GLsizei, values: *mut GLfloat) {
		(self.getnpixelmapfv)(map, bufSize, values)
	}
	#[inline(always)]
	fn glGetnPixelMapuiv(&self, map: GLenum, bufSize: GLsizei, values: *mut GLuint) {
		(self.getnpixelmapuiv)(map, bufSize, values)
	}
	#[inline(always)]
	fn glGetnPixelMapusv(&self, map: GLenum, bufSize: GLsizei, values: *mut GLushort) {
		(self.getnpixelmapusv)(map, bufSize, values)
	}
	#[inline(always)]
	fn glGetnPolygonStipple(&self, bufSize: GLsizei, pattern: *mut GLubyte) {
		(self.getnpolygonstipple)(bufSize, pattern)
	}
	#[inline(always)]
	fn glGetnColorTable(&self, target: GLenum, format: GLenum, type_: GLenum, bufSize: GLsizei, table: *mut c_void) {
		(self.getncolortable)(target, format, type_, bufSize, table)
	}
	#[inline(always)]
	fn glGetnConvolutionFilter(&self, target: GLenum, format: GLenum, type_: GLenum, bufSize: GLsizei, image: *mut c_void) {
		(self.getnconvolutionfilter)(target, format, type_, bufSize, image)
	}
	#[inline(always)]
	fn glGetnSeparableFilter(&self, target: GLenum, format: GLenum, type_: GLenum, rowBufSize: GLsizei, row: *mut c_void, columnBufSize: GLsizei, column: *mut c_void, span: *mut c_void) {
		(self.getnseparablefilter)(target, format, type_, rowBufSize, row, columnBufSize, column, span)
	}
	#[inline(always)]
	fn glGetnHistogram(&self, target: GLenum, reset: GLboolean, format: GLenum, type_: GLenum, bufSize: GLsizei, values: *mut c_void) {
		(self.getnhistogram)(target, reset, format, type_, bufSize, values)
	}
	#[inline(always)]
	fn glGetnMinmax(&self, target: GLenum, reset: GLboolean, format: GLenum, type_: GLenum, bufSize: GLsizei, values: *mut c_void) {
		(self.getnminmax)(target, reset, format, type_, bufSize, values)
	}
	#[inline(always)]
	fn glTextureBarrier(&self) {
		(self.texturebarrier)()
	}
}

impl Version45 {
	pub fn new(base: impl GL_1_0, get_proc_address: &impl Fn(&'static str) -> *const c_void) -> Self {
		let (spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 40500 {
			return Self::default();
		}
		Self {
			available: true,
			clipcontrol: unsafe{transmute(get_proc_address("glClipControl"))},
			createtransformfeedbacks: unsafe{transmute(get_proc_address("glCreateTransformFeedbacks"))},
			transformfeedbackbufferbase: unsafe{transmute(get_proc_address("glTransformFeedbackBufferBase"))},
			transformfeedbackbufferrange: unsafe{transmute(get_proc_address("glTransformFeedbackBufferRange"))},
			gettransformfeedbackiv: unsafe{transmute(get_proc_address("glGetTransformFeedbackiv"))},
			gettransformfeedbacki_v: unsafe{transmute(get_proc_address("glGetTransformFeedbacki_v"))},
			gettransformfeedbacki64_v: unsafe{transmute(get_proc_address("glGetTransformFeedbacki64_v"))},
			createbuffers: unsafe{transmute(get_proc_address("glCreateBuffers"))},
			namedbufferstorage: unsafe{transmute(get_proc_address("glNamedBufferStorage"))},
			namedbufferdata: unsafe{transmute(get_proc_address("glNamedBufferData"))},
			namedbuffersubdata: unsafe{transmute(get_proc_address("glNamedBufferSubData"))},
			copynamedbuffersubdata: unsafe{transmute(get_proc_address("glCopyNamedBufferSubData"))},
			clearnamedbufferdata: unsafe{transmute(get_proc_address("glClearNamedBufferData"))},
			clearnamedbuffersubdata: unsafe{transmute(get_proc_address("glClearNamedBufferSubData"))},
			mapnamedbuffer: unsafe{transmute(get_proc_address("glMapNamedBuffer"))},
			mapnamedbufferrange: unsafe{transmute(get_proc_address("glMapNamedBufferRange"))},
			unmapnamedbuffer: unsafe{transmute(get_proc_address("glUnmapNamedBuffer"))},
			flushmappednamedbufferrange: unsafe{transmute(get_proc_address("glFlushMappedNamedBufferRange"))},
			getnamedbufferparameteriv: unsafe{transmute(get_proc_address("glGetNamedBufferParameteriv"))},
			getnamedbufferparameteri64v: unsafe{transmute(get_proc_address("glGetNamedBufferParameteri64v"))},
			getnamedbufferpointerv: unsafe{transmute(get_proc_address("glGetNamedBufferPointerv"))},
			getnamedbuffersubdata: unsafe{transmute(get_proc_address("glGetNamedBufferSubData"))},
			createframebuffers: unsafe{transmute(get_proc_address("glCreateFramebuffers"))},
			namedframebufferrenderbuffer: unsafe{transmute(get_proc_address("glNamedFramebufferRenderbuffer"))},
			namedframebufferparameteri: unsafe{transmute(get_proc_address("glNamedFramebufferParameteri"))},
			namedframebuffertexture: unsafe{transmute(get_proc_address("glNamedFramebufferTexture"))},
			namedframebuffertexturelayer: unsafe{transmute(get_proc_address("glNamedFramebufferTextureLayer"))},
			namedframebufferdrawbuffer: unsafe{transmute(get_proc_address("glNamedFramebufferDrawBuffer"))},
			namedframebufferdrawbuffers: unsafe{transmute(get_proc_address("glNamedFramebufferDrawBuffers"))},
			namedframebufferreadbuffer: unsafe{transmute(get_proc_address("glNamedFramebufferReadBuffer"))},
			invalidatenamedframebufferdata: unsafe{transmute(get_proc_address("glInvalidateNamedFramebufferData"))},
			invalidatenamedframebuffersubdata: unsafe{transmute(get_proc_address("glInvalidateNamedFramebufferSubData"))},
			clearnamedframebufferiv: unsafe{transmute(get_proc_address("glClearNamedFramebufferiv"))},
			clearnamedframebufferuiv: unsafe{transmute(get_proc_address("glClearNamedFramebufferuiv"))},
			clearnamedframebufferfv: unsafe{transmute(get_proc_address("glClearNamedFramebufferfv"))},
			clearnamedframebufferfi: unsafe{transmute(get_proc_address("glClearNamedFramebufferfi"))},
			blitnamedframebuffer: unsafe{transmute(get_proc_address("glBlitNamedFramebuffer"))},
			checknamedframebufferstatus: unsafe{transmute(get_proc_address("glCheckNamedFramebufferStatus"))},
			getnamedframebufferparameteriv: unsafe{transmute(get_proc_address("glGetNamedFramebufferParameteriv"))},
			getnamedframebufferattachmentparameteriv: unsafe{transmute(get_proc_address("glGetNamedFramebufferAttachmentParameteriv"))},
			createrenderbuffers: unsafe{transmute(get_proc_address("glCreateRenderbuffers"))},
			namedrenderbufferstorage: unsafe{transmute(get_proc_address("glNamedRenderbufferStorage"))},
			namedrenderbufferstoragemultisample: unsafe{transmute(get_proc_address("glNamedRenderbufferStorageMultisample"))},
			getnamedrenderbufferparameteriv: unsafe{transmute(get_proc_address("glGetNamedRenderbufferParameteriv"))},
			createtextures: unsafe{transmute(get_proc_address("glCreateTextures"))},
			texturebuffer: unsafe{transmute(get_proc_address("glTextureBuffer"))},
			texturebufferrange: unsafe{transmute(get_proc_address("glTextureBufferRange"))},
			texturestorage1d: unsafe{transmute(get_proc_address("glTextureStorage1D"))},
			texturestorage2d: unsafe{transmute(get_proc_address("glTextureStorage2D"))},
			texturestorage3d: unsafe{transmute(get_proc_address("glTextureStorage3D"))},
			texturestorage2dmultisample: unsafe{transmute(get_proc_address("glTextureStorage2DMultisample"))},
			texturestorage3dmultisample: unsafe{transmute(get_proc_address("glTextureStorage3DMultisample"))},
			texturesubimage1d: unsafe{transmute(get_proc_address("glTextureSubImage1D"))},
			texturesubimage2d: unsafe{transmute(get_proc_address("glTextureSubImage2D"))},
			texturesubimage3d: unsafe{transmute(get_proc_address("glTextureSubImage3D"))},
			compressedtexturesubimage1d: unsafe{transmute(get_proc_address("glCompressedTextureSubImage1D"))},
			compressedtexturesubimage2d: unsafe{transmute(get_proc_address("glCompressedTextureSubImage2D"))},
			compressedtexturesubimage3d: unsafe{transmute(get_proc_address("glCompressedTextureSubImage3D"))},
			copytexturesubimage1d: unsafe{transmute(get_proc_address("glCopyTextureSubImage1D"))},
			copytexturesubimage2d: unsafe{transmute(get_proc_address("glCopyTextureSubImage2D"))},
			copytexturesubimage3d: unsafe{transmute(get_proc_address("glCopyTextureSubImage3D"))},
			textureparameterf: unsafe{transmute(get_proc_address("glTextureParameterf"))},
			textureparameterfv: unsafe{transmute(get_proc_address("glTextureParameterfv"))},
			textureparameteri: unsafe{transmute(get_proc_address("glTextureParameteri"))},
			textureparameteriiv: unsafe{transmute(get_proc_address("glTextureParameterIiv"))},
			textureparameteriuiv: unsafe{transmute(get_proc_address("glTextureParameterIuiv"))},
			textureparameteriv: unsafe{transmute(get_proc_address("glTextureParameteriv"))},
			generatetexturemipmap: unsafe{transmute(get_proc_address("glGenerateTextureMipmap"))},
			bindtextureunit: unsafe{transmute(get_proc_address("glBindTextureUnit"))},
			gettextureimage: unsafe{transmute(get_proc_address("glGetTextureImage"))},
			getcompressedtextureimage: unsafe{transmute(get_proc_address("glGetCompressedTextureImage"))},
			gettexturelevelparameterfv: unsafe{transmute(get_proc_address("glGetTextureLevelParameterfv"))},
			gettexturelevelparameteriv: unsafe{transmute(get_proc_address("glGetTextureLevelParameteriv"))},
			gettextureparameterfv: unsafe{transmute(get_proc_address("glGetTextureParameterfv"))},
			gettextureparameteriiv: unsafe{transmute(get_proc_address("glGetTextureParameterIiv"))},
			gettextureparameteriuiv: unsafe{transmute(get_proc_address("glGetTextureParameterIuiv"))},
			gettextureparameteriv: unsafe{transmute(get_proc_address("glGetTextureParameteriv"))},
			createvertexarrays: unsafe{transmute(get_proc_address("glCreateVertexArrays"))},
			disablevertexarrayattrib: unsafe{transmute(get_proc_address("glDisableVertexArrayAttrib"))},
			enablevertexarrayattrib: unsafe{transmute(get_proc_address("glEnableVertexArrayAttrib"))},
			vertexarrayelementbuffer: unsafe{transmute(get_proc_address("glVertexArrayElementBuffer"))},
			vertexarrayvertexbuffer: unsafe{transmute(get_proc_address("glVertexArrayVertexBuffer"))},
			vertexarrayvertexbuffers: unsafe{transmute(get_proc_address("glVertexArrayVertexBuffers"))},
			vertexarrayattribbinding: unsafe{transmute(get_proc_address("glVertexArrayAttribBinding"))},
			vertexarrayattribformat: unsafe{transmute(get_proc_address("glVertexArrayAttribFormat"))},
			vertexarrayattribiformat: unsafe{transmute(get_proc_address("glVertexArrayAttribIFormat"))},
			vertexarrayattriblformat: unsafe{transmute(get_proc_address("glVertexArrayAttribLFormat"))},
			vertexarraybindingdivisor: unsafe{transmute(get_proc_address("glVertexArrayBindingDivisor"))},
			getvertexarrayiv: unsafe{transmute(get_proc_address("glGetVertexArrayiv"))},
			getvertexarrayindexediv: unsafe{transmute(get_proc_address("glGetVertexArrayIndexediv"))},
			getvertexarrayindexed64iv: unsafe{transmute(get_proc_address("glGetVertexArrayIndexed64iv"))},
			createsamplers: unsafe{transmute(get_proc_address("glCreateSamplers"))},
			createprogrampipelines: unsafe{transmute(get_proc_address("glCreateProgramPipelines"))},
			createqueries: unsafe{transmute(get_proc_address("glCreateQueries"))},
			getquerybufferobjecti64v: unsafe{transmute(get_proc_address("glGetQueryBufferObjecti64v"))},
			getquerybufferobjectiv: unsafe{transmute(get_proc_address("glGetQueryBufferObjectiv"))},
			getquerybufferobjectui64v: unsafe{transmute(get_proc_address("glGetQueryBufferObjectui64v"))},
			getquerybufferobjectuiv: unsafe{transmute(get_proc_address("glGetQueryBufferObjectuiv"))},
			memorybarrierbyregion: unsafe{transmute(get_proc_address("glMemoryBarrierByRegion"))},
			gettexturesubimage: unsafe{transmute(get_proc_address("glGetTextureSubImage"))},
			getcompressedtexturesubimage: unsafe{transmute(get_proc_address("glGetCompressedTextureSubImage"))},
			getgraphicsresetstatus: unsafe{transmute(get_proc_address("glGetGraphicsResetStatus"))},
			getncompressedteximage: unsafe{transmute(get_proc_address("glGetnCompressedTexImage"))},
			getnteximage: unsafe{transmute(get_proc_address("glGetnTexImage"))},
			getnuniformdv: unsafe{transmute(get_proc_address("glGetnUniformdv"))},
			getnuniformfv: unsafe{transmute(get_proc_address("glGetnUniformfv"))},
			getnuniformiv: unsafe{transmute(get_proc_address("glGetnUniformiv"))},
			getnuniformuiv: unsafe{transmute(get_proc_address("glGetnUniformuiv"))},
			readnpixels: unsafe{transmute(get_proc_address("glReadnPixels"))},
			getnmapdv: unsafe{transmute(get_proc_address("glGetnMapdv"))},
			getnmapfv: unsafe{transmute(get_proc_address("glGetnMapfv"))},
			getnmapiv: unsafe{transmute(get_proc_address("glGetnMapiv"))},
			getnpixelmapfv: unsafe{transmute(get_proc_address("glGetnPixelMapfv"))},
			getnpixelmapuiv: unsafe{transmute(get_proc_address("glGetnPixelMapuiv"))},
			getnpixelmapusv: unsafe{transmute(get_proc_address("glGetnPixelMapusv"))},
			getnpolygonstipple: unsafe{transmute(get_proc_address("glGetnPolygonStipple"))},
			getncolortable: unsafe{transmute(get_proc_address("glGetnColorTable"))},
			getnconvolutionfilter: unsafe{transmute(get_proc_address("glGetnConvolutionFilter"))},
			getnseparablefilter: unsafe{transmute(get_proc_address("glGetnSeparableFilter"))},
			getnhistogram: unsafe{transmute(get_proc_address("glGetnHistogram"))},
			getnminmax: unsafe{transmute(get_proc_address("glGetnMinmax"))},
			texturebarrier: unsafe{transmute(get_proc_address("glTextureBarrier"))},
		}
	}
	#[inline(always)]
	pub fn get_available(&self) -> bool {
		self.available
	}
}

impl Default for Version45 {
	fn default() -> Self {
		Self {
			available: false,
			clipcontrol: null(),
			createtransformfeedbacks: null(),
			transformfeedbackbufferbase: null(),
			transformfeedbackbufferrange: null(),
			gettransformfeedbackiv: null(),
			gettransformfeedbacki_v: null(),
			gettransformfeedbacki64_v: null(),
			createbuffers: null(),
			namedbufferstorage: null(),
			namedbufferdata: null(),
			namedbuffersubdata: null(),
			copynamedbuffersubdata: null(),
			clearnamedbufferdata: null(),
			clearnamedbuffersubdata: null(),
			mapnamedbuffer: null(),
			mapnamedbufferrange: null(),
			unmapnamedbuffer: null(),
			flushmappednamedbufferrange: null(),
			getnamedbufferparameteriv: null(),
			getnamedbufferparameteri64v: null(),
			getnamedbufferpointerv: null(),
			getnamedbuffersubdata: null(),
			createframebuffers: null(),
			namedframebufferrenderbuffer: null(),
			namedframebufferparameteri: null(),
			namedframebuffertexture: null(),
			namedframebuffertexturelayer: null(),
			namedframebufferdrawbuffer: null(),
			namedframebufferdrawbuffers: null(),
			namedframebufferreadbuffer: null(),
			invalidatenamedframebufferdata: null(),
			invalidatenamedframebuffersubdata: null(),
			clearnamedframebufferiv: null(),
			clearnamedframebufferuiv: null(),
			clearnamedframebufferfv: null(),
			clearnamedframebufferfi: null(),
			blitnamedframebuffer: null(),
			checknamedframebufferstatus: null(),
			getnamedframebufferparameteriv: null(),
			getnamedframebufferattachmentparameteriv: null(),
			createrenderbuffers: null(),
			namedrenderbufferstorage: null(),
			namedrenderbufferstoragemultisample: null(),
			getnamedrenderbufferparameteriv: null(),
			createtextures: null(),
			texturebuffer: null(),
			texturebufferrange: null(),
			texturestorage1d: null(),
			texturestorage2d: null(),
			texturestorage3d: null(),
			texturestorage2dmultisample: null(),
			texturestorage3dmultisample: null(),
			texturesubimage1d: null(),
			texturesubimage2d: null(),
			texturesubimage3d: null(),
			compressedtexturesubimage1d: null(),
			compressedtexturesubimage2d: null(),
			compressedtexturesubimage3d: null(),
			copytexturesubimage1d: null(),
			copytexturesubimage2d: null(),
			copytexturesubimage3d: null(),
			textureparameterf: null(),
			textureparameterfv: null(),
			textureparameteri: null(),
			textureparameteriiv: null(),
			textureparameteriuiv: null(),
			textureparameteriv: null(),
			generatetexturemipmap: null(),
			bindtextureunit: null(),
			gettextureimage: null(),
			getcompressedtextureimage: null(),
			gettexturelevelparameterfv: null(),
			gettexturelevelparameteriv: null(),
			gettextureparameterfv: null(),
			gettextureparameteriiv: null(),
			gettextureparameteriuiv: null(),
			gettextureparameteriv: null(),
			createvertexarrays: null(),
			disablevertexarrayattrib: null(),
			enablevertexarrayattrib: null(),
			vertexarrayelementbuffer: null(),
			vertexarrayvertexbuffer: null(),
			vertexarrayvertexbuffers: null(),
			vertexarrayattribbinding: null(),
			vertexarrayattribformat: null(),
			vertexarrayattribiformat: null(),
			vertexarrayattriblformat: null(),
			vertexarraybindingdivisor: null(),
			getvertexarrayiv: null(),
			getvertexarrayindexediv: null(),
			getvertexarrayindexed64iv: null(),
			createsamplers: null(),
			createprogrampipelines: null(),
			createqueries: null(),
			getquerybufferobjecti64v: null(),
			getquerybufferobjectiv: null(),
			getquerybufferobjectui64v: null(),
			getquerybufferobjectuiv: null(),
			memorybarrierbyregion: null(),
			gettexturesubimage: null(),
			getcompressedtexturesubimage: null(),
			getgraphicsresetstatus: null(),
			getncompressedteximage: null(),
			getnteximage: null(),
			getnuniformdv: null(),
			getnuniformfv: null(),
			getnuniformiv: null(),
			getnuniformuiv: null(),
			readnpixels: null(),
			getnmapdv: null(),
			getnmapfv: null(),
			getnmapiv: null(),
			getnpixelmapfv: null(),
			getnpixelmapuiv: null(),
			getnpixelmapusv: null(),
			getnpolygonstipple: null(),
			getncolortable: null(),
			getnconvolutionfilter: null(),
			getnseparablefilter: null(),
			getnhistogram: null(),
			getnminmax: null(),
			texturebarrier: null(),
		}
	}
}

type PFNGLSPECIALIZESHADERPROC = extern "system" fn(GLuint, *const GLchar, GLuint, *const GLuint, *const GLuint);
type PFNGLMULTIDRAWARRAYSINDIRECTCOUNTPROC = extern "system" fn(GLenum, *const c_void, GLintptr, GLsizei, GLsizei);
type PFNGLMULTIDRAWELEMENTSINDIRECTCOUNTPROC = extern "system" fn(GLenum, GLenum, *const c_void, GLintptr, GLsizei, GLsizei);
type PFNGLPOLYGONOFFSETCLAMPPROC = extern "system" fn(GLfloat, GLfloat, GLfloat);
const SHADER_BINARY_FORMAT_SPIR_V: GLenum = 0x9551;
const SPIR_V_BINARY: GLenum = 0x9552;
const PARAMETER_BUFFER: GLenum = 0x80EE;
const PARAMETER_BUFFER_BINDING: GLenum = 0x80EF;
const CONTEXT_FLAG_NO_ERROR_BIT: GLbitfield = 0x00000008;
const VERTICES_SUBMITTED: GLenum = 0x82EE;
const PRIMITIVES_SUBMITTED: GLenum = 0x82EF;
const VERTEX_SHADER_INVOCATIONS: GLenum = 0x82F0;
const TESS_CONTROL_SHADER_PATCHES: GLenum = 0x82F1;
const TESS_EVALUATION_SHADER_INVOCATIONS: GLenum = 0x82F2;
const GEOMETRY_SHADER_PRIMITIVES_EMITTED: GLenum = 0x82F3;
const FRAGMENT_SHADER_INVOCATIONS: GLenum = 0x82F4;
const COMPUTE_SHADER_INVOCATIONS: GLenum = 0x82F5;
const CLIPPING_INPUT_PRIMITIVES: GLenum = 0x82F6;
const CLIPPING_OUTPUT_PRIMITIVES: GLenum = 0x82F7;
const POLYGON_OFFSET_CLAMP: GLenum = 0x8E1B;
const SPIR_V_EXTENSIONS: GLenum = 0x9553;
const NUM_SPIR_V_EXTENSIONS: GLenum = 0x9554;
const TEXTURE_MAX_ANISOTROPY: GLenum = 0x84FE;
const MAX_TEXTURE_MAX_ANISOTROPY: GLenum = 0x84FF;
const TRANSFORM_FEEDBACK_OVERFLOW: GLenum = 0x82EC;
const TRANSFORM_FEEDBACK_STREAM_OVERFLOW: GLenum = 0x82ED;

pub trait GL_4_6 {
	fn glSpecializeShader(&self, _: GLuint, _: *const GLchar, _: GLuint, _: *const GLuint, _: *const GLuint) {
		panic!("OpenGL function pointer of `glSpecializeShader` is NULL");
	}
	fn glMultiDrawArraysIndirectCount(&self, _: GLenum, _: *const c_void, _: GLintptr, _: GLsizei, _: GLsizei) {
		panic!("OpenGL function pointer of `glMultiDrawArraysIndirectCount` is NULL");
	}
	fn glMultiDrawElementsIndirectCount(&self, _: GLenum, _: GLenum, _: *const c_void, _: GLintptr, _: GLsizei, _: GLsizei) {
		panic!("OpenGL function pointer of `glMultiDrawElementsIndirectCount` is NULL");
	}
	fn glPolygonOffsetClamp(&self, _: GLfloat, _: GLfloat, _: GLfloat) {
		panic!("OpenGL function pointer of `glPolygonOffsetClamp` is NULL");
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Version46 {
	available: bool,
	specializeshader: PFNGLSPECIALIZESHADERPROC,
	multidrawarraysindirectcount: PFNGLMULTIDRAWARRAYSINDIRECTCOUNTPROC,
	multidrawelementsindirectcount: PFNGLMULTIDRAWELEMENTSINDIRECTCOUNTPROC,
	polygonoffsetclamp: PFNGLPOLYGONOFFSETCLAMPPROC,
}

impl GL_4_6 for Version46 {
	#[inline(always)]
	fn glSpecializeShader(&self, shader: GLuint, pEntryPoint: *const GLchar, numSpecializationConstants: GLuint, pConstantIndex: *const GLuint, pConstantValue: *const GLuint) {
		(self.specializeshader)(shader, pEntryPoint, numSpecializationConstants, pConstantIndex, pConstantValue)
	}
	#[inline(always)]
	fn glMultiDrawArraysIndirectCount(&self, mode: GLenum, indirect: *const c_void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei) {
		(self.multidrawarraysindirectcount)(mode, indirect, drawcount, maxdrawcount, stride)
	}
	#[inline(always)]
	fn glMultiDrawElementsIndirectCount(&self, mode: GLenum, type_: GLenum, indirect: *const c_void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei) {
		(self.multidrawelementsindirectcount)(mode, type_, indirect, drawcount, maxdrawcount, stride)
	}
	#[inline(always)]
	fn glPolygonOffsetClamp(&self, factor: GLfloat, units: GLfloat, clamp: GLfloat) {
		(self.polygonoffsetclamp)(factor, units, clamp)
	}
}

impl Version46 {
	pub fn new(base: impl GL_1_0, get_proc_address: &impl Fn(&'static str) -> *const c_void) -> Self {
		let (spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 40600 {
			return Self::default();
		}
		Self {
			available: true,
			specializeshader: unsafe{transmute(get_proc_address("glSpecializeShader"))},
			multidrawarraysindirectcount: unsafe{transmute(get_proc_address("glMultiDrawArraysIndirectCount"))},
			multidrawelementsindirectcount: unsafe{transmute(get_proc_address("glMultiDrawElementsIndirectCount"))},
			polygonoffsetclamp: unsafe{transmute(get_proc_address("glPolygonOffsetClamp"))},
		}
	}
	#[inline(always)]
	pub fn get_available(&self) -> bool {
		self.available
	}
}

impl Default for Version46 {
	fn default() -> Self {
		Self {
			available: false,
			specializeshader: null(),
			multidrawarraysindirectcount: null(),
			multidrawelementsindirectcount: null(),
			polygonoffsetclamp: null(),
		}
	}
}
#[derive(Debug, Clone, Copy)]
pub struct GLCore {
	pub version_1_0: Version10,
	pub version_1_1: Version11,
	pub version_1_2: Version12,
	pub version_1_3: Version13,
	pub version_1_4: Version14,
	pub version_1_5: Version15,
	pub version_2_0: Version20,
	pub version_2_1: Version21,
	pub version_3_0: Version30,
	pub version_3_1: Version31,
	pub version_3_2: Version32,
	pub version_3_3: Version33,
	pub version_4_0: Version40,
	pub version_4_1: Version41,
	pub version_4_2: Version42,
	pub version_4_3: Version43,
	pub version_4_4: Version44,
	pub version_4_5: Version45,
	pub version_4_6: Version46,
}

impl GL_1_0 for GLCore {
	#[inline(always)]
	fn glCullFace(&self, mode: GLenum) {
		(self.version_1_0.cullface)(mode)
	}
	#[inline(always)]
	fn glFrontFace(&self, mode: GLenum) {
		(self.version_1_0.frontface)(mode)
	}
	#[inline(always)]
	fn glHint(&self, target: GLenum, mode: GLenum) {
		(self.version_1_0.hint)(target, mode)
	}
	#[inline(always)]
	fn glLineWidth(&self, width: GLfloat) {
		(self.version_1_0.linewidth)(width)
	}
	#[inline(always)]
	fn glPointSize(&self, size: GLfloat) {
		(self.version_1_0.pointsize)(size)
	}
	#[inline(always)]
	fn glPolygonMode(&self, face: GLenum, mode: GLenum) {
		(self.version_1_0.polygonmode)(face, mode)
	}
	#[inline(always)]
	fn glScissor(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
		(self.version_1_0.scissor)(x, y, width, height)
	}
	#[inline(always)]
	fn glTexParameterf(&self, target: GLenum, pname: GLenum, param: GLfloat) {
		(self.version_1_0.texparameterf)(target, pname, param)
	}
	#[inline(always)]
	fn glTexParameterfv(&self, target: GLenum, pname: GLenum, params: *const GLfloat) {
		(self.version_1_0.texparameterfv)(target, pname, params)
	}
	#[inline(always)]
	fn glTexParameteri(&self, target: GLenum, pname: GLenum, param: GLint) {
		(self.version_1_0.texparameteri)(target, pname, param)
	}
	#[inline(always)]
	fn glTexParameteriv(&self, target: GLenum, pname: GLenum, params: *const GLint) {
		(self.version_1_0.texparameteriv)(target, pname, params)
	}
	#[inline(always)]
	fn glTexImage1D(&self, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void) {
		(self.version_1_0.teximage1d)(target, level, internalformat, width, border, format, type_, pixels)
	}
	#[inline(always)]
	fn glTexImage2D(&self, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void) {
		(self.version_1_0.teximage2d)(target, level, internalformat, width, height, border, format, type_, pixels)
	}
	#[inline(always)]
	fn glDrawBuffer(&self, buf: GLenum) {
		(self.version_1_0.drawbuffer)(buf)
	}
	#[inline(always)]
	fn glClear(&self, mask: GLbitfield) {
		(self.version_1_0.clear)(mask)
	}
	#[inline(always)]
	fn glClearColor(&self, red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
		(self.version_1_0.clearcolor)(red, green, blue, alpha)
	}
	#[inline(always)]
	fn glClearStencil(&self, s: GLint) {
		(self.version_1_0.clearstencil)(s)
	}
	#[inline(always)]
	fn glClearDepth(&self, depth: GLdouble) {
		(self.version_1_0.cleardepth)(depth)
	}
	#[inline(always)]
	fn glStencilMask(&self, mask: GLuint) {
		(self.version_1_0.stencilmask)(mask)
	}
	#[inline(always)]
	fn glColorMask(&self, red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) {
		(self.version_1_0.colormask)(red, green, blue, alpha)
	}
	#[inline(always)]
	fn glDepthMask(&self, flag: GLboolean) {
		(self.version_1_0.depthmask)(flag)
	}
	#[inline(always)]
	fn glDisable(&self, cap: GLenum) {
		(self.version_1_0.disable)(cap)
	}
	#[inline(always)]
	fn glEnable(&self, cap: GLenum) {
		(self.version_1_0.enable)(cap)
	}
	#[inline(always)]
	fn glFinish(&self) {
		(self.version_1_0.finish)()
	}
	#[inline(always)]
	fn glFlush(&self) {
		(self.version_1_0.flush)()
	}
	#[inline(always)]
	fn glBlendFunc(&self, sfactor: GLenum, dfactor: GLenum) {
		(self.version_1_0.blendfunc)(sfactor, dfactor)
	}
	#[inline(always)]
	fn glLogicOp(&self, opcode: GLenum) {
		(self.version_1_0.logicop)(opcode)
	}
	#[inline(always)]
	fn glStencilFunc(&self, func: GLenum, ref_: GLint, mask: GLuint) {
		(self.version_1_0.stencilfunc)(func, ref_, mask)
	}
	#[inline(always)]
	fn glStencilOp(&self, fail: GLenum, zfail: GLenum, zpass: GLenum) {
		(self.version_1_0.stencilop)(fail, zfail, zpass)
	}
	#[inline(always)]
	fn glDepthFunc(&self, func: GLenum) {
		(self.version_1_0.depthfunc)(func)
	}
	#[inline(always)]
	fn glPixelStoref(&self, pname: GLenum, param: GLfloat) {
		(self.version_1_0.pixelstoref)(pname, param)
	}
	#[inline(always)]
	fn glPixelStorei(&self, pname: GLenum, param: GLint) {
		(self.version_1_0.pixelstorei)(pname, param)
	}
	#[inline(always)]
	fn glReadBuffer(&self, src: GLenum) {
		(self.version_1_0.readbuffer)(src)
	}
	#[inline(always)]
	fn glReadPixels(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *mut c_void) {
		(self.version_1_0.readpixels)(x, y, width, height, format, type_, pixels)
	}
	#[inline(always)]
	fn glGetBooleanv(&self, pname: GLenum, data: *mut GLboolean) {
		(self.version_1_0.getbooleanv)(pname, data)
	}
	#[inline(always)]
	fn glGetDoublev(&self, pname: GLenum, data: *mut GLdouble) {
		(self.version_1_0.getdoublev)(pname, data)
	}
	#[inline(always)]
	fn glGetError(&self) -> GLenum {
		(self.version_1_0.geterror)()
	}
	#[inline(always)]
	fn glGetFloatv(&self, pname: GLenum, data: *mut GLfloat) {
		(self.version_1_0.getfloatv)(pname, data)
	}
	#[inline(always)]
	fn glGetIntegerv(&self, pname: GLenum, data: *mut GLint) {
		(self.version_1_0.getintegerv)(pname, data)
	}
	#[inline(always)]
	fn glGetString(&self, name: GLenum) -> *const GLubyte {
		(self.version_1_0.getstring)(name)
	}
	#[inline(always)]
	fn glGetTexImage(&self, target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *mut c_void) {
		(self.version_1_0.getteximage)(target, level, format, type_, pixels)
	}
	#[inline(always)]
	fn glGetTexParameterfv(&self, target: GLenum, pname: GLenum, params: *mut GLfloat) {
		(self.version_1_0.gettexparameterfv)(target, pname, params)
	}
	#[inline(always)]
	fn glGetTexParameteriv(&self, target: GLenum, pname: GLenum, params: *mut GLint) {
		(self.version_1_0.gettexparameteriv)(target, pname, params)
	}
	#[inline(always)]
	fn glGetTexLevelParameterfv(&self, target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat) {
		(self.version_1_0.gettexlevelparameterfv)(target, level, pname, params)
	}
	#[inline(always)]
	fn glGetTexLevelParameteriv(&self, target: GLenum, level: GLint, pname: GLenum, params: *mut GLint) {
		(self.version_1_0.gettexlevelparameteriv)(target, level, pname, params)
	}
	#[inline(always)]
	fn glIsEnabled(&self, cap: GLenum) -> GLboolean {
		(self.version_1_0.isenabled)(cap)
	}
	#[inline(always)]
	fn glDepthRange(&self, n: GLdouble, f: GLdouble) {
		(self.version_1_0.depthrange)(n, f)
	}
	#[inline(always)]
	fn glViewport(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
		(self.version_1_0.viewport)(x, y, width, height)
	}
	#[inline(always)]
	fn get_version(&self) -> (&'static str, u32, u32, u32) {
		self.version_1_0.get_version()
	}
	#[inline(always)]
	fn get_vendor(&self) -> &'static str {
		self.version_1_0.get_vendor()
	}
	#[inline(always)]
	fn get_renderer(&self) -> &'static str {
		self.version_1_0.get_renderer()
	}
	#[inline(always)]
	fn get_versionstr(&self) -> &'static str {
		self.version_1_0.get_versionstr()
	}
}

impl GL_1_1 for GLCore {
	#[inline(always)]
	fn glDrawArrays(&self, mode: GLenum, first: GLint, count: GLsizei) {
		(self.version_1_1.drawarrays)(mode, first, count)
	}
	#[inline(always)]
	fn glDrawElements(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void) {
		(self.version_1_1.drawelements)(mode, count, type_, indices)
	}
	#[inline(always)]
	fn glGetPointerv(&self, pname: GLenum, params: *mut *mut c_void) {
		(self.version_1_1.getpointerv)(pname, params)
	}
	#[inline(always)]
	fn glPolygonOffset(&self, factor: GLfloat, units: GLfloat) {
		(self.version_1_1.polygonoffset)(factor, units)
	}
	#[inline(always)]
	fn glCopyTexImage1D(&self, target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint) {
		(self.version_1_1.copyteximage1d)(target, level, internalformat, x, y, width, border)
	}
	#[inline(always)]
	fn glCopyTexImage2D(&self, target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint) {
		(self.version_1_1.copyteximage2d)(target, level, internalformat, x, y, width, height, border)
	}
	#[inline(always)]
	fn glCopyTexSubImage1D(&self, target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) {
		(self.version_1_1.copytexsubimage1d)(target, level, xoffset, x, y, width)
	}
	#[inline(always)]
	fn glCopyTexSubImage2D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
		(self.version_1_1.copytexsubimage2d)(target, level, xoffset, yoffset, x, y, width, height)
	}
	#[inline(always)]
	fn glTexSubImage1D(&self, target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) {
		(self.version_1_1.texsubimage1d)(target, level, xoffset, width, format, type_, pixels)
	}
	#[inline(always)]
	fn glTexSubImage2D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) {
		(self.version_1_1.texsubimage2d)(target, level, xoffset, yoffset, width, height, format, type_, pixels)
	}
	#[inline(always)]
	fn glBindTexture(&self, target: GLenum, texture: GLuint) {
		(self.version_1_1.bindtexture)(target, texture)
	}
	#[inline(always)]
	fn glDeleteTextures(&self, n: GLsizei, textures: *const GLuint) {
		(self.version_1_1.deletetextures)(n, textures)
	}
	#[inline(always)]
	fn glGenTextures(&self, n: GLsizei, textures: *mut GLuint) {
		(self.version_1_1.gentextures)(n, textures)
	}
	#[inline(always)]
	fn glIsTexture(&self, texture: GLuint) -> GLboolean {
		(self.version_1_1.istexture)(texture)
	}
}

impl GL_1_2 for GLCore {
	#[inline(always)]
	fn glDrawRangeElements(&self, mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const c_void) {
		(self.version_1_2.drawrangeelements)(mode, start, end, count, type_, indices)
	}
	#[inline(always)]
	fn glTexImage3D(&self, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void) {
		(self.version_1_2.teximage3d)(target, level, internalformat, width, height, depth, border, format, type_, pixels)
	}
	#[inline(always)]
	fn glTexSubImage3D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) {
		(self.version_1_2.texsubimage3d)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels)
	}
	#[inline(always)]
	fn glCopyTexSubImage3D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
		(self.version_1_2.copytexsubimage3d)(target, level, xoffset, yoffset, zoffset, x, y, width, height)
	}
}

impl GL_1_3 for GLCore {
	#[inline(always)]
	fn glActiveTexture(&self, texture: GLenum) {
		(self.version_1_3.activetexture)(texture)
	}
	#[inline(always)]
	fn glSampleCoverage(&self, value: GLfloat, invert: GLboolean) {
		(self.version_1_3.samplecoverage)(value, invert)
	}
	#[inline(always)]
	fn glCompressedTexImage3D(&self, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void) {
		(self.version_1_3.compressedteximage3d)(target, level, internalformat, width, height, depth, border, imageSize, data)
	}
	#[inline(always)]
	fn glCompressedTexImage2D(&self, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void) {
		(self.version_1_3.compressedteximage2d)(target, level, internalformat, width, height, border, imageSize, data)
	}
	#[inline(always)]
	fn glCompressedTexImage1D(&self, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void) {
		(self.version_1_3.compressedteximage1d)(target, level, internalformat, width, border, imageSize, data)
	}
	#[inline(always)]
	fn glCompressedTexSubImage3D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) {
		(self.version_1_3.compressedtexsubimage3d)(target, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data)
	}
	#[inline(always)]
	fn glCompressedTexSubImage2D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) {
		(self.version_1_3.compressedtexsubimage2d)(target, level, xoffset, yoffset, width, height, format, imageSize, data)
	}
	#[inline(always)]
	fn glCompressedTexSubImage1D(&self, target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) {
		(self.version_1_3.compressedtexsubimage1d)(target, level, xoffset, width, format, imageSize, data)
	}
	#[inline(always)]
	fn glGetCompressedTexImage(&self, target: GLenum, level: GLint, img: *mut c_void) {
		(self.version_1_3.getcompressedteximage)(target, level, img)
	}
	#[inline(always)]
	fn glClientActiveTexture(&self, texture: GLenum) {
		(self.version_1_3.clientactivetexture)(texture)
	}
	#[inline(always)]
	fn glMultiTexCoord1d(&self, target: GLenum, s: GLdouble) {
		(self.version_1_3.multitexcoord1d)(target, s)
	}
	#[inline(always)]
	fn glMultiTexCoord1dv(&self, target: GLenum, v: *const GLdouble) {
		(self.version_1_3.multitexcoord1dv)(target, v)
	}
	#[inline(always)]
	fn glMultiTexCoord1f(&self, target: GLenum, s: GLfloat) {
		(self.version_1_3.multitexcoord1f)(target, s)
	}
	#[inline(always)]
	fn glMultiTexCoord1fv(&self, target: GLenum, v: *const GLfloat) {
		(self.version_1_3.multitexcoord1fv)(target, v)
	}
	#[inline(always)]
	fn glMultiTexCoord1i(&self, target: GLenum, s: GLint) {
		(self.version_1_3.multitexcoord1i)(target, s)
	}
	#[inline(always)]
	fn glMultiTexCoord1iv(&self, target: GLenum, v: *const GLint) {
		(self.version_1_3.multitexcoord1iv)(target, v)
	}
	#[inline(always)]
	fn glMultiTexCoord1s(&self, target: GLenum, s: GLshort) {
		(self.version_1_3.multitexcoord1s)(target, s)
	}
	#[inline(always)]
	fn glMultiTexCoord1sv(&self, target: GLenum, v: *const GLshort) {
		(self.version_1_3.multitexcoord1sv)(target, v)
	}
	#[inline(always)]
	fn glMultiTexCoord2d(&self, target: GLenum, s: GLdouble, t: GLdouble) {
		(self.version_1_3.multitexcoord2d)(target, s, t)
	}
	#[inline(always)]
	fn glMultiTexCoord2dv(&self, target: GLenum, v: *const GLdouble) {
		(self.version_1_3.multitexcoord2dv)(target, v)
	}
	#[inline(always)]
	fn glMultiTexCoord2f(&self, target: GLenum, s: GLfloat, t: GLfloat) {
		(self.version_1_3.multitexcoord2f)(target, s, t)
	}
	#[inline(always)]
	fn glMultiTexCoord2fv(&self, target: GLenum, v: *const GLfloat) {
		(self.version_1_3.multitexcoord2fv)(target, v)
	}
	#[inline(always)]
	fn glMultiTexCoord2i(&self, target: GLenum, s: GLint, t: GLint) {
		(self.version_1_3.multitexcoord2i)(target, s, t)
	}
	#[inline(always)]
	fn glMultiTexCoord2iv(&self, target: GLenum, v: *const GLint) {
		(self.version_1_3.multitexcoord2iv)(target, v)
	}
	#[inline(always)]
	fn glMultiTexCoord2s(&self, target: GLenum, s: GLshort, t: GLshort) {
		(self.version_1_3.multitexcoord2s)(target, s, t)
	}
	#[inline(always)]
	fn glMultiTexCoord2sv(&self, target: GLenum, v: *const GLshort) {
		(self.version_1_3.multitexcoord2sv)(target, v)
	}
	#[inline(always)]
	fn glMultiTexCoord3d(&self, target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble) {
		(self.version_1_3.multitexcoord3d)(target, s, t, r)
	}
	#[inline(always)]
	fn glMultiTexCoord3dv(&self, target: GLenum, v: *const GLdouble) {
		(self.version_1_3.multitexcoord3dv)(target, v)
	}
	#[inline(always)]
	fn glMultiTexCoord3f(&self, target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat) {
		(self.version_1_3.multitexcoord3f)(target, s, t, r)
	}
	#[inline(always)]
	fn glMultiTexCoord3fv(&self, target: GLenum, v: *const GLfloat) {
		(self.version_1_3.multitexcoord3fv)(target, v)
	}
	#[inline(always)]
	fn glMultiTexCoord3i(&self, target: GLenum, s: GLint, t: GLint, r: GLint) {
		(self.version_1_3.multitexcoord3i)(target, s, t, r)
	}
	#[inline(always)]
	fn glMultiTexCoord3iv(&self, target: GLenum, v: *const GLint) {
		(self.version_1_3.multitexcoord3iv)(target, v)
	}
	#[inline(always)]
	fn glMultiTexCoord3s(&self, target: GLenum, s: GLshort, t: GLshort, r: GLshort) {
		(self.version_1_3.multitexcoord3s)(target, s, t, r)
	}
	#[inline(always)]
	fn glMultiTexCoord3sv(&self, target: GLenum, v: *const GLshort) {
		(self.version_1_3.multitexcoord3sv)(target, v)
	}
	#[inline(always)]
	fn glMultiTexCoord4d(&self, target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble) {
		(self.version_1_3.multitexcoord4d)(target, s, t, r, q)
	}
	#[inline(always)]
	fn glMultiTexCoord4dv(&self, target: GLenum, v: *const GLdouble) {
		(self.version_1_3.multitexcoord4dv)(target, v)
	}
	#[inline(always)]
	fn glMultiTexCoord4f(&self, target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat) {
		(self.version_1_3.multitexcoord4f)(target, s, t, r, q)
	}
	#[inline(always)]
	fn glMultiTexCoord4fv(&self, target: GLenum, v: *const GLfloat) {
		(self.version_1_3.multitexcoord4fv)(target, v)
	}
	#[inline(always)]
	fn glMultiTexCoord4i(&self, target: GLenum, s: GLint, t: GLint, r: GLint, q: GLint) {
		(self.version_1_3.multitexcoord4i)(target, s, t, r, q)
	}
	#[inline(always)]
	fn glMultiTexCoord4iv(&self, target: GLenum, v: *const GLint) {
		(self.version_1_3.multitexcoord4iv)(target, v)
	}
	#[inline(always)]
	fn glMultiTexCoord4s(&self, target: GLenum, s: GLshort, t: GLshort, r: GLshort, q: GLshort) {
		(self.version_1_3.multitexcoord4s)(target, s, t, r, q)
	}
	#[inline(always)]
	fn glMultiTexCoord4sv(&self, target: GLenum, v: *const GLshort) {
		(self.version_1_3.multitexcoord4sv)(target, v)
	}
	#[inline(always)]
	fn glLoadTransposeMatrixf(&self, m: *const GLfloat) {
		(self.version_1_3.loadtransposematrixf)(m)
	}
	#[inline(always)]
	fn glLoadTransposeMatrixd(&self, m: *const GLdouble) {
		(self.version_1_3.loadtransposematrixd)(m)
	}
	#[inline(always)]
	fn glMultTransposeMatrixf(&self, m: *const GLfloat) {
		(self.version_1_3.multtransposematrixf)(m)
	}
	#[inline(always)]
	fn glMultTransposeMatrixd(&self, m: *const GLdouble) {
		(self.version_1_3.multtransposematrixd)(m)
	}
}

impl GL_1_4 for GLCore {
	#[inline(always)]
	fn glBlendFuncSeparate(&self, sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum) {
		(self.version_1_4.blendfuncseparate)(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha)
	}
	#[inline(always)]
	fn glMultiDrawArrays(&self, mode: GLenum, first: *const GLint, count: *const GLsizei, drawcount: GLsizei) {
		(self.version_1_4.multidrawarrays)(mode, first, count, drawcount)
	}
	#[inline(always)]
	fn glMultiDrawElements(&self, mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const c_void, drawcount: GLsizei) {
		(self.version_1_4.multidrawelements)(mode, count, type_, indices, drawcount)
	}
	#[inline(always)]
	fn glPointParameterf(&self, pname: GLenum, param: GLfloat) {
		(self.version_1_4.pointparameterf)(pname, param)
	}
	#[inline(always)]
	fn glPointParameterfv(&self, pname: GLenum, params: *const GLfloat) {
		(self.version_1_4.pointparameterfv)(pname, params)
	}
	#[inline(always)]
	fn glPointParameteri(&self, pname: GLenum, param: GLint) {
		(self.version_1_4.pointparameteri)(pname, param)
	}
	#[inline(always)]
	fn glPointParameteriv(&self, pname: GLenum, params: *const GLint) {
		(self.version_1_4.pointparameteriv)(pname, params)
	}
	#[inline(always)]
	fn glFogCoordf(&self, coord: GLfloat) {
		(self.version_1_4.fogcoordf)(coord)
	}
	#[inline(always)]
	fn glFogCoordfv(&self, coord: *const GLfloat) {
		(self.version_1_4.fogcoordfv)(coord)
	}
	#[inline(always)]
	fn glFogCoordd(&self, coord: GLdouble) {
		(self.version_1_4.fogcoordd)(coord)
	}
	#[inline(always)]
	fn glFogCoorddv(&self, coord: *const GLdouble) {
		(self.version_1_4.fogcoorddv)(coord)
	}
	#[inline(always)]
	fn glFogCoordPointer(&self, type_: GLenum, stride: GLsizei, pointer: *const c_void) {
		(self.version_1_4.fogcoordpointer)(type_, stride, pointer)
	}
	#[inline(always)]
	fn glSecondaryColor3b(&self, red: GLbyte, green: GLbyte, blue: GLbyte) {
		(self.version_1_4.secondarycolor3b)(red, green, blue)
	}
	#[inline(always)]
	fn glSecondaryColor3bv(&self, v: *const GLbyte) {
		(self.version_1_4.secondarycolor3bv)(v)
	}
	#[inline(always)]
	fn glSecondaryColor3d(&self, red: GLdouble, green: GLdouble, blue: GLdouble) {
		(self.version_1_4.secondarycolor3d)(red, green, blue)
	}
	#[inline(always)]
	fn glSecondaryColor3dv(&self, v: *const GLdouble) {
		(self.version_1_4.secondarycolor3dv)(v)
	}
	#[inline(always)]
	fn glSecondaryColor3f(&self, red: GLfloat, green: GLfloat, blue: GLfloat) {
		(self.version_1_4.secondarycolor3f)(red, green, blue)
	}
	#[inline(always)]
	fn glSecondaryColor3fv(&self, v: *const GLfloat) {
		(self.version_1_4.secondarycolor3fv)(v)
	}
	#[inline(always)]
	fn glSecondaryColor3i(&self, red: GLint, green: GLint, blue: GLint) {
		(self.version_1_4.secondarycolor3i)(red, green, blue)
	}
	#[inline(always)]
	fn glSecondaryColor3iv(&self, v: *const GLint) {
		(self.version_1_4.secondarycolor3iv)(v)
	}
	#[inline(always)]
	fn glSecondaryColor3s(&self, red: GLshort, green: GLshort, blue: GLshort) {
		(self.version_1_4.secondarycolor3s)(red, green, blue)
	}
	#[inline(always)]
	fn glSecondaryColor3sv(&self, v: *const GLshort) {
		(self.version_1_4.secondarycolor3sv)(v)
	}
	#[inline(always)]
	fn glSecondaryColor3ub(&self, red: GLubyte, green: GLubyte, blue: GLubyte) {
		(self.version_1_4.secondarycolor3ub)(red, green, blue)
	}
	#[inline(always)]
	fn glSecondaryColor3ubv(&self, v: *const GLubyte) {
		(self.version_1_4.secondarycolor3ubv)(v)
	}
	#[inline(always)]
	fn glSecondaryColor3ui(&self, red: GLuint, green: GLuint, blue: GLuint) {
		(self.version_1_4.secondarycolor3ui)(red, green, blue)
	}
	#[inline(always)]
	fn glSecondaryColor3uiv(&self, v: *const GLuint) {
		(self.version_1_4.secondarycolor3uiv)(v)
	}
	#[inline(always)]
	fn glSecondaryColor3us(&self, red: GLushort, green: GLushort, blue: GLushort) {
		(self.version_1_4.secondarycolor3us)(red, green, blue)
	}
	#[inline(always)]
	fn glSecondaryColor3usv(&self, v: *const GLushort) {
		(self.version_1_4.secondarycolor3usv)(v)
	}
	#[inline(always)]
	fn glSecondaryColorPointer(&self, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void) {
		(self.version_1_4.secondarycolorpointer)(size, type_, stride, pointer)
	}
	#[inline(always)]
	fn glWindowPos2d(&self, x: GLdouble, y: GLdouble) {
		(self.version_1_4.windowpos2d)(x, y)
	}
	#[inline(always)]
	fn glWindowPos2dv(&self, v: *const GLdouble) {
		(self.version_1_4.windowpos2dv)(v)
	}
	#[inline(always)]
	fn glWindowPos2f(&self, x: GLfloat, y: GLfloat) {
		(self.version_1_4.windowpos2f)(x, y)
	}
	#[inline(always)]
	fn glWindowPos2fv(&self, v: *const GLfloat) {
		(self.version_1_4.windowpos2fv)(v)
	}
	#[inline(always)]
	fn glWindowPos2i(&self, x: GLint, y: GLint) {
		(self.version_1_4.windowpos2i)(x, y)
	}
	#[inline(always)]
	fn glWindowPos2iv(&self, v: *const GLint) {
		(self.version_1_4.windowpos2iv)(v)
	}
	#[inline(always)]
	fn glWindowPos2s(&self, x: GLshort, y: GLshort) {
		(self.version_1_4.windowpos2s)(x, y)
	}
	#[inline(always)]
	fn glWindowPos2sv(&self, v: *const GLshort) {
		(self.version_1_4.windowpos2sv)(v)
	}
	#[inline(always)]
	fn glWindowPos3d(&self, x: GLdouble, y: GLdouble, z: GLdouble) {
		(self.version_1_4.windowpos3d)(x, y, z)
	}
	#[inline(always)]
	fn glWindowPos3dv(&self, v: *const GLdouble) {
		(self.version_1_4.windowpos3dv)(v)
	}
	#[inline(always)]
	fn glWindowPos3f(&self, x: GLfloat, y: GLfloat, z: GLfloat) {
		(self.version_1_4.windowpos3f)(x, y, z)
	}
	#[inline(always)]
	fn glWindowPos3fv(&self, v: *const GLfloat) {
		(self.version_1_4.windowpos3fv)(v)
	}
	#[inline(always)]
	fn glWindowPos3i(&self, x: GLint, y: GLint, z: GLint) {
		(self.version_1_4.windowpos3i)(x, y, z)
	}
	#[inline(always)]
	fn glWindowPos3iv(&self, v: *const GLint) {
		(self.version_1_4.windowpos3iv)(v)
	}
	#[inline(always)]
	fn glWindowPos3s(&self, x: GLshort, y: GLshort, z: GLshort) {
		(self.version_1_4.windowpos3s)(x, y, z)
	}
	#[inline(always)]
	fn glWindowPos3sv(&self, v: *const GLshort) {
		(self.version_1_4.windowpos3sv)(v)
	}
	#[inline(always)]
	fn glBlendColor(&self, red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
		(self.version_1_4.blendcolor)(red, green, blue, alpha)
	}
	#[inline(always)]
	fn glBlendEquation(&self, mode: GLenum) {
		(self.version_1_4.blendequation)(mode)
	}
}

impl GL_1_5 for GLCore {
	#[inline(always)]
	fn glGenQueries(&self, n: GLsizei, ids: *mut GLuint) {
		(self.version_1_5.genqueries)(n, ids)
	}
	#[inline(always)]
	fn glDeleteQueries(&self, n: GLsizei, ids: *const GLuint) {
		(self.version_1_5.deletequeries)(n, ids)
	}
	#[inline(always)]
	fn glIsQuery(&self, id: GLuint) -> GLboolean {
		(self.version_1_5.isquery)(id)
	}
	#[inline(always)]
	fn glBeginQuery(&self, target: GLenum, id: GLuint) {
		(self.version_1_5.beginquery)(target, id)
	}
	#[inline(always)]
	fn glEndQuery(&self, target: GLenum) {
		(self.version_1_5.endquery)(target)
	}
	#[inline(always)]
	fn glGetQueryiv(&self, target: GLenum, pname: GLenum, params: *mut GLint) {
		(self.version_1_5.getqueryiv)(target, pname, params)
	}
	#[inline(always)]
	fn glGetQueryObjectiv(&self, id: GLuint, pname: GLenum, params: *mut GLint) {
		(self.version_1_5.getqueryobjectiv)(id, pname, params)
	}
	#[inline(always)]
	fn glGetQueryObjectuiv(&self, id: GLuint, pname: GLenum, params: *mut GLuint) {
		(self.version_1_5.getqueryobjectuiv)(id, pname, params)
	}
	#[inline(always)]
	fn glBindBuffer(&self, target: GLenum, buffer: GLuint) {
		(self.version_1_5.bindbuffer)(target, buffer)
	}
	#[inline(always)]
	fn glDeleteBuffers(&self, n: GLsizei, buffers: *const GLuint) {
		(self.version_1_5.deletebuffers)(n, buffers)
	}
	#[inline(always)]
	fn glGenBuffers(&self, n: GLsizei, buffers: *mut GLuint) {
		(self.version_1_5.genbuffers)(n, buffers)
	}
	#[inline(always)]
	fn glIsBuffer(&self, buffer: GLuint) -> GLboolean {
		(self.version_1_5.isbuffer)(buffer)
	}
	#[inline(always)]
	fn glBufferData(&self, target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum) {
		(self.version_1_5.bufferdata)(target, size, data, usage)
	}
	#[inline(always)]
	fn glBufferSubData(&self, target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const c_void) {
		(self.version_1_5.buffersubdata)(target, offset, size, data)
	}
	#[inline(always)]
	fn glGetBufferSubData(&self, target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut c_void) {
		(self.version_1_5.getbuffersubdata)(target, offset, size, data)
	}
	#[inline(always)]
	fn glMapBuffer(&self, target: GLenum, access: GLenum) -> *mut c_void {
		(self.version_1_5.mapbuffer)(target, access)
	}
	#[inline(always)]
	fn glUnmapBuffer(&self, target: GLenum) -> GLboolean {
		(self.version_1_5.unmapbuffer)(target)
	}
	#[inline(always)]
	fn glGetBufferParameteriv(&self, target: GLenum, pname: GLenum, params: *mut GLint) {
		(self.version_1_5.getbufferparameteriv)(target, pname, params)
	}
	#[inline(always)]
	fn glGetBufferPointerv(&self, target: GLenum, pname: GLenum, params: *mut *mut c_void) {
		(self.version_1_5.getbufferpointerv)(target, pname, params)
	}
}

impl GL_2_0 for GLCore {
	#[inline(always)]
	fn glBlendEquationSeparate(&self, modeRGB: GLenum, modeAlpha: GLenum) {
		(self.version_2_0.blendequationseparate)(modeRGB, modeAlpha)
	}
	#[inline(always)]
	fn glDrawBuffers(&self, n: GLsizei, bufs: *const GLenum) {
		(self.version_2_0.drawbuffers)(n, bufs)
	}
	#[inline(always)]
	fn glStencilOpSeparate(&self, face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum) {
		(self.version_2_0.stencilopseparate)(face, sfail, dpfail, dppass)
	}
	#[inline(always)]
	fn glStencilFuncSeparate(&self, face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) {
		(self.version_2_0.stencilfuncseparate)(face, func, ref_, mask)
	}
	#[inline(always)]
	fn glStencilMaskSeparate(&self, face: GLenum, mask: GLuint) {
		(self.version_2_0.stencilmaskseparate)(face, mask)
	}
	#[inline(always)]
	fn glAttachShader(&self, program: GLuint, shader: GLuint) {
		(self.version_2_0.attachshader)(program, shader)
	}
	#[inline(always)]
	fn glBindAttribLocation(&self, program: GLuint, index: GLuint, name: *const GLchar) {
		(self.version_2_0.bindattriblocation)(program, index, name)
	}
	#[inline(always)]
	fn glCompileShader(&self, shader: GLuint) {
		(self.version_2_0.compileshader)(shader)
	}
	#[inline(always)]
	fn glCreateProgram(&self) -> GLuint {
		(self.version_2_0.createprogram)()
	}
	#[inline(always)]
	fn glCreateShader(&self, type_: GLenum) -> GLuint {
		(self.version_2_0.createshader)(type_)
	}
	#[inline(always)]
	fn glDeleteProgram(&self, program: GLuint) {
		(self.version_2_0.deleteprogram)(program)
	}
	#[inline(always)]
	fn glDeleteShader(&self, shader: GLuint) {
		(self.version_2_0.deleteshader)(shader)
	}
	#[inline(always)]
	fn glDetachShader(&self, program: GLuint, shader: GLuint) {
		(self.version_2_0.detachshader)(program, shader)
	}
	#[inline(always)]
	fn glDisableVertexAttribArray(&self, index: GLuint) {
		(self.version_2_0.disablevertexattribarray)(index)
	}
	#[inline(always)]
	fn glEnableVertexAttribArray(&self, index: GLuint) {
		(self.version_2_0.enablevertexattribarray)(index)
	}
	#[inline(always)]
	fn glGetActiveAttrib(&self, program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) {
		(self.version_2_0.getactiveattrib)(program, index, bufSize, length, size, type_, name)
	}
	#[inline(always)]
	fn glGetActiveUniform(&self, program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) {
		(self.version_2_0.getactiveuniform)(program, index, bufSize, length, size, type_, name)
	}
	#[inline(always)]
	fn glGetAttachedShaders(&self, program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint) {
		(self.version_2_0.getattachedshaders)(program, maxCount, count, shaders)
	}
	#[inline(always)]
	fn glGetAttribLocation(&self, program: GLuint, name: *const GLchar) -> GLint {
		(self.version_2_0.getattriblocation)(program, name)
	}
	#[inline(always)]
	fn glGetProgramiv(&self, program: GLuint, pname: GLenum, params: *mut GLint) {
		(self.version_2_0.getprogramiv)(program, pname, params)
	}
	#[inline(always)]
	fn glGetProgramInfoLog(&self, program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) {
		(self.version_2_0.getprograminfolog)(program, bufSize, length, infoLog)
	}
	#[inline(always)]
	fn glGetShaderiv(&self, shader: GLuint, pname: GLenum, params: *mut GLint) {
		(self.version_2_0.getshaderiv)(shader, pname, params)
	}
	#[inline(always)]
	fn glGetShaderInfoLog(&self, shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) {
		(self.version_2_0.getshaderinfolog)(shader, bufSize, length, infoLog)
	}
	#[inline(always)]
	fn glGetShaderSource(&self, shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar) {
		(self.version_2_0.getshadersource)(shader, bufSize, length, source)
	}
	#[inline(always)]
	fn glGetUniformLocation(&self, program: GLuint, name: *const GLchar) -> GLint {
		(self.version_2_0.getuniformlocation)(program, name)
	}
	#[inline(always)]
	fn glGetUniformfv(&self, program: GLuint, location: GLint, params: *mut GLfloat) {
		(self.version_2_0.getuniformfv)(program, location, params)
	}
	#[inline(always)]
	fn glGetUniformiv(&self, program: GLuint, location: GLint, params: *mut GLint) {
		(self.version_2_0.getuniformiv)(program, location, params)
	}
	#[inline(always)]
	fn glGetVertexAttribdv(&self, index: GLuint, pname: GLenum, params: *mut GLdouble) {
		(self.version_2_0.getvertexattribdv)(index, pname, params)
	}
	#[inline(always)]
	fn glGetVertexAttribfv(&self, index: GLuint, pname: GLenum, params: *mut GLfloat) {
		(self.version_2_0.getvertexattribfv)(index, pname, params)
	}
	#[inline(always)]
	fn glGetVertexAttribiv(&self, index: GLuint, pname: GLenum, params: *mut GLint) {
		(self.version_2_0.getvertexattribiv)(index, pname, params)
	}
	#[inline(always)]
	fn glGetVertexAttribPointerv(&self, index: GLuint, pname: GLenum, pointer: *mut *mut c_void) {
		(self.version_2_0.getvertexattribpointerv)(index, pname, pointer)
	}
	#[inline(always)]
	fn glIsProgram(&self, program: GLuint) -> GLboolean {
		(self.version_2_0.isprogram)(program)
	}
	#[inline(always)]
	fn glIsShader(&self, shader: GLuint) -> GLboolean {
		(self.version_2_0.isshader)(shader)
	}
	#[inline(always)]
	fn glLinkProgram(&self, program: GLuint) {
		(self.version_2_0.linkprogram)(program)
	}
	#[inline(always)]
	fn glShaderSource(&self, shader: GLuint, count: GLsizei, string_: *const *const GLchar, length: *const GLint) {
		(self.version_2_0.shadersource)(shader, count, string_, length)
	}
	#[inline(always)]
	fn glUseProgram(&self, program: GLuint) {
		(self.version_2_0.useprogram)(program)
	}
	#[inline(always)]
	fn glUniform1f(&self, location: GLint, v0: GLfloat) {
		(self.version_2_0.uniform1f)(location, v0)
	}
	#[inline(always)]
	fn glUniform2f(&self, location: GLint, v0: GLfloat, v1: GLfloat) {
		(self.version_2_0.uniform2f)(location, v0, v1)
	}
	#[inline(always)]
	fn glUniform3f(&self, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) {
		(self.version_2_0.uniform3f)(location, v0, v1, v2)
	}
	#[inline(always)]
	fn glUniform4f(&self, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) {
		(self.version_2_0.uniform4f)(location, v0, v1, v2, v3)
	}
	#[inline(always)]
	fn glUniform1i(&self, location: GLint, v0: GLint) {
		(self.version_2_0.uniform1i)(location, v0)
	}
	#[inline(always)]
	fn glUniform2i(&self, location: GLint, v0: GLint, v1: GLint) {
		(self.version_2_0.uniform2i)(location, v0, v1)
	}
	#[inline(always)]
	fn glUniform3i(&self, location: GLint, v0: GLint, v1: GLint, v2: GLint) {
		(self.version_2_0.uniform3i)(location, v0, v1, v2)
	}
	#[inline(always)]
	fn glUniform4i(&self, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) {
		(self.version_2_0.uniform4i)(location, v0, v1, v2, v3)
	}
	#[inline(always)]
	fn glUniform1fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
		(self.version_2_0.uniform1fv)(location, count, value)
	}
	#[inline(always)]
	fn glUniform2fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
		(self.version_2_0.uniform2fv)(location, count, value)
	}
	#[inline(always)]
	fn glUniform3fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
		(self.version_2_0.uniform3fv)(location, count, value)
	}
	#[inline(always)]
	fn glUniform4fv(&self, location: GLint, count: GLsizei, value: *const GLfloat) {
		(self.version_2_0.uniform4fv)(location, count, value)
	}
	#[inline(always)]
	fn glUniform1iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
		(self.version_2_0.uniform1iv)(location, count, value)
	}
	#[inline(always)]
	fn glUniform2iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
		(self.version_2_0.uniform2iv)(location, count, value)
	}
	#[inline(always)]
	fn glUniform3iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
		(self.version_2_0.uniform3iv)(location, count, value)
	}
	#[inline(always)]
	fn glUniform4iv(&self, location: GLint, count: GLsizei, value: *const GLint) {
		(self.version_2_0.uniform4iv)(location, count, value)
	}
	#[inline(always)]
	fn glUniformMatrix2fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.version_2_0.uniformmatrix2fv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glUniformMatrix3fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.version_2_0.uniformmatrix3fv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glUniformMatrix4fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.version_2_0.uniformmatrix4fv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glValidateProgram(&self, program: GLuint) {
		(self.version_2_0.validateprogram)(program)
	}
	#[inline(always)]
	fn glVertexAttrib1d(&self, index: GLuint, x: GLdouble) {
		(self.version_2_0.vertexattrib1d)(index, x)
	}
	#[inline(always)]
	fn glVertexAttrib1dv(&self, index: GLuint, v: *const GLdouble) {
		(self.version_2_0.vertexattrib1dv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib1f(&self, index: GLuint, x: GLfloat) {
		(self.version_2_0.vertexattrib1f)(index, x)
	}
	#[inline(always)]
	fn glVertexAttrib1fv(&self, index: GLuint, v: *const GLfloat) {
		(self.version_2_0.vertexattrib1fv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib1s(&self, index: GLuint, x: GLshort) {
		(self.version_2_0.vertexattrib1s)(index, x)
	}
	#[inline(always)]
	fn glVertexAttrib1sv(&self, index: GLuint, v: *const GLshort) {
		(self.version_2_0.vertexattrib1sv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib2d(&self, index: GLuint, x: GLdouble, y: GLdouble) {
		(self.version_2_0.vertexattrib2d)(index, x, y)
	}
	#[inline(always)]
	fn glVertexAttrib2dv(&self, index: GLuint, v: *const GLdouble) {
		(self.version_2_0.vertexattrib2dv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib2f(&self, index: GLuint, x: GLfloat, y: GLfloat) {
		(self.version_2_0.vertexattrib2f)(index, x, y)
	}
	#[inline(always)]
	fn glVertexAttrib2fv(&self, index: GLuint, v: *const GLfloat) {
		(self.version_2_0.vertexattrib2fv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib2s(&self, index: GLuint, x: GLshort, y: GLshort) {
		(self.version_2_0.vertexattrib2s)(index, x, y)
	}
	#[inline(always)]
	fn glVertexAttrib2sv(&self, index: GLuint, v: *const GLshort) {
		(self.version_2_0.vertexattrib2sv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib3d(&self, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) {
		(self.version_2_0.vertexattrib3d)(index, x, y, z)
	}
	#[inline(always)]
	fn glVertexAttrib3dv(&self, index: GLuint, v: *const GLdouble) {
		(self.version_2_0.vertexattrib3dv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib3f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) {
		(self.version_2_0.vertexattrib3f)(index, x, y, z)
	}
	#[inline(always)]
	fn glVertexAttrib3fv(&self, index: GLuint, v: *const GLfloat) {
		(self.version_2_0.vertexattrib3fv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib3s(&self, index: GLuint, x: GLshort, y: GLshort, z: GLshort) {
		(self.version_2_0.vertexattrib3s)(index, x, y, z)
	}
	#[inline(always)]
	fn glVertexAttrib3sv(&self, index: GLuint, v: *const GLshort) {
		(self.version_2_0.vertexattrib3sv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib4Nbv(&self, index: GLuint, v: *const GLbyte) {
		(self.version_2_0.vertexattrib4nbv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib4Niv(&self, index: GLuint, v: *const GLint) {
		(self.version_2_0.vertexattrib4niv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib4Nsv(&self, index: GLuint, v: *const GLshort) {
		(self.version_2_0.vertexattrib4nsv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib4Nub(&self, index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte) {
		(self.version_2_0.vertexattrib4nub)(index, x, y, z, w)
	}
	#[inline(always)]
	fn glVertexAttrib4Nubv(&self, index: GLuint, v: *const GLubyte) {
		(self.version_2_0.vertexattrib4nubv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib4Nuiv(&self, index: GLuint, v: *const GLuint) {
		(self.version_2_0.vertexattrib4nuiv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib4Nusv(&self, index: GLuint, v: *const GLushort) {
		(self.version_2_0.vertexattrib4nusv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib4bv(&self, index: GLuint, v: *const GLbyte) {
		(self.version_2_0.vertexattrib4bv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib4d(&self, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) {
		(self.version_2_0.vertexattrib4d)(index, x, y, z, w)
	}
	#[inline(always)]
	fn glVertexAttrib4dv(&self, index: GLuint, v: *const GLdouble) {
		(self.version_2_0.vertexattrib4dv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib4f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
		(self.version_2_0.vertexattrib4f)(index, x, y, z, w)
	}
	#[inline(always)]
	fn glVertexAttrib4fv(&self, index: GLuint, v: *const GLfloat) {
		(self.version_2_0.vertexattrib4fv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib4iv(&self, index: GLuint, v: *const GLint) {
		(self.version_2_0.vertexattrib4iv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib4s(&self, index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort) {
		(self.version_2_0.vertexattrib4s)(index, x, y, z, w)
	}
	#[inline(always)]
	fn glVertexAttrib4sv(&self, index: GLuint, v: *const GLshort) {
		(self.version_2_0.vertexattrib4sv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib4ubv(&self, index: GLuint, v: *const GLubyte) {
		(self.version_2_0.vertexattrib4ubv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib4uiv(&self, index: GLuint, v: *const GLuint) {
		(self.version_2_0.vertexattrib4uiv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttrib4usv(&self, index: GLuint, v: *const GLushort) {
		(self.version_2_0.vertexattrib4usv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribPointer(&self, index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void) {
		(self.version_2_0.vertexattribpointer)(index, size, type_, normalized, stride, pointer)
	}
	#[inline(always)]
	fn get_shading_language_version(&self) -> &'static str {
		self.version_2_0.shading_language_version
	}
}

impl GL_2_1 for GLCore {
	#[inline(always)]
	fn glUniformMatrix2x3fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.version_2_1.uniformmatrix2x3fv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glUniformMatrix3x2fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.version_2_1.uniformmatrix3x2fv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glUniformMatrix2x4fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.version_2_1.uniformmatrix2x4fv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glUniformMatrix4x2fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.version_2_1.uniformmatrix4x2fv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glUniformMatrix3x4fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.version_2_1.uniformmatrix3x4fv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glUniformMatrix4x3fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.version_2_1.uniformmatrix4x3fv)(location, count, transpose, value)
	}
}

impl GL_3_0 for GLCore {
	#[inline(always)]
	fn glColorMaski(&self, index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean) {
		(self.version_3_0.colormaski)(index, r, g, b, a)
	}
	#[inline(always)]
	fn glGetBooleani_v(&self, target: GLenum, index: GLuint, data: *mut GLboolean) {
		(self.version_3_0.getbooleani_v)(target, index, data)
	}
	#[inline(always)]
	fn glGetIntegeri_v(&self, target: GLenum, index: GLuint, data: *mut GLint) {
		(self.version_3_0.getintegeri_v)(target, index, data)
	}
	#[inline(always)]
	fn glEnablei(&self, target: GLenum, index: GLuint) {
		(self.version_3_0.enablei)(target, index)
	}
	#[inline(always)]
	fn glDisablei(&self, target: GLenum, index: GLuint) {
		(self.version_3_0.disablei)(target, index)
	}
	#[inline(always)]
	fn glIsEnabledi(&self, target: GLenum, index: GLuint) -> GLboolean {
		(self.version_3_0.isenabledi)(target, index)
	}
	#[inline(always)]
	fn glBeginTransformFeedback(&self, primitiveMode: GLenum) {
		(self.version_3_0.begintransformfeedback)(primitiveMode)
	}
	#[inline(always)]
	fn glEndTransformFeedback(&self) {
		(self.version_3_0.endtransformfeedback)()
	}
	#[inline(always)]
	fn glBindBufferRange(&self, target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) {
		(self.version_3_0.bindbufferrange)(target, index, buffer, offset, size)
	}
	#[inline(always)]
	fn glBindBufferBase(&self, target: GLenum, index: GLuint, buffer: GLuint) {
		(self.version_3_0.bindbufferbase)(target, index, buffer)
	}
	#[inline(always)]
	fn glTransformFeedbackVaryings(&self, program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: GLenum) {
		(self.version_3_0.transformfeedbackvaryings)(program, count, varyings, bufferMode)
	}
	#[inline(always)]
	fn glGetTransformFeedbackVarying(&self, program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut GLenum, name: *mut GLchar) {
		(self.version_3_0.gettransformfeedbackvarying)(program, index, bufSize, length, size, type_, name)
	}
	#[inline(always)]
	fn glClampColor(&self, target: GLenum, clamp: GLenum) {
		(self.version_3_0.clampcolor)(target, clamp)
	}
	#[inline(always)]
	fn glBeginConditionalRender(&self, id: GLuint, mode: GLenum) {
		(self.version_3_0.beginconditionalrender)(id, mode)
	}
	#[inline(always)]
	fn glEndConditionalRender(&self) {
		(self.version_3_0.endconditionalrender)()
	}
	#[inline(always)]
	fn glVertexAttribIPointer(&self, index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void) {
		(self.version_3_0.vertexattribipointer)(index, size, type_, stride, pointer)
	}
	#[inline(always)]
	fn glGetVertexAttribIiv(&self, index: GLuint, pname: GLenum, params: *mut GLint) {
		(self.version_3_0.getvertexattribiiv)(index, pname, params)
	}
	#[inline(always)]
	fn glGetVertexAttribIuiv(&self, index: GLuint, pname: GLenum, params: *mut GLuint) {
		(self.version_3_0.getvertexattribiuiv)(index, pname, params)
	}
	#[inline(always)]
	fn glVertexAttribI1i(&self, index: GLuint, x: GLint) {
		(self.version_3_0.vertexattribi1i)(index, x)
	}
	#[inline(always)]
	fn glVertexAttribI2i(&self, index: GLuint, x: GLint, y: GLint) {
		(self.version_3_0.vertexattribi2i)(index, x, y)
	}
	#[inline(always)]
	fn glVertexAttribI3i(&self, index: GLuint, x: GLint, y: GLint, z: GLint) {
		(self.version_3_0.vertexattribi3i)(index, x, y, z)
	}
	#[inline(always)]
	fn glVertexAttribI4i(&self, index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) {
		(self.version_3_0.vertexattribi4i)(index, x, y, z, w)
	}
	#[inline(always)]
	fn glVertexAttribI1ui(&self, index: GLuint, x: GLuint) {
		(self.version_3_0.vertexattribi1ui)(index, x)
	}
	#[inline(always)]
	fn glVertexAttribI2ui(&self, index: GLuint, x: GLuint, y: GLuint) {
		(self.version_3_0.vertexattribi2ui)(index, x, y)
	}
	#[inline(always)]
	fn glVertexAttribI3ui(&self, index: GLuint, x: GLuint, y: GLuint, z: GLuint) {
		(self.version_3_0.vertexattribi3ui)(index, x, y, z)
	}
	#[inline(always)]
	fn glVertexAttribI4ui(&self, index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) {
		(self.version_3_0.vertexattribi4ui)(index, x, y, z, w)
	}
	#[inline(always)]
	fn glVertexAttribI1iv(&self, index: GLuint, v: *const GLint) {
		(self.version_3_0.vertexattribi1iv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribI2iv(&self, index: GLuint, v: *const GLint) {
		(self.version_3_0.vertexattribi2iv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribI3iv(&self, index: GLuint, v: *const GLint) {
		(self.version_3_0.vertexattribi3iv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribI4iv(&self, index: GLuint, v: *const GLint) {
		(self.version_3_0.vertexattribi4iv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribI1uiv(&self, index: GLuint, v: *const GLuint) {
		(self.version_3_0.vertexattribi1uiv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribI2uiv(&self, index: GLuint, v: *const GLuint) {
		(self.version_3_0.vertexattribi2uiv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribI3uiv(&self, index: GLuint, v: *const GLuint) {
		(self.version_3_0.vertexattribi3uiv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribI4uiv(&self, index: GLuint, v: *const GLuint) {
		(self.version_3_0.vertexattribi4uiv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribI4bv(&self, index: GLuint, v: *const GLbyte) {
		(self.version_3_0.vertexattribi4bv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribI4sv(&self, index: GLuint, v: *const GLshort) {
		(self.version_3_0.vertexattribi4sv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribI4ubv(&self, index: GLuint, v: *const GLubyte) {
		(self.version_3_0.vertexattribi4ubv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribI4usv(&self, index: GLuint, v: *const GLushort) {
		(self.version_3_0.vertexattribi4usv)(index, v)
	}
	#[inline(always)]
	fn glGetUniformuiv(&self, program: GLuint, location: GLint, params: *mut GLuint) {
		(self.version_3_0.getuniformuiv)(program, location, params)
	}
	#[inline(always)]
	fn glBindFragDataLocation(&self, program: GLuint, color: GLuint, name: *const GLchar) {
		(self.version_3_0.bindfragdatalocation)(program, color, name)
	}
	#[inline(always)]
	fn glGetFragDataLocation(&self, program: GLuint, name: *const GLchar) -> GLint {
		(self.version_3_0.getfragdatalocation)(program, name)
	}
	#[inline(always)]
	fn glUniform1ui(&self, location: GLint, v0: GLuint) {
		(self.version_3_0.uniform1ui)(location, v0)
	}
	#[inline(always)]
	fn glUniform2ui(&self, location: GLint, v0: GLuint, v1: GLuint) {
		(self.version_3_0.uniform2ui)(location, v0, v1)
	}
	#[inline(always)]
	fn glUniform3ui(&self, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) {
		(self.version_3_0.uniform3ui)(location, v0, v1, v2)
	}
	#[inline(always)]
	fn glUniform4ui(&self, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) {
		(self.version_3_0.uniform4ui)(location, v0, v1, v2, v3)
	}
	#[inline(always)]
	fn glUniform1uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
		(self.version_3_0.uniform1uiv)(location, count, value)
	}
	#[inline(always)]
	fn glUniform2uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
		(self.version_3_0.uniform2uiv)(location, count, value)
	}
	#[inline(always)]
	fn glUniform3uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
		(self.version_3_0.uniform3uiv)(location, count, value)
	}
	#[inline(always)]
	fn glUniform4uiv(&self, location: GLint, count: GLsizei, value: *const GLuint) {
		(self.version_3_0.uniform4uiv)(location, count, value)
	}
	#[inline(always)]
	fn glTexParameterIiv(&self, target: GLenum, pname: GLenum, params: *const GLint) {
		(self.version_3_0.texparameteriiv)(target, pname, params)
	}
	#[inline(always)]
	fn glTexParameterIuiv(&self, target: GLenum, pname: GLenum, params: *const GLuint) {
		(self.version_3_0.texparameteriuiv)(target, pname, params)
	}
	#[inline(always)]
	fn glGetTexParameterIiv(&self, target: GLenum, pname: GLenum, params: *mut GLint) {
		(self.version_3_0.gettexparameteriiv)(target, pname, params)
	}
	#[inline(always)]
	fn glGetTexParameterIuiv(&self, target: GLenum, pname: GLenum, params: *mut GLuint) {
		(self.version_3_0.gettexparameteriuiv)(target, pname, params)
	}
	#[inline(always)]
	fn glClearBufferiv(&self, buffer: GLenum, drawbuffer: GLint, value: *const GLint) {
		(self.version_3_0.clearbufferiv)(buffer, drawbuffer, value)
	}
	#[inline(always)]
	fn glClearBufferuiv(&self, buffer: GLenum, drawbuffer: GLint, value: *const GLuint) {
		(self.version_3_0.clearbufferuiv)(buffer, drawbuffer, value)
	}
	#[inline(always)]
	fn glClearBufferfv(&self, buffer: GLenum, drawbuffer: GLint, value: *const GLfloat) {
		(self.version_3_0.clearbufferfv)(buffer, drawbuffer, value)
	}
	#[inline(always)]
	fn glClearBufferfi(&self, buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint) {
		(self.version_3_0.clearbufferfi)(buffer, drawbuffer, depth, stencil)
	}
	#[inline(always)]
	fn glGetStringi(&self, name: GLenum, index: GLuint) -> *const GLubyte {
		(self.version_3_0.getstringi)(name, index)
	}
	#[inline(always)]
	fn glIsRenderbuffer(&self, renderbuffer: GLuint) -> GLboolean {
		(self.version_3_0.isrenderbuffer)(renderbuffer)
	}
	#[inline(always)]
	fn glBindRenderbuffer(&self, target: GLenum, renderbuffer: GLuint) {
		(self.version_3_0.bindrenderbuffer)(target, renderbuffer)
	}
	#[inline(always)]
	fn glDeleteRenderbuffers(&self, n: GLsizei, renderbuffers: *const GLuint) {
		(self.version_3_0.deleterenderbuffers)(n, renderbuffers)
	}
	#[inline(always)]
	fn glGenRenderbuffers(&self, n: GLsizei, renderbuffers: *mut GLuint) {
		(self.version_3_0.genrenderbuffers)(n, renderbuffers)
	}
	#[inline(always)]
	fn glRenderbufferStorage(&self, target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei) {
		(self.version_3_0.renderbufferstorage)(target, internalformat, width, height)
	}
	#[inline(always)]
	fn glGetRenderbufferParameteriv(&self, target: GLenum, pname: GLenum, params: *mut GLint) {
		(self.version_3_0.getrenderbufferparameteriv)(target, pname, params)
	}
	#[inline(always)]
	fn glIsFramebuffer(&self, framebuffer: GLuint) -> GLboolean {
		(self.version_3_0.isframebuffer)(framebuffer)
	}
	#[inline(always)]
	fn glBindFramebuffer(&self, target: GLenum, framebuffer: GLuint) {
		(self.version_3_0.bindframebuffer)(target, framebuffer)
	}
	#[inline(always)]
	fn glDeleteFramebuffers(&self, n: GLsizei, framebuffers: *const GLuint) {
		(self.version_3_0.deleteframebuffers)(n, framebuffers)
	}
	#[inline(always)]
	fn glGenFramebuffers(&self, n: GLsizei, framebuffers: *mut GLuint) {
		(self.version_3_0.genframebuffers)(n, framebuffers)
	}
	#[inline(always)]
	fn glCheckFramebufferStatus(&self, target: GLenum) -> GLenum {
		(self.version_3_0.checkframebufferstatus)(target)
	}
	#[inline(always)]
	fn glFramebufferTexture1D(&self, target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) {
		(self.version_3_0.framebuffertexture1d)(target, attachment, textarget, texture, level)
	}
	#[inline(always)]
	fn glFramebufferTexture2D(&self, target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) {
		(self.version_3_0.framebuffertexture2d)(target, attachment, textarget, texture, level)
	}
	#[inline(always)]
	fn glFramebufferTexture3D(&self, target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint) {
		(self.version_3_0.framebuffertexture3d)(target, attachment, textarget, texture, level, zoffset)
	}
	#[inline(always)]
	fn glFramebufferRenderbuffer(&self, target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint) {
		(self.version_3_0.framebufferrenderbuffer)(target, attachment, renderbuffertarget, renderbuffer)
	}
	#[inline(always)]
	fn glGetFramebufferAttachmentParameteriv(&self, target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint) {
		(self.version_3_0.getframebufferattachmentparameteriv)(target, attachment, pname, params)
	}
	#[inline(always)]
	fn glGenerateMipmap(&self, target: GLenum) {
		(self.version_3_0.generatemipmap)(target)
	}
	#[inline(always)]
	fn glBlitFramebuffer(&self, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum) {
		(self.version_3_0.blitframebuffer)(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter)
	}
	#[inline(always)]
	fn glRenderbufferStorageMultisample(&self, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) {
		(self.version_3_0.renderbufferstoragemultisample)(target, samples, internalformat, width, height)
	}
	#[inline(always)]
	fn glFramebufferTextureLayer(&self, target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint) {
		(self.version_3_0.framebuffertexturelayer)(target, attachment, texture, level, layer)
	}
	#[inline(always)]
	fn glMapBufferRange(&self, target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut c_void {
		(self.version_3_0.mapbufferrange)(target, offset, length, access)
	}
	#[inline(always)]
	fn glFlushMappedBufferRange(&self, target: GLenum, offset: GLintptr, length: GLsizeiptr) {
		(self.version_3_0.flushmappedbufferrange)(target, offset, length)
	}
	#[inline(always)]
	fn glBindVertexArray(&self, array: GLuint) {
		(self.version_3_0.bindvertexarray)(array)
	}
	#[inline(always)]
	fn glDeleteVertexArrays(&self, n: GLsizei, arrays: *const GLuint) {
		(self.version_3_0.deletevertexarrays)(n, arrays)
	}
	#[inline(always)]
	fn glGenVertexArrays(&self, n: GLsizei, arrays: *mut GLuint) {
		(self.version_3_0.genvertexarrays)(n, arrays)
	}
	#[inline(always)]
	fn glIsVertexArray(&self, array: GLuint) -> GLboolean {
		(self.version_3_0.isvertexarray)(array)
	}
}

impl GL_3_1 for GLCore {
	#[inline(always)]
	fn glDrawArraysInstanced(&self, mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei) {
		(self.version_3_1.drawarraysinstanced)(mode, first, count, instancecount)
	}
	#[inline(always)]
	fn glDrawElementsInstanced(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei) {
		(self.version_3_1.drawelementsinstanced)(mode, count, type_, indices, instancecount)
	}
	#[inline(always)]
	fn glTexBuffer(&self, target: GLenum, internalformat: GLenum, buffer: GLuint) {
		(self.version_3_1.texbuffer)(target, internalformat, buffer)
	}
	#[inline(always)]
	fn glPrimitiveRestartIndex(&self, index: GLuint) {
		(self.version_3_1.primitiverestartindex)(index)
	}
	#[inline(always)]
	fn glCopyBufferSubData(&self, readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr) {
		(self.version_3_1.copybuffersubdata)(readTarget, writeTarget, readOffset, writeOffset, size)
	}
	#[inline(always)]
	fn glGetUniformIndices(&self, program: GLuint, uniformCount: GLsizei, uniformNames: *const *const GLchar, uniformIndices: *mut GLuint) {
		(self.version_3_1.getuniformindices)(program, uniformCount, uniformNames, uniformIndices)
	}
	#[inline(always)]
	fn glGetActiveUniformsiv(&self, program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint, pname: GLenum, params: *mut GLint) {
		(self.version_3_1.getactiveuniformsiv)(program, uniformCount, uniformIndices, pname, params)
	}
	#[inline(always)]
	fn glGetActiveUniformName(&self, program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar) {
		(self.version_3_1.getactiveuniformname)(program, uniformIndex, bufSize, length, uniformName)
	}
	#[inline(always)]
	fn glGetUniformBlockIndex(&self, program: GLuint, uniformBlockName: *const GLchar) -> GLuint {
		(self.version_3_1.getuniformblockindex)(program, uniformBlockName)
	}
	#[inline(always)]
	fn glGetActiveUniformBlockiv(&self, program: GLuint, uniformBlockIndex: GLuint, pname: GLenum, params: *mut GLint) {
		(self.version_3_1.getactiveuniformblockiv)(program, uniformBlockIndex, pname, params)
	}
	#[inline(always)]
	fn glGetActiveUniformBlockName(&self, program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar) {
		(self.version_3_1.getactiveuniformblockname)(program, uniformBlockIndex, bufSize, length, uniformBlockName)
	}
	#[inline(always)]
	fn glUniformBlockBinding(&self, program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint) {
		(self.version_3_1.uniformblockbinding)(program, uniformBlockIndex, uniformBlockBinding)
	}
}

impl GL_3_2 for GLCore {
	#[inline(always)]
	fn glDrawElementsBaseVertex(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, basevertex: GLint) {
		(self.version_3_2.drawelementsbasevertex)(mode, count, type_, indices, basevertex)
	}
	#[inline(always)]
	fn glDrawRangeElementsBaseVertex(&self, mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const c_void, basevertex: GLint) {
		(self.version_3_2.drawrangeelementsbasevertex)(mode, start, end, count, type_, indices, basevertex)
	}
	#[inline(always)]
	fn glDrawElementsInstancedBaseVertex(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei, basevertex: GLint) {
		(self.version_3_2.drawelementsinstancedbasevertex)(mode, count, type_, indices, instancecount, basevertex)
	}
	#[inline(always)]
	fn glMultiDrawElementsBaseVertex(&self, mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const c_void, drawcount: GLsizei, basevertex: *const GLint) {
		(self.version_3_2.multidrawelementsbasevertex)(mode, count, type_, indices, drawcount, basevertex)
	}
	#[inline(always)]
	fn glProvokingVertex(&self, mode: GLenum) {
		(self.version_3_2.provokingvertex)(mode)
	}
	#[inline(always)]
	fn glFenceSync(&self, condition: GLenum, flags: GLbitfield) -> GLsync {
		(self.version_3_2.fencesync)(condition, flags)
	}
	#[inline(always)]
	fn glIsSync(&self, sync: GLsync) -> GLboolean {
		(self.version_3_2.issync)(sync)
	}
	#[inline(always)]
	fn glDeleteSync(&self, sync: GLsync) {
		(self.version_3_2.deletesync)(sync)
	}
	#[inline(always)]
	fn glClientWaitSync(&self, sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum {
		(self.version_3_2.clientwaitsync)(sync, flags, timeout)
	}
	#[inline(always)]
	fn glWaitSync(&self, sync: GLsync, flags: GLbitfield, timeout: GLuint64) {
		(self.version_3_2.waitsync)(sync, flags, timeout)
	}
	#[inline(always)]
	fn glGetInteger64v(&self, pname: GLenum, data: *mut GLint64) {
		(self.version_3_2.getinteger64v)(pname, data)
	}
	#[inline(always)]
	fn glGetSynciv(&self, sync: GLsync, pname: GLenum, count: GLsizei, length: *mut GLsizei, values: *mut GLint) {
		(self.version_3_2.getsynciv)(sync, pname, count, length, values)
	}
	#[inline(always)]
	fn glGetInteger64i_v(&self, target: GLenum, index: GLuint, data: *mut GLint64) {
		(self.version_3_2.getinteger64i_v)(target, index, data)
	}
	#[inline(always)]
	fn glGetBufferParameteri64v(&self, target: GLenum, pname: GLenum, params: *mut GLint64) {
		(self.version_3_2.getbufferparameteri64v)(target, pname, params)
	}
	#[inline(always)]
	fn glFramebufferTexture(&self, target: GLenum, attachment: GLenum, texture: GLuint, level: GLint) {
		(self.version_3_2.framebuffertexture)(target, attachment, texture, level)
	}
	#[inline(always)]
	fn glTexImage2DMultisample(&self, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) {
		(self.version_3_2.teximage2dmultisample)(target, samples, internalformat, width, height, fixedsamplelocations)
	}
	#[inline(always)]
	fn glTexImage3DMultisample(&self, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) {
		(self.version_3_2.teximage3dmultisample)(target, samples, internalformat, width, height, depth, fixedsamplelocations)
	}
	#[inline(always)]
	fn glGetMultisamplefv(&self, pname: GLenum, index: GLuint, val: *mut GLfloat) {
		(self.version_3_2.getmultisamplefv)(pname, index, val)
	}
	#[inline(always)]
	fn glSampleMaski(&self, maskNumber: GLuint, mask: GLbitfield) {
		(self.version_3_2.samplemaski)(maskNumber, mask)
	}
}

impl GL_3_3 for GLCore {
	#[inline(always)]
	fn glBindFragDataLocationIndexed(&self, program: GLuint, colorNumber: GLuint, index: GLuint, name: *const GLchar) {
		(self.version_3_3.bindfragdatalocationindexed)(program, colorNumber, index, name)
	}
	#[inline(always)]
	fn glGetFragDataIndex(&self, program: GLuint, name: *const GLchar) -> GLint {
		(self.version_3_3.getfragdataindex)(program, name)
	}
	#[inline(always)]
	fn glGenSamplers(&self, count: GLsizei, samplers: *mut GLuint) {
		(self.version_3_3.gensamplers)(count, samplers)
	}
	#[inline(always)]
	fn glDeleteSamplers(&self, count: GLsizei, samplers: *const GLuint) {
		(self.version_3_3.deletesamplers)(count, samplers)
	}
	#[inline(always)]
	fn glIsSampler(&self, sampler: GLuint) -> GLboolean {
		(self.version_3_3.issampler)(sampler)
	}
	#[inline(always)]
	fn glBindSampler(&self, unit: GLuint, sampler: GLuint) {
		(self.version_3_3.bindsampler)(unit, sampler)
	}
	#[inline(always)]
	fn glSamplerParameteri(&self, sampler: GLuint, pname: GLenum, param: GLint) {
		(self.version_3_3.samplerparameteri)(sampler, pname, param)
	}
	#[inline(always)]
	fn glSamplerParameteriv(&self, sampler: GLuint, pname: GLenum, param: *const GLint) {
		(self.version_3_3.samplerparameteriv)(sampler, pname, param)
	}
	#[inline(always)]
	fn glSamplerParameterf(&self, sampler: GLuint, pname: GLenum, param: GLfloat) {
		(self.version_3_3.samplerparameterf)(sampler, pname, param)
	}
	#[inline(always)]
	fn glSamplerParameterfv(&self, sampler: GLuint, pname: GLenum, param: *const GLfloat) {
		(self.version_3_3.samplerparameterfv)(sampler, pname, param)
	}
	#[inline(always)]
	fn glSamplerParameterIiv(&self, sampler: GLuint, pname: GLenum, param: *const GLint) {
		(self.version_3_3.samplerparameteriiv)(sampler, pname, param)
	}
	#[inline(always)]
	fn glSamplerParameterIuiv(&self, sampler: GLuint, pname: GLenum, param: *const GLuint) {
		(self.version_3_3.samplerparameteriuiv)(sampler, pname, param)
	}
	#[inline(always)]
	fn glGetSamplerParameteriv(&self, sampler: GLuint, pname: GLenum, params: *mut GLint) {
		(self.version_3_3.getsamplerparameteriv)(sampler, pname, params)
	}
	#[inline(always)]
	fn glGetSamplerParameterIiv(&self, sampler: GLuint, pname: GLenum, params: *mut GLint) {
		(self.version_3_3.getsamplerparameteriiv)(sampler, pname, params)
	}
	#[inline(always)]
	fn glGetSamplerParameterfv(&self, sampler: GLuint, pname: GLenum, params: *mut GLfloat) {
		(self.version_3_3.getsamplerparameterfv)(sampler, pname, params)
	}
	#[inline(always)]
	fn glGetSamplerParameterIuiv(&self, sampler: GLuint, pname: GLenum, params: *mut GLuint) {
		(self.version_3_3.getsamplerparameteriuiv)(sampler, pname, params)
	}
	#[inline(always)]
	fn glQueryCounter(&self, id: GLuint, target: GLenum) {
		(self.version_3_3.querycounter)(id, target)
	}
	#[inline(always)]
	fn glGetQueryObjecti64v(&self, id: GLuint, pname: GLenum, params: *mut GLint64) {
		(self.version_3_3.getqueryobjecti64v)(id, pname, params)
	}
	#[inline(always)]
	fn glGetQueryObjectui64v(&self, id: GLuint, pname: GLenum, params: *mut GLuint64) {
		(self.version_3_3.getqueryobjectui64v)(id, pname, params)
	}
	#[inline(always)]
	fn glVertexAttribDivisor(&self, index: GLuint, divisor: GLuint) {
		(self.version_3_3.vertexattribdivisor)(index, divisor)
	}
	#[inline(always)]
	fn glVertexAttribP1ui(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) {
		(self.version_3_3.vertexattribp1ui)(index, type_, normalized, value)
	}
	#[inline(always)]
	fn glVertexAttribP1uiv(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) {
		(self.version_3_3.vertexattribp1uiv)(index, type_, normalized, value)
	}
	#[inline(always)]
	fn glVertexAttribP2ui(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) {
		(self.version_3_3.vertexattribp2ui)(index, type_, normalized, value)
	}
	#[inline(always)]
	fn glVertexAttribP2uiv(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) {
		(self.version_3_3.vertexattribp2uiv)(index, type_, normalized, value)
	}
	#[inline(always)]
	fn glVertexAttribP3ui(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) {
		(self.version_3_3.vertexattribp3ui)(index, type_, normalized, value)
	}
	#[inline(always)]
	fn glVertexAttribP3uiv(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) {
		(self.version_3_3.vertexattribp3uiv)(index, type_, normalized, value)
	}
	#[inline(always)]
	fn glVertexAttribP4ui(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) {
		(self.version_3_3.vertexattribp4ui)(index, type_, normalized, value)
	}
	#[inline(always)]
	fn glVertexAttribP4uiv(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) {
		(self.version_3_3.vertexattribp4uiv)(index, type_, normalized, value)
	}
	#[inline(always)]
	fn glVertexP2ui(&self, type_: GLenum, value: GLuint) {
		(self.version_3_3.vertexp2ui)(type_, value)
	}
	#[inline(always)]
	fn glVertexP2uiv(&self, type_: GLenum, value: *const GLuint) {
		(self.version_3_3.vertexp2uiv)(type_, value)
	}
	#[inline(always)]
	fn glVertexP3ui(&self, type_: GLenum, value: GLuint) {
		(self.version_3_3.vertexp3ui)(type_, value)
	}
	#[inline(always)]
	fn glVertexP3uiv(&self, type_: GLenum, value: *const GLuint) {
		(self.version_3_3.vertexp3uiv)(type_, value)
	}
	#[inline(always)]
	fn glVertexP4ui(&self, type_: GLenum, value: GLuint) {
		(self.version_3_3.vertexp4ui)(type_, value)
	}
	#[inline(always)]
	fn glVertexP4uiv(&self, type_: GLenum, value: *const GLuint) {
		(self.version_3_3.vertexp4uiv)(type_, value)
	}
	#[inline(always)]
	fn glTexCoordP1ui(&self, type_: GLenum, coords: GLuint) {
		(self.version_3_3.texcoordp1ui)(type_, coords)
	}
	#[inline(always)]
	fn glTexCoordP1uiv(&self, type_: GLenum, coords: *const GLuint) {
		(self.version_3_3.texcoordp1uiv)(type_, coords)
	}
	#[inline(always)]
	fn glTexCoordP2ui(&self, type_: GLenum, coords: GLuint) {
		(self.version_3_3.texcoordp2ui)(type_, coords)
	}
	#[inline(always)]
	fn glTexCoordP2uiv(&self, type_: GLenum, coords: *const GLuint) {
		(self.version_3_3.texcoordp2uiv)(type_, coords)
	}
	#[inline(always)]
	fn glTexCoordP3ui(&self, type_: GLenum, coords: GLuint) {
		(self.version_3_3.texcoordp3ui)(type_, coords)
	}
	#[inline(always)]
	fn glTexCoordP3uiv(&self, type_: GLenum, coords: *const GLuint) {
		(self.version_3_3.texcoordp3uiv)(type_, coords)
	}
	#[inline(always)]
	fn glTexCoordP4ui(&self, type_: GLenum, coords: GLuint) {
		(self.version_3_3.texcoordp4ui)(type_, coords)
	}
	#[inline(always)]
	fn glTexCoordP4uiv(&self, type_: GLenum, coords: *const GLuint) {
		(self.version_3_3.texcoordp4uiv)(type_, coords)
	}
	#[inline(always)]
	fn glMultiTexCoordP1ui(&self, texture: GLenum, type_: GLenum, coords: GLuint) {
		(self.version_3_3.multitexcoordp1ui)(texture, type_, coords)
	}
	#[inline(always)]
	fn glMultiTexCoordP1uiv(&self, texture: GLenum, type_: GLenum, coords: *const GLuint) {
		(self.version_3_3.multitexcoordp1uiv)(texture, type_, coords)
	}
	#[inline(always)]
	fn glMultiTexCoordP2ui(&self, texture: GLenum, type_: GLenum, coords: GLuint) {
		(self.version_3_3.multitexcoordp2ui)(texture, type_, coords)
	}
	#[inline(always)]
	fn glMultiTexCoordP2uiv(&self, texture: GLenum, type_: GLenum, coords: *const GLuint) {
		(self.version_3_3.multitexcoordp2uiv)(texture, type_, coords)
	}
	#[inline(always)]
	fn glMultiTexCoordP3ui(&self, texture: GLenum, type_: GLenum, coords: GLuint) {
		(self.version_3_3.multitexcoordp3ui)(texture, type_, coords)
	}
	#[inline(always)]
	fn glMultiTexCoordP3uiv(&self, texture: GLenum, type_: GLenum, coords: *const GLuint) {
		(self.version_3_3.multitexcoordp3uiv)(texture, type_, coords)
	}
	#[inline(always)]
	fn glMultiTexCoordP4ui(&self, texture: GLenum, type_: GLenum, coords: GLuint) {
		(self.version_3_3.multitexcoordp4ui)(texture, type_, coords)
	}
	#[inline(always)]
	fn glMultiTexCoordP4uiv(&self, texture: GLenum, type_: GLenum, coords: *const GLuint) {
		(self.version_3_3.multitexcoordp4uiv)(texture, type_, coords)
	}
	#[inline(always)]
	fn glNormalP3ui(&self, type_: GLenum, coords: GLuint) {
		(self.version_3_3.normalp3ui)(type_, coords)
	}
	#[inline(always)]
	fn glNormalP3uiv(&self, type_: GLenum, coords: *const GLuint) {
		(self.version_3_3.normalp3uiv)(type_, coords)
	}
	#[inline(always)]
	fn glColorP3ui(&self, type_: GLenum, color: GLuint) {
		(self.version_3_3.colorp3ui)(type_, color)
	}
	#[inline(always)]
	fn glColorP3uiv(&self, type_: GLenum, color: *const GLuint) {
		(self.version_3_3.colorp3uiv)(type_, color)
	}
	#[inline(always)]
	fn glColorP4ui(&self, type_: GLenum, color: GLuint) {
		(self.version_3_3.colorp4ui)(type_, color)
	}
	#[inline(always)]
	fn glColorP4uiv(&self, type_: GLenum, color: *const GLuint) {
		(self.version_3_3.colorp4uiv)(type_, color)
	}
	#[inline(always)]
	fn glSecondaryColorP3ui(&self, type_: GLenum, color: GLuint) {
		(self.version_3_3.secondarycolorp3ui)(type_, color)
	}
	#[inline(always)]
	fn glSecondaryColorP3uiv(&self, type_: GLenum, color: *const GLuint) {
		(self.version_3_3.secondarycolorp3uiv)(type_, color)
	}
}

impl GL_4_0 for GLCore {
	#[inline(always)]
	fn glMinSampleShading(&self, value: GLfloat) {
		(self.version_4_0.minsampleshading)(value)
	}
	#[inline(always)]
	fn glBlendEquationi(&self, buf: GLuint, mode: GLenum) {
		(self.version_4_0.blendequationi)(buf, mode)
	}
	#[inline(always)]
	fn glBlendEquationSeparatei(&self, buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum) {
		(self.version_4_0.blendequationseparatei)(buf, modeRGB, modeAlpha)
	}
	#[inline(always)]
	fn glBlendFunci(&self, buf: GLuint, src: GLenum, dst: GLenum) {
		(self.version_4_0.blendfunci)(buf, src, dst)
	}
	#[inline(always)]
	fn glBlendFuncSeparatei(&self, buf: GLuint, srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum) {
		(self.version_4_0.blendfuncseparatei)(buf, srcRGB, dstRGB, srcAlpha, dstAlpha)
	}
	#[inline(always)]
	fn glDrawArraysIndirect(&self, mode: GLenum, indirect: *const c_void) {
		(self.version_4_0.drawarraysindirect)(mode, indirect)
	}
	#[inline(always)]
	fn glDrawElementsIndirect(&self, mode: GLenum, type_: GLenum, indirect: *const c_void) {
		(self.version_4_0.drawelementsindirect)(mode, type_, indirect)
	}
	#[inline(always)]
	fn glUniform1d(&self, location: GLint, x: GLdouble) {
		(self.version_4_0.uniform1d)(location, x)
	}
	#[inline(always)]
	fn glUniform2d(&self, location: GLint, x: GLdouble, y: GLdouble) {
		(self.version_4_0.uniform2d)(location, x, y)
	}
	#[inline(always)]
	fn glUniform3d(&self, location: GLint, x: GLdouble, y: GLdouble, z: GLdouble) {
		(self.version_4_0.uniform3d)(location, x, y, z)
	}
	#[inline(always)]
	fn glUniform4d(&self, location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) {
		(self.version_4_0.uniform4d)(location, x, y, z, w)
	}
	#[inline(always)]
	fn glUniform1dv(&self, location: GLint, count: GLsizei, value: *const GLdouble) {
		(self.version_4_0.uniform1dv)(location, count, value)
	}
	#[inline(always)]
	fn glUniform2dv(&self, location: GLint, count: GLsizei, value: *const GLdouble) {
		(self.version_4_0.uniform2dv)(location, count, value)
	}
	#[inline(always)]
	fn glUniform3dv(&self, location: GLint, count: GLsizei, value: *const GLdouble) {
		(self.version_4_0.uniform3dv)(location, count, value)
	}
	#[inline(always)]
	fn glUniform4dv(&self, location: GLint, count: GLsizei, value: *const GLdouble) {
		(self.version_4_0.uniform4dv)(location, count, value)
	}
	#[inline(always)]
	fn glUniformMatrix2dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.version_4_0.uniformmatrix2dv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glUniformMatrix3dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.version_4_0.uniformmatrix3dv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glUniformMatrix4dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.version_4_0.uniformmatrix4dv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glUniformMatrix2x3dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.version_4_0.uniformmatrix2x3dv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glUniformMatrix2x4dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.version_4_0.uniformmatrix2x4dv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glUniformMatrix3x2dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.version_4_0.uniformmatrix3x2dv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glUniformMatrix3x4dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.version_4_0.uniformmatrix3x4dv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glUniformMatrix4x2dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.version_4_0.uniformmatrix4x2dv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glUniformMatrix4x3dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.version_4_0.uniformmatrix4x3dv)(location, count, transpose, value)
	}
	#[inline(always)]
	fn glGetUniformdv(&self, program: GLuint, location: GLint, params: *mut GLdouble) {
		(self.version_4_0.getuniformdv)(program, location, params)
	}
	#[inline(always)]
	fn glGetSubroutineUniformLocation(&self, program: GLuint, shadertype: GLenum, name: *const GLchar) -> GLint {
		(self.version_4_0.getsubroutineuniformlocation)(program, shadertype, name)
	}
	#[inline(always)]
	fn glGetSubroutineIndex(&self, program: GLuint, shadertype: GLenum, name: *const GLchar) -> GLuint {
		(self.version_4_0.getsubroutineindex)(program, shadertype, name)
	}
	#[inline(always)]
	fn glGetActiveSubroutineUniformiv(&self, program: GLuint, shadertype: GLenum, index: GLuint, pname: GLenum, values: *mut GLint) {
		(self.version_4_0.getactivesubroutineuniformiv)(program, shadertype, index, pname, values)
	}
	#[inline(always)]
	fn glGetActiveSubroutineUniformName(&self, program: GLuint, shadertype: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar) {
		(self.version_4_0.getactivesubroutineuniformname)(program, shadertype, index, bufSize, length, name)
	}
	#[inline(always)]
	fn glGetActiveSubroutineName(&self, program: GLuint, shadertype: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar) {
		(self.version_4_0.getactivesubroutinename)(program, shadertype, index, bufSize, length, name)
	}
	#[inline(always)]
	fn glUniformSubroutinesuiv(&self, shadertype: GLenum, count: GLsizei, indices: *const GLuint) {
		(self.version_4_0.uniformsubroutinesuiv)(shadertype, count, indices)
	}
	#[inline(always)]
	fn glGetUniformSubroutineuiv(&self, shadertype: GLenum, location: GLint, params: *mut GLuint) {
		(self.version_4_0.getuniformsubroutineuiv)(shadertype, location, params)
	}
	#[inline(always)]
	fn glGetProgramStageiv(&self, program: GLuint, shadertype: GLenum, pname: GLenum, values: *mut GLint) {
		(self.version_4_0.getprogramstageiv)(program, shadertype, pname, values)
	}
	#[inline(always)]
	fn glPatchParameteri(&self, pname: GLenum, value: GLint) {
		(self.version_4_0.patchparameteri)(pname, value)
	}
	#[inline(always)]
	fn glPatchParameterfv(&self, pname: GLenum, values: *const GLfloat) {
		(self.version_4_0.patchparameterfv)(pname, values)
	}
	#[inline(always)]
	fn glBindTransformFeedback(&self, target: GLenum, id: GLuint) {
		(self.version_4_0.bindtransformfeedback)(target, id)
	}
	#[inline(always)]
	fn glDeleteTransformFeedbacks(&self, n: GLsizei, ids: *const GLuint) {
		(self.version_4_0.deletetransformfeedbacks)(n, ids)
	}
	#[inline(always)]
	fn glGenTransformFeedbacks(&self, n: GLsizei, ids: *mut GLuint) {
		(self.version_4_0.gentransformfeedbacks)(n, ids)
	}
	#[inline(always)]
	fn glIsTransformFeedback(&self, id: GLuint) -> GLboolean {
		(self.version_4_0.istransformfeedback)(id)
	}
	#[inline(always)]
	fn glPauseTransformFeedback(&self) {
		(self.version_4_0.pausetransformfeedback)()
	}
	#[inline(always)]
	fn glResumeTransformFeedback(&self) {
		(self.version_4_0.resumetransformfeedback)()
	}
	#[inline(always)]
	fn glDrawTransformFeedback(&self, mode: GLenum, id: GLuint) {
		(self.version_4_0.drawtransformfeedback)(mode, id)
	}
	#[inline(always)]
	fn glDrawTransformFeedbackStream(&self, mode: GLenum, id: GLuint, stream: GLuint) {
		(self.version_4_0.drawtransformfeedbackstream)(mode, id, stream)
	}
	#[inline(always)]
	fn glBeginQueryIndexed(&self, target: GLenum, index: GLuint, id: GLuint) {
		(self.version_4_0.beginqueryindexed)(target, index, id)
	}
	#[inline(always)]
	fn glEndQueryIndexed(&self, target: GLenum, index: GLuint) {
		(self.version_4_0.endqueryindexed)(target, index)
	}
	#[inline(always)]
	fn glGetQueryIndexediv(&self, target: GLenum, index: GLuint, pname: GLenum, params: *mut GLint) {
		(self.version_4_0.getqueryindexediv)(target, index, pname, params)
	}
}

impl GL_4_1 for GLCore {
	#[inline(always)]
	fn glReleaseShaderCompiler(&self) {
		(self.version_4_1.releaseshadercompiler)()
	}
	#[inline(always)]
	fn glShaderBinary(&self, count: GLsizei, shaders: *const GLuint, binaryFormat: GLenum, binary: *const c_void, length: GLsizei) {
		(self.version_4_1.shaderbinary)(count, shaders, binaryFormat, binary, length)
	}
	#[inline(always)]
	fn glGetShaderPrecisionFormat(&self, shadertype: GLenum, precisiontype: GLenum, range: *mut GLint, precision: *mut GLint) {
		(self.version_4_1.getshaderprecisionformat)(shadertype, precisiontype, range, precision)
	}
	#[inline(always)]
	fn glDepthRangef(&self, n: GLfloat, f: GLfloat) {
		(self.version_4_1.depthrangef)(n, f)
	}
	#[inline(always)]
	fn glClearDepthf(&self, d: GLfloat) {
		(self.version_4_1.cleardepthf)(d)
	}
	#[inline(always)]
	fn glGetProgramBinary(&self, program: GLuint, bufSize: GLsizei, length: *mut GLsizei, binaryFormat: *mut GLenum, binary: *mut c_void) {
		(self.version_4_1.getprogrambinary)(program, bufSize, length, binaryFormat, binary)
	}
	#[inline(always)]
	fn glProgramBinary(&self, program: GLuint, binaryFormat: GLenum, binary: *const c_void, length: GLsizei) {
		(self.version_4_1.programbinary)(program, binaryFormat, binary, length)
	}
	#[inline(always)]
	fn glProgramParameteri(&self, program: GLuint, pname: GLenum, value: GLint) {
		(self.version_4_1.programparameteri)(program, pname, value)
	}
	#[inline(always)]
	fn glUseProgramStages(&self, pipeline: GLuint, stages: GLbitfield, program: GLuint) {
		(self.version_4_1.useprogramstages)(pipeline, stages, program)
	}
	#[inline(always)]
	fn glActiveShaderProgram(&self, pipeline: GLuint, program: GLuint) {
		(self.version_4_1.activeshaderprogram)(pipeline, program)
	}
	#[inline(always)]
	fn glCreateShaderProgramv(&self, type_: GLenum, count: GLsizei, strings: *const *const GLchar) -> GLuint {
		(self.version_4_1.createshaderprogramv)(type_, count, strings)
	}
	#[inline(always)]
	fn glBindProgramPipeline(&self, pipeline: GLuint) {
		(self.version_4_1.bindprogrampipeline)(pipeline)
	}
	#[inline(always)]
	fn glDeleteProgramPipelines(&self, n: GLsizei, pipelines: *const GLuint) {
		(self.version_4_1.deleteprogrampipelines)(n, pipelines)
	}
	#[inline(always)]
	fn glGenProgramPipelines(&self, n: GLsizei, pipelines: *mut GLuint) {
		(self.version_4_1.genprogrampipelines)(n, pipelines)
	}
	#[inline(always)]
	fn glIsProgramPipeline(&self, pipeline: GLuint) -> GLboolean {
		(self.version_4_1.isprogrampipeline)(pipeline)
	}
	#[inline(always)]
	fn glGetProgramPipelineiv(&self, pipeline: GLuint, pname: GLenum, params: *mut GLint) {
		(self.version_4_1.getprogrampipelineiv)(pipeline, pname, params)
	}
	#[inline(always)]
	fn glProgramUniform1i(&self, program: GLuint, location: GLint, v0: GLint) {
		(self.version_4_1.programuniform1i)(program, location, v0)
	}
	#[inline(always)]
	fn glProgramUniform1iv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLint) {
		(self.version_4_1.programuniform1iv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniform1f(&self, program: GLuint, location: GLint, v0: GLfloat) {
		(self.version_4_1.programuniform1f)(program, location, v0)
	}
	#[inline(always)]
	fn glProgramUniform1fv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) {
		(self.version_4_1.programuniform1fv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniform1d(&self, program: GLuint, location: GLint, v0: GLdouble) {
		(self.version_4_1.programuniform1d)(program, location, v0)
	}
	#[inline(always)]
	fn glProgramUniform1dv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) {
		(self.version_4_1.programuniform1dv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniform1ui(&self, program: GLuint, location: GLint, v0: GLuint) {
		(self.version_4_1.programuniform1ui)(program, location, v0)
	}
	#[inline(always)]
	fn glProgramUniform1uiv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) {
		(self.version_4_1.programuniform1uiv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniform2i(&self, program: GLuint, location: GLint, v0: GLint, v1: GLint) {
		(self.version_4_1.programuniform2i)(program, location, v0, v1)
	}
	#[inline(always)]
	fn glProgramUniform2iv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLint) {
		(self.version_4_1.programuniform2iv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniform2f(&self, program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat) {
		(self.version_4_1.programuniform2f)(program, location, v0, v1)
	}
	#[inline(always)]
	fn glProgramUniform2fv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) {
		(self.version_4_1.programuniform2fv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniform2d(&self, program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble) {
		(self.version_4_1.programuniform2d)(program, location, v0, v1)
	}
	#[inline(always)]
	fn glProgramUniform2dv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) {
		(self.version_4_1.programuniform2dv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniform2ui(&self, program: GLuint, location: GLint, v0: GLuint, v1: GLuint) {
		(self.version_4_1.programuniform2ui)(program, location, v0, v1)
	}
	#[inline(always)]
	fn glProgramUniform2uiv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) {
		(self.version_4_1.programuniform2uiv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniform3i(&self, program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint) {
		(self.version_4_1.programuniform3i)(program, location, v0, v1, v2)
	}
	#[inline(always)]
	fn glProgramUniform3iv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLint) {
		(self.version_4_1.programuniform3iv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniform3f(&self, program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) {
		(self.version_4_1.programuniform3f)(program, location, v0, v1, v2)
	}
	#[inline(always)]
	fn glProgramUniform3fv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) {
		(self.version_4_1.programuniform3fv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniform3d(&self, program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble) {
		(self.version_4_1.programuniform3d)(program, location, v0, v1, v2)
	}
	#[inline(always)]
	fn glProgramUniform3dv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) {
		(self.version_4_1.programuniform3dv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniform3ui(&self, program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) {
		(self.version_4_1.programuniform3ui)(program, location, v0, v1, v2)
	}
	#[inline(always)]
	fn glProgramUniform3uiv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) {
		(self.version_4_1.programuniform3uiv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniform4i(&self, program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) {
		(self.version_4_1.programuniform4i)(program, location, v0, v1, v2, v3)
	}
	#[inline(always)]
	fn glProgramUniform4iv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLint) {
		(self.version_4_1.programuniform4iv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniform4f(&self, program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) {
		(self.version_4_1.programuniform4f)(program, location, v0, v1, v2, v3)
	}
	#[inline(always)]
	fn glProgramUniform4fv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) {
		(self.version_4_1.programuniform4fv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniform4d(&self, program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble, v3: GLdouble) {
		(self.version_4_1.programuniform4d)(program, location, v0, v1, v2, v3)
	}
	#[inline(always)]
	fn glProgramUniform4dv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) {
		(self.version_4_1.programuniform4dv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniform4ui(&self, program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) {
		(self.version_4_1.programuniform4ui)(program, location, v0, v1, v2, v3)
	}
	#[inline(always)]
	fn glProgramUniform4uiv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) {
		(self.version_4_1.programuniform4uiv)(program, location, count, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix2fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.version_4_1.programuniformmatrix2fv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix3fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.version_4_1.programuniformmatrix3fv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix4fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.version_4_1.programuniformmatrix4fv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix2dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.version_4_1.programuniformmatrix2dv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix3dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.version_4_1.programuniformmatrix3dv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix4dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.version_4_1.programuniformmatrix4dv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix2x3fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.version_4_1.programuniformmatrix2x3fv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix3x2fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.version_4_1.programuniformmatrix3x2fv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix2x4fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.version_4_1.programuniformmatrix2x4fv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix4x2fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.version_4_1.programuniformmatrix4x2fv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix3x4fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.version_4_1.programuniformmatrix3x4fv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix4x3fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) {
		(self.version_4_1.programuniformmatrix4x3fv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix2x3dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.version_4_1.programuniformmatrix2x3dv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix3x2dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.version_4_1.programuniformmatrix3x2dv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix2x4dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.version_4_1.programuniformmatrix2x4dv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix4x2dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.version_4_1.programuniformmatrix4x2dv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix3x4dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.version_4_1.programuniformmatrix3x4dv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glProgramUniformMatrix4x3dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) {
		(self.version_4_1.programuniformmatrix4x3dv)(program, location, count, transpose, value)
	}
	#[inline(always)]
	fn glValidateProgramPipeline(&self, pipeline: GLuint) {
		(self.version_4_1.validateprogrampipeline)(pipeline)
	}
	#[inline(always)]
	fn glGetProgramPipelineInfoLog(&self, pipeline: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) {
		(self.version_4_1.getprogrampipelineinfolog)(pipeline, bufSize, length, infoLog)
	}
	#[inline(always)]
	fn glVertexAttribL1d(&self, index: GLuint, x: GLdouble) {
		(self.version_4_1.vertexattribl1d)(index, x)
	}
	#[inline(always)]
	fn glVertexAttribL2d(&self, index: GLuint, x: GLdouble, y: GLdouble) {
		(self.version_4_1.vertexattribl2d)(index, x, y)
	}
	#[inline(always)]
	fn glVertexAttribL3d(&self, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) {
		(self.version_4_1.vertexattribl3d)(index, x, y, z)
	}
	#[inline(always)]
	fn glVertexAttribL4d(&self, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) {
		(self.version_4_1.vertexattribl4d)(index, x, y, z, w)
	}
	#[inline(always)]
	fn glVertexAttribL1dv(&self, index: GLuint, v: *const GLdouble) {
		(self.version_4_1.vertexattribl1dv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribL2dv(&self, index: GLuint, v: *const GLdouble) {
		(self.version_4_1.vertexattribl2dv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribL3dv(&self, index: GLuint, v: *const GLdouble) {
		(self.version_4_1.vertexattribl3dv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribL4dv(&self, index: GLuint, v: *const GLdouble) {
		(self.version_4_1.vertexattribl4dv)(index, v)
	}
	#[inline(always)]
	fn glVertexAttribLPointer(&self, index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void) {
		(self.version_4_1.vertexattriblpointer)(index, size, type_, stride, pointer)
	}
	#[inline(always)]
	fn glGetVertexAttribLdv(&self, index: GLuint, pname: GLenum, params: *mut GLdouble) {
		(self.version_4_1.getvertexattribldv)(index, pname, params)
	}
	#[inline(always)]
	fn glViewportArrayv(&self, first: GLuint, count: GLsizei, v: *const GLfloat) {
		(self.version_4_1.viewportarrayv)(first, count, v)
	}
	#[inline(always)]
	fn glViewportIndexedf(&self, index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat) {
		(self.version_4_1.viewportindexedf)(index, x, y, w, h)
	}
	#[inline(always)]
	fn glViewportIndexedfv(&self, index: GLuint, v: *const GLfloat) {
		(self.version_4_1.viewportindexedfv)(index, v)
	}
	#[inline(always)]
	fn glScissorArrayv(&self, first: GLuint, count: GLsizei, v: *const GLint) {
		(self.version_4_1.scissorarrayv)(first, count, v)
	}
	#[inline(always)]
	fn glScissorIndexed(&self, index: GLuint, left: GLint, bottom: GLint, width: GLsizei, height: GLsizei) {
		(self.version_4_1.scissorindexed)(index, left, bottom, width, height)
	}
	#[inline(always)]
	fn glScissorIndexedv(&self, index: GLuint, v: *const GLint) {
		(self.version_4_1.scissorindexedv)(index, v)
	}
	#[inline(always)]
	fn glDepthRangeArrayv(&self, first: GLuint, count: GLsizei, v: *const GLdouble) {
		(self.version_4_1.depthrangearrayv)(first, count, v)
	}
	#[inline(always)]
	fn glDepthRangeIndexed(&self, index: GLuint, n: GLdouble, f: GLdouble) {
		(self.version_4_1.depthrangeindexed)(index, n, f)
	}
	#[inline(always)]
	fn glGetFloati_v(&self, target: GLenum, index: GLuint, data: *mut GLfloat) {
		(self.version_4_1.getfloati_v)(target, index, data)
	}
	#[inline(always)]
	fn glGetDoublei_v(&self, target: GLenum, index: GLuint, data: *mut GLdouble) {
		(self.version_4_1.getdoublei_v)(target, index, data)
	}
}

impl GL_4_2 for GLCore {
	#[inline(always)]
	fn glDrawArraysInstancedBaseInstance(&self, mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei, baseinstance: GLuint) {
		(self.version_4_2.drawarraysinstancedbaseinstance)(mode, first, count, instancecount, baseinstance)
	}
	#[inline(always)]
	fn glDrawElementsInstancedBaseInstance(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei, baseinstance: GLuint) {
		(self.version_4_2.drawelementsinstancedbaseinstance)(mode, count, type_, indices, instancecount, baseinstance)
	}
	#[inline(always)]
	fn glDrawElementsInstancedBaseVertexBaseInstance(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei, basevertex: GLint, baseinstance: GLuint) {
		(self.version_4_2.drawelementsinstancedbasevertexbaseinstance)(mode, count, type_, indices, instancecount, basevertex, baseinstance)
	}
	#[inline(always)]
	fn glGetInternalformativ(&self, target: GLenum, internalformat: GLenum, pname: GLenum, count: GLsizei, params: *mut GLint) {
		(self.version_4_2.getinternalformativ)(target, internalformat, pname, count, params)
	}
	#[inline(always)]
	fn glGetActiveAtomicCounterBufferiv(&self, program: GLuint, bufferIndex: GLuint, pname: GLenum, params: *mut GLint) {
		(self.version_4_2.getactiveatomiccounterbufferiv)(program, bufferIndex, pname, params)
	}
	#[inline(always)]
	fn glBindImageTexture(&self, unit: GLuint, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, access: GLenum, format: GLenum) {
		(self.version_4_2.bindimagetexture)(unit, texture, level, layered, layer, access, format)
	}
	#[inline(always)]
	fn glMemoryBarrier(&self, barriers: GLbitfield) {
		(self.version_4_2.memorybarrier)(barriers)
	}
	#[inline(always)]
	fn glTexStorage1D(&self, target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei) {
		(self.version_4_2.texstorage1d)(target, levels, internalformat, width)
	}
	#[inline(always)]
	fn glTexStorage2D(&self, target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) {
		(self.version_4_2.texstorage2d)(target, levels, internalformat, width, height)
	}
	#[inline(always)]
	fn glTexStorage3D(&self, target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei) {
		(self.version_4_2.texstorage3d)(target, levels, internalformat, width, height, depth)
	}
	#[inline(always)]
	fn glDrawTransformFeedbackInstanced(&self, mode: GLenum, id: GLuint, instancecount: GLsizei) {
		(self.version_4_2.drawtransformfeedbackinstanced)(mode, id, instancecount)
	}
	#[inline(always)]
	fn glDrawTransformFeedbackStreamInstanced(&self, mode: GLenum, id: GLuint, stream: GLuint, instancecount: GLsizei) {
		(self.version_4_2.drawtransformfeedbackstreaminstanced)(mode, id, stream, instancecount)
	}
}

impl GL_4_3 for GLCore {
	#[inline(always)]
	fn glClearBufferData(&self, target: GLenum, internalformat: GLenum, format: GLenum, type_: GLenum, data: *const c_void) {
		(self.version_4_3.clearbufferdata)(target, internalformat, format, type_, data)
	}
	#[inline(always)]
	fn glClearBufferSubData(&self, target: GLenum, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *const c_void) {
		(self.version_4_3.clearbuffersubdata)(target, internalformat, offset, size, format, type_, data)
	}
	#[inline(always)]
	fn glDispatchCompute(&self, num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint) {
		(self.version_4_3.dispatchcompute)(num_groups_x, num_groups_y, num_groups_z)
	}
	#[inline(always)]
	fn glDispatchComputeIndirect(&self, indirect: GLintptr) {
		(self.version_4_3.dispatchcomputeindirect)(indirect)
	}
	#[inline(always)]
	fn glCopyImageSubData(&self, srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei) {
		(self.version_4_3.copyimagesubdata)(srcName, srcTarget, srcLevel, srcX, srcY, srcZ, dstName, dstTarget, dstLevel, dstX, dstY, dstZ, srcWidth, srcHeight, srcDepth)
	}
	#[inline(always)]
	fn glFramebufferParameteri(&self, target: GLenum, pname: GLenum, param: GLint) {
		(self.version_4_3.framebufferparameteri)(target, pname, param)
	}
	#[inline(always)]
	fn glGetFramebufferParameteriv(&self, target: GLenum, pname: GLenum, params: *mut GLint) {
		(self.version_4_3.getframebufferparameteriv)(target, pname, params)
	}
	#[inline(always)]
	fn glGetInternalformati64v(&self, target: GLenum, internalformat: GLenum, pname: GLenum, count: GLsizei, params: *mut GLint64) {
		(self.version_4_3.getinternalformati64v)(target, internalformat, pname, count, params)
	}
	#[inline(always)]
	fn glInvalidateTexSubImage(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei) {
		(self.version_4_3.invalidatetexsubimage)(texture, level, xoffset, yoffset, zoffset, width, height, depth)
	}
	#[inline(always)]
	fn glInvalidateTexImage(&self, texture: GLuint, level: GLint) {
		(self.version_4_3.invalidateteximage)(texture, level)
	}
	#[inline(always)]
	fn glInvalidateBufferSubData(&self, buffer: GLuint, offset: GLintptr, length: GLsizeiptr) {
		(self.version_4_3.invalidatebuffersubdata)(buffer, offset, length)
	}
	#[inline(always)]
	fn glInvalidateBufferData(&self, buffer: GLuint) {
		(self.version_4_3.invalidatebufferdata)(buffer)
	}
	#[inline(always)]
	fn glInvalidateFramebuffer(&self, target: GLenum, numAttachments: GLsizei, attachments: *const GLenum) {
		(self.version_4_3.invalidateframebuffer)(target, numAttachments, attachments)
	}
	#[inline(always)]
	fn glInvalidateSubFramebuffer(&self, target: GLenum, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
		(self.version_4_3.invalidatesubframebuffer)(target, numAttachments, attachments, x, y, width, height)
	}
	#[inline(always)]
	fn glMultiDrawArraysIndirect(&self, mode: GLenum, indirect: *const c_void, drawcount: GLsizei, stride: GLsizei) {
		(self.version_4_3.multidrawarraysindirect)(mode, indirect, drawcount, stride)
	}
	#[inline(always)]
	fn glMultiDrawElementsIndirect(&self, mode: GLenum, type_: GLenum, indirect: *const c_void, drawcount: GLsizei, stride: GLsizei) {
		(self.version_4_3.multidrawelementsindirect)(mode, type_, indirect, drawcount, stride)
	}
	#[inline(always)]
	fn glGetProgramInterfaceiv(&self, program: GLuint, programInterface: GLenum, pname: GLenum, params: *mut GLint) {
		(self.version_4_3.getprograminterfaceiv)(program, programInterface, pname, params)
	}
	#[inline(always)]
	fn glGetProgramResourceIndex(&self, program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLuint {
		(self.version_4_3.getprogramresourceindex)(program, programInterface, name)
	}
	#[inline(always)]
	fn glGetProgramResourceName(&self, program: GLuint, programInterface: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar) {
		(self.version_4_3.getprogramresourcename)(program, programInterface, index, bufSize, length, name)
	}
	#[inline(always)]
	fn glGetProgramResourceiv(&self, program: GLuint, programInterface: GLenum, index: GLuint, propCount: GLsizei, props: *const GLenum, count: GLsizei, length: *mut GLsizei, params: *mut GLint) {
		(self.version_4_3.getprogramresourceiv)(program, programInterface, index, propCount, props, count, length, params)
	}
	#[inline(always)]
	fn glGetProgramResourceLocation(&self, program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLint {
		(self.version_4_3.getprogramresourcelocation)(program, programInterface, name)
	}
	#[inline(always)]
	fn glGetProgramResourceLocationIndex(&self, program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLint {
		(self.version_4_3.getprogramresourcelocationindex)(program, programInterface, name)
	}
	#[inline(always)]
	fn glShaderStorageBlockBinding(&self, program: GLuint, storageBlockIndex: GLuint, storageBlockBinding: GLuint) {
		(self.version_4_3.shaderstorageblockbinding)(program, storageBlockIndex, storageBlockBinding)
	}
	#[inline(always)]
	fn glTexBufferRange(&self, target: GLenum, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) {
		(self.version_4_3.texbufferrange)(target, internalformat, buffer, offset, size)
	}
	#[inline(always)]
	fn glTexStorage2DMultisample(&self, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) {
		(self.version_4_3.texstorage2dmultisample)(target, samples, internalformat, width, height, fixedsamplelocations)
	}
	#[inline(always)]
	fn glTexStorage3DMultisample(&self, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) {
		(self.version_4_3.texstorage3dmultisample)(target, samples, internalformat, width, height, depth, fixedsamplelocations)
	}
	#[inline(always)]
	fn glTextureView(&self, texture: GLuint, target: GLenum, origtexture: GLuint, internalformat: GLenum, minlevel: GLuint, numlevels: GLuint, minlayer: GLuint, numlayers: GLuint) {
		(self.version_4_3.textureview)(texture, target, origtexture, internalformat, minlevel, numlevels, minlayer, numlayers)
	}
	#[inline(always)]
	fn glBindVertexBuffer(&self, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei) {
		(self.version_4_3.bindvertexbuffer)(bindingindex, buffer, offset, stride)
	}
	#[inline(always)]
	fn glVertexAttribFormat(&self, attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint) {
		(self.version_4_3.vertexattribformat)(attribindex, size, type_, normalized, relativeoffset)
	}
	#[inline(always)]
	fn glVertexAttribIFormat(&self, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) {
		(self.version_4_3.vertexattribiformat)(attribindex, size, type_, relativeoffset)
	}
	#[inline(always)]
	fn glVertexAttribLFormat(&self, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) {
		(self.version_4_3.vertexattriblformat)(attribindex, size, type_, relativeoffset)
	}
	#[inline(always)]
	fn glVertexAttribBinding(&self, attribindex: GLuint, bindingindex: GLuint) {
		(self.version_4_3.vertexattribbinding)(attribindex, bindingindex)
	}
	#[inline(always)]
	fn glVertexBindingDivisor(&self, bindingindex: GLuint, divisor: GLuint) {
		(self.version_4_3.vertexbindingdivisor)(bindingindex, divisor)
	}
	#[inline(always)]
	fn glDebugMessageControl(&self, source: GLenum, type_: GLenum, severity: GLenum, count: GLsizei, ids: *const GLuint, enabled: GLboolean) {
		(self.version_4_3.debugmessagecontrol)(source, type_, severity, count, ids, enabled)
	}
	#[inline(always)]
	fn glDebugMessageInsert(&self, source: GLenum, type_: GLenum, id: GLuint, severity: GLenum, length: GLsizei, buf: *const GLchar) {
		(self.version_4_3.debugmessageinsert)(source, type_, id, severity, length, buf)
	}
	#[inline(always)]
	fn glDebugMessageCallback(&self, callback: GLDEBUGPROC, userParam: *const c_void) {
		(self.version_4_3.debugmessagecallback)(callback, userParam)
	}
	#[inline(always)]
	fn glGetDebugMessageLog(&self, count: GLuint, bufSize: GLsizei, sources: *mut GLenum, types: *mut GLenum, ids: *mut GLuint, severities: *mut GLenum, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint {
		(self.version_4_3.getdebugmessagelog)(count, bufSize, sources, types, ids, severities, lengths, messageLog)
	}
	#[inline(always)]
	fn glPushDebugGroup(&self, source: GLenum, id: GLuint, length: GLsizei, message: *const GLchar) {
		(self.version_4_3.pushdebuggroup)(source, id, length, message)
	}
	#[inline(always)]
	fn glPopDebugGroup(&self) {
		(self.version_4_3.popdebuggroup)()
	}
	#[inline(always)]
	fn glObjectLabel(&self, identifier: GLenum, name: GLuint, length: GLsizei, label: *const GLchar) {
		(self.version_4_3.objectlabel)(identifier, name, length, label)
	}
	#[inline(always)]
	fn glGetObjectLabel(&self, identifier: GLenum, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) {
		(self.version_4_3.getobjectlabel)(identifier, name, bufSize, length, label)
	}
	#[inline(always)]
	fn glObjectPtrLabel(&self, ptr: *const c_void, length: GLsizei, label: *const GLchar) {
		(self.version_4_3.objectptrlabel)(ptr, length, label)
	}
	#[inline(always)]
	fn glGetObjectPtrLabel(&self, ptr: *const c_void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) {
		(self.version_4_3.getobjectptrlabel)(ptr, bufSize, length, label)
	}
}

impl GL_4_4 for GLCore {
	#[inline(always)]
	fn glBufferStorage(&self, target: GLenum, size: GLsizeiptr, data: *const c_void, flags: GLbitfield) {
		(self.version_4_4.bufferstorage)(target, size, data, flags)
	}
	#[inline(always)]
	fn glClearTexImage(&self, texture: GLuint, level: GLint, format: GLenum, type_: GLenum, data: *const c_void) {
		(self.version_4_4.clearteximage)(texture, level, format, type_, data)
	}
	#[inline(always)]
	fn glClearTexSubImage(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, data: *const c_void) {
		(self.version_4_4.cleartexsubimage)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, data)
	}
	#[inline(always)]
	fn glBindBuffersBase(&self, target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint) {
		(self.version_4_4.bindbuffersbase)(target, first, count, buffers)
	}
	#[inline(always)]
	fn glBindBuffersRange(&self, target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, sizes: *const GLsizeiptr) {
		(self.version_4_4.bindbuffersrange)(target, first, count, buffers, offsets, sizes)
	}
	#[inline(always)]
	fn glBindTextures(&self, first: GLuint, count: GLsizei, textures: *const GLuint) {
		(self.version_4_4.bindtextures)(first, count, textures)
	}
	#[inline(always)]
	fn glBindSamplers(&self, first: GLuint, count: GLsizei, samplers: *const GLuint) {
		(self.version_4_4.bindsamplers)(first, count, samplers)
	}
	#[inline(always)]
	fn glBindImageTextures(&self, first: GLuint, count: GLsizei, textures: *const GLuint) {
		(self.version_4_4.bindimagetextures)(first, count, textures)
	}
	#[inline(always)]
	fn glBindVertexBuffers(&self, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei) {
		(self.version_4_4.bindvertexbuffers)(first, count, buffers, offsets, strides)
	}
}

impl GL_4_5 for GLCore {
	#[inline(always)]
	fn glClipControl(&self, origin: GLenum, depth: GLenum) {
		(self.version_4_5.clipcontrol)(origin, depth)
	}
	#[inline(always)]
	fn glCreateTransformFeedbacks(&self, n: GLsizei, ids: *mut GLuint) {
		(self.version_4_5.createtransformfeedbacks)(n, ids)
	}
	#[inline(always)]
	fn glTransformFeedbackBufferBase(&self, xfb: GLuint, index: GLuint, buffer: GLuint) {
		(self.version_4_5.transformfeedbackbufferbase)(xfb, index, buffer)
	}
	#[inline(always)]
	fn glTransformFeedbackBufferRange(&self, xfb: GLuint, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) {
		(self.version_4_5.transformfeedbackbufferrange)(xfb, index, buffer, offset, size)
	}
	#[inline(always)]
	fn glGetTransformFeedbackiv(&self, xfb: GLuint, pname: GLenum, param: *mut GLint) {
		(self.version_4_5.gettransformfeedbackiv)(xfb, pname, param)
	}
	#[inline(always)]
	fn glGetTransformFeedbacki_v(&self, xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint) {
		(self.version_4_5.gettransformfeedbacki_v)(xfb, pname, index, param)
	}
	#[inline(always)]
	fn glGetTransformFeedbacki64_v(&self, xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint64) {
		(self.version_4_5.gettransformfeedbacki64_v)(xfb, pname, index, param)
	}
	#[inline(always)]
	fn glCreateBuffers(&self, n: GLsizei, buffers: *mut GLuint) {
		(self.version_4_5.createbuffers)(n, buffers)
	}
	#[inline(always)]
	fn glNamedBufferStorage(&self, buffer: GLuint, size: GLsizeiptr, data: *const c_void, flags: GLbitfield) {
		(self.version_4_5.namedbufferstorage)(buffer, size, data, flags)
	}
	#[inline(always)]
	fn glNamedBufferData(&self, buffer: GLuint, size: GLsizeiptr, data: *const c_void, usage: GLenum) {
		(self.version_4_5.namedbufferdata)(buffer, size, data, usage)
	}
	#[inline(always)]
	fn glNamedBufferSubData(&self, buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const c_void) {
		(self.version_4_5.namedbuffersubdata)(buffer, offset, size, data)
	}
	#[inline(always)]
	fn glCopyNamedBufferSubData(&self, readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr) {
		(self.version_4_5.copynamedbuffersubdata)(readBuffer, writeBuffer, readOffset, writeOffset, size)
	}
	#[inline(always)]
	fn glClearNamedBufferData(&self, buffer: GLuint, internalformat: GLenum, format: GLenum, type_: GLenum, data: *const c_void) {
		(self.version_4_5.clearnamedbufferdata)(buffer, internalformat, format, type_, data)
	}
	#[inline(always)]
	fn glClearNamedBufferSubData(&self, buffer: GLuint, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *const c_void) {
		(self.version_4_5.clearnamedbuffersubdata)(buffer, internalformat, offset, size, format, type_, data)
	}
	#[inline(always)]
	fn glMapNamedBuffer(&self, buffer: GLuint, access: GLenum) -> *mut c_void {
		(self.version_4_5.mapnamedbuffer)(buffer, access)
	}
	#[inline(always)]
	fn glMapNamedBufferRange(&self, buffer: GLuint, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut c_void {
		(self.version_4_5.mapnamedbufferrange)(buffer, offset, length, access)
	}
	#[inline(always)]
	fn glUnmapNamedBuffer(&self, buffer: GLuint) -> GLboolean {
		(self.version_4_5.unmapnamedbuffer)(buffer)
	}
	#[inline(always)]
	fn glFlushMappedNamedBufferRange(&self, buffer: GLuint, offset: GLintptr, length: GLsizeiptr) {
		(self.version_4_5.flushmappednamedbufferrange)(buffer, offset, length)
	}
	#[inline(always)]
	fn glGetNamedBufferParameteriv(&self, buffer: GLuint, pname: GLenum, params: *mut GLint) {
		(self.version_4_5.getnamedbufferparameteriv)(buffer, pname, params)
	}
	#[inline(always)]
	fn glGetNamedBufferParameteri64v(&self, buffer: GLuint, pname: GLenum, params: *mut GLint64) {
		(self.version_4_5.getnamedbufferparameteri64v)(buffer, pname, params)
	}
	#[inline(always)]
	fn glGetNamedBufferPointerv(&self, buffer: GLuint, pname: GLenum, params: *mut *mut c_void) {
		(self.version_4_5.getnamedbufferpointerv)(buffer, pname, params)
	}
	#[inline(always)]
	fn glGetNamedBufferSubData(&self, buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *mut c_void) {
		(self.version_4_5.getnamedbuffersubdata)(buffer, offset, size, data)
	}
	#[inline(always)]
	fn glCreateFramebuffers(&self, n: GLsizei, framebuffers: *mut GLuint) {
		(self.version_4_5.createframebuffers)(n, framebuffers)
	}
	#[inline(always)]
	fn glNamedFramebufferRenderbuffer(&self, framebuffer: GLuint, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint) {
		(self.version_4_5.namedframebufferrenderbuffer)(framebuffer, attachment, renderbuffertarget, renderbuffer)
	}
	#[inline(always)]
	fn glNamedFramebufferParameteri(&self, framebuffer: GLuint, pname: GLenum, param: GLint) {
		(self.version_4_5.namedframebufferparameteri)(framebuffer, pname, param)
	}
	#[inline(always)]
	fn glNamedFramebufferTexture(&self, framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint) {
		(self.version_4_5.namedframebuffertexture)(framebuffer, attachment, texture, level)
	}
	#[inline(always)]
	fn glNamedFramebufferTextureLayer(&self, framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint) {
		(self.version_4_5.namedframebuffertexturelayer)(framebuffer, attachment, texture, level, layer)
	}
	#[inline(always)]
	fn glNamedFramebufferDrawBuffer(&self, framebuffer: GLuint, buf: GLenum) {
		(self.version_4_5.namedframebufferdrawbuffer)(framebuffer, buf)
	}
	#[inline(always)]
	fn glNamedFramebufferDrawBuffers(&self, framebuffer: GLuint, n: GLsizei, bufs: *const GLenum) {
		(self.version_4_5.namedframebufferdrawbuffers)(framebuffer, n, bufs)
	}
	#[inline(always)]
	fn glNamedFramebufferReadBuffer(&self, framebuffer: GLuint, src: GLenum) {
		(self.version_4_5.namedframebufferreadbuffer)(framebuffer, src)
	}
	#[inline(always)]
	fn glInvalidateNamedFramebufferData(&self, framebuffer: GLuint, numAttachments: GLsizei, attachments: *const GLenum) {
		(self.version_4_5.invalidatenamedframebufferdata)(framebuffer, numAttachments, attachments)
	}
	#[inline(always)]
	fn glInvalidateNamedFramebufferSubData(&self, framebuffer: GLuint, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
		(self.version_4_5.invalidatenamedframebuffersubdata)(framebuffer, numAttachments, attachments, x, y, width, height)
	}
	#[inline(always)]
	fn glClearNamedFramebufferiv(&self, framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLint) {
		(self.version_4_5.clearnamedframebufferiv)(framebuffer, buffer, drawbuffer, value)
	}
	#[inline(always)]
	fn glClearNamedFramebufferuiv(&self, framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLuint) {
		(self.version_4_5.clearnamedframebufferuiv)(framebuffer, buffer, drawbuffer, value)
	}
	#[inline(always)]
	fn glClearNamedFramebufferfv(&self, framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLfloat) {
		(self.version_4_5.clearnamedframebufferfv)(framebuffer, buffer, drawbuffer, value)
	}
	#[inline(always)]
	fn glClearNamedFramebufferfi(&self, framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint) {
		(self.version_4_5.clearnamedframebufferfi)(framebuffer, buffer, drawbuffer, depth, stencil)
	}
	#[inline(always)]
	fn glBlitNamedFramebuffer(&self, readFramebuffer: GLuint, drawFramebuffer: GLuint, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum) {
		(self.version_4_5.blitnamedframebuffer)(readFramebuffer, drawFramebuffer, srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter)
	}
	#[inline(always)]
	fn glCheckNamedFramebufferStatus(&self, framebuffer: GLuint, target: GLenum) -> GLenum {
		(self.version_4_5.checknamedframebufferstatus)(framebuffer, target)
	}
	#[inline(always)]
	fn glGetNamedFramebufferParameteriv(&self, framebuffer: GLuint, pname: GLenum, param: *mut GLint) {
		(self.version_4_5.getnamedframebufferparameteriv)(framebuffer, pname, param)
	}
	#[inline(always)]
	fn glGetNamedFramebufferAttachmentParameteriv(&self, framebuffer: GLuint, attachment: GLenum, pname: GLenum, params: *mut GLint) {
		(self.version_4_5.getnamedframebufferattachmentparameteriv)(framebuffer, attachment, pname, params)
	}
	#[inline(always)]
	fn glCreateRenderbuffers(&self, n: GLsizei, renderbuffers: *mut GLuint) {
		(self.version_4_5.createrenderbuffers)(n, renderbuffers)
	}
	#[inline(always)]
	fn glNamedRenderbufferStorage(&self, renderbuffer: GLuint, internalformat: GLenum, width: GLsizei, height: GLsizei) {
		(self.version_4_5.namedrenderbufferstorage)(renderbuffer, internalformat, width, height)
	}
	#[inline(always)]
	fn glNamedRenderbufferStorageMultisample(&self, renderbuffer: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) {
		(self.version_4_5.namedrenderbufferstoragemultisample)(renderbuffer, samples, internalformat, width, height)
	}
	#[inline(always)]
	fn glGetNamedRenderbufferParameteriv(&self, renderbuffer: GLuint, pname: GLenum, params: *mut GLint) {
		(self.version_4_5.getnamedrenderbufferparameteriv)(renderbuffer, pname, params)
	}
	#[inline(always)]
	fn glCreateTextures(&self, target: GLenum, n: GLsizei, textures: *mut GLuint) {
		(self.version_4_5.createtextures)(target, n, textures)
	}
	#[inline(always)]
	fn glTextureBuffer(&self, texture: GLuint, internalformat: GLenum, buffer: GLuint) {
		(self.version_4_5.texturebuffer)(texture, internalformat, buffer)
	}
	#[inline(always)]
	fn glTextureBufferRange(&self, texture: GLuint, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) {
		(self.version_4_5.texturebufferrange)(texture, internalformat, buffer, offset, size)
	}
	#[inline(always)]
	fn glTextureStorage1D(&self, texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei) {
		(self.version_4_5.texturestorage1d)(texture, levels, internalformat, width)
	}
	#[inline(always)]
	fn glTextureStorage2D(&self, texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) {
		(self.version_4_5.texturestorage2d)(texture, levels, internalformat, width, height)
	}
	#[inline(always)]
	fn glTextureStorage3D(&self, texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei) {
		(self.version_4_5.texturestorage3d)(texture, levels, internalformat, width, height, depth)
	}
	#[inline(always)]
	fn glTextureStorage2DMultisample(&self, texture: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) {
		(self.version_4_5.texturestorage2dmultisample)(texture, samples, internalformat, width, height, fixedsamplelocations)
	}
	#[inline(always)]
	fn glTextureStorage3DMultisample(&self, texture: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) {
		(self.version_4_5.texturestorage3dmultisample)(texture, samples, internalformat, width, height, depth, fixedsamplelocations)
	}
	#[inline(always)]
	fn glTextureSubImage1D(&self, texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) {
		(self.version_4_5.texturesubimage1d)(texture, level, xoffset, width, format, type_, pixels)
	}
	#[inline(always)]
	fn glTextureSubImage2D(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) {
		(self.version_4_5.texturesubimage2d)(texture, level, xoffset, yoffset, width, height, format, type_, pixels)
	}
	#[inline(always)]
	fn glTextureSubImage3D(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) {
		(self.version_4_5.texturesubimage3d)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, pixels)
	}
	#[inline(always)]
	fn glCompressedTextureSubImage1D(&self, texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) {
		(self.version_4_5.compressedtexturesubimage1d)(texture, level, xoffset, width, format, imageSize, data)
	}
	#[inline(always)]
	fn glCompressedTextureSubImage2D(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) {
		(self.version_4_5.compressedtexturesubimage2d)(texture, level, xoffset, yoffset, width, height, format, imageSize, data)
	}
	#[inline(always)]
	fn glCompressedTextureSubImage3D(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) {
		(self.version_4_5.compressedtexturesubimage3d)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, imageSize, data)
	}
	#[inline(always)]
	fn glCopyTextureSubImage1D(&self, texture: GLuint, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) {
		(self.version_4_5.copytexturesubimage1d)(texture, level, xoffset, x, y, width)
	}
	#[inline(always)]
	fn glCopyTextureSubImage2D(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
		(self.version_4_5.copytexturesubimage2d)(texture, level, xoffset, yoffset, x, y, width, height)
	}
	#[inline(always)]
	fn glCopyTextureSubImage3D(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
		(self.version_4_5.copytexturesubimage3d)(texture, level, xoffset, yoffset, zoffset, x, y, width, height)
	}
	#[inline(always)]
	fn glTextureParameterf(&self, texture: GLuint, pname: GLenum, param: GLfloat) {
		(self.version_4_5.textureparameterf)(texture, pname, param)
	}
	#[inline(always)]
	fn glTextureParameterfv(&self, texture: GLuint, pname: GLenum, param: *const GLfloat) {
		(self.version_4_5.textureparameterfv)(texture, pname, param)
	}
	#[inline(always)]
	fn glTextureParameteri(&self, texture: GLuint, pname: GLenum, param: GLint) {
		(self.version_4_5.textureparameteri)(texture, pname, param)
	}
	#[inline(always)]
	fn glTextureParameterIiv(&self, texture: GLuint, pname: GLenum, params: *const GLint) {
		(self.version_4_5.textureparameteriiv)(texture, pname, params)
	}
	#[inline(always)]
	fn glTextureParameterIuiv(&self, texture: GLuint, pname: GLenum, params: *const GLuint) {
		(self.version_4_5.textureparameteriuiv)(texture, pname, params)
	}
	#[inline(always)]
	fn glTextureParameteriv(&self, texture: GLuint, pname: GLenum, param: *const GLint) {
		(self.version_4_5.textureparameteriv)(texture, pname, param)
	}
	#[inline(always)]
	fn glGenerateTextureMipmap(&self, texture: GLuint) {
		(self.version_4_5.generatetexturemipmap)(texture)
	}
	#[inline(always)]
	fn glBindTextureUnit(&self, unit: GLuint, texture: GLuint) {
		(self.version_4_5.bindtextureunit)(unit, texture)
	}
	#[inline(always)]
	fn glGetTextureImage(&self, texture: GLuint, level: GLint, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut c_void) {
		(self.version_4_5.gettextureimage)(texture, level, format, type_, bufSize, pixels)
	}
	#[inline(always)]
	fn glGetCompressedTextureImage(&self, texture: GLuint, level: GLint, bufSize: GLsizei, pixels: *mut c_void) {
		(self.version_4_5.getcompressedtextureimage)(texture, level, bufSize, pixels)
	}
	#[inline(always)]
	fn glGetTextureLevelParameterfv(&self, texture: GLuint, level: GLint, pname: GLenum, params: *mut GLfloat) {
		(self.version_4_5.gettexturelevelparameterfv)(texture, level, pname, params)
	}
	#[inline(always)]
	fn glGetTextureLevelParameteriv(&self, texture: GLuint, level: GLint, pname: GLenum, params: *mut GLint) {
		(self.version_4_5.gettexturelevelparameteriv)(texture, level, pname, params)
	}
	#[inline(always)]
	fn glGetTextureParameterfv(&self, texture: GLuint, pname: GLenum, params: *mut GLfloat) {
		(self.version_4_5.gettextureparameterfv)(texture, pname, params)
	}
	#[inline(always)]
	fn glGetTextureParameterIiv(&self, texture: GLuint, pname: GLenum, params: *mut GLint) {
		(self.version_4_5.gettextureparameteriiv)(texture, pname, params)
	}
	#[inline(always)]
	fn glGetTextureParameterIuiv(&self, texture: GLuint, pname: GLenum, params: *mut GLuint) {
		(self.version_4_5.gettextureparameteriuiv)(texture, pname, params)
	}
	#[inline(always)]
	fn glGetTextureParameteriv(&self, texture: GLuint, pname: GLenum, params: *mut GLint) {
		(self.version_4_5.gettextureparameteriv)(texture, pname, params)
	}
	#[inline(always)]
	fn glCreateVertexArrays(&self, n: GLsizei, arrays: *mut GLuint) {
		(self.version_4_5.createvertexarrays)(n, arrays)
	}
	#[inline(always)]
	fn glDisableVertexArrayAttrib(&self, vaobj: GLuint, index: GLuint) {
		(self.version_4_5.disablevertexarrayattrib)(vaobj, index)
	}
	#[inline(always)]
	fn glEnableVertexArrayAttrib(&self, vaobj: GLuint, index: GLuint) {
		(self.version_4_5.enablevertexarrayattrib)(vaobj, index)
	}
	#[inline(always)]
	fn glVertexArrayElementBuffer(&self, vaobj: GLuint, buffer: GLuint) {
		(self.version_4_5.vertexarrayelementbuffer)(vaobj, buffer)
	}
	#[inline(always)]
	fn glVertexArrayVertexBuffer(&self, vaobj: GLuint, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei) {
		(self.version_4_5.vertexarrayvertexbuffer)(vaobj, bindingindex, buffer, offset, stride)
	}
	#[inline(always)]
	fn glVertexArrayVertexBuffers(&self, vaobj: GLuint, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei) {
		(self.version_4_5.vertexarrayvertexbuffers)(vaobj, first, count, buffers, offsets, strides)
	}
	#[inline(always)]
	fn glVertexArrayAttribBinding(&self, vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint) {
		(self.version_4_5.vertexarrayattribbinding)(vaobj, attribindex, bindingindex)
	}
	#[inline(always)]
	fn glVertexArrayAttribFormat(&self, vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint) {
		(self.version_4_5.vertexarrayattribformat)(vaobj, attribindex, size, type_, normalized, relativeoffset)
	}
	#[inline(always)]
	fn glVertexArrayAttribIFormat(&self, vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) {
		(self.version_4_5.vertexarrayattribiformat)(vaobj, attribindex, size, type_, relativeoffset)
	}
	#[inline(always)]
	fn glVertexArrayAttribLFormat(&self, vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) {
		(self.version_4_5.vertexarrayattriblformat)(vaobj, attribindex, size, type_, relativeoffset)
	}
	#[inline(always)]
	fn glVertexArrayBindingDivisor(&self, vaobj: GLuint, bindingindex: GLuint, divisor: GLuint) {
		(self.version_4_5.vertexarraybindingdivisor)(vaobj, bindingindex, divisor)
	}
	#[inline(always)]
	fn glGetVertexArrayiv(&self, vaobj: GLuint, pname: GLenum, param: *mut GLint) {
		(self.version_4_5.getvertexarrayiv)(vaobj, pname, param)
	}
	#[inline(always)]
	fn glGetVertexArrayIndexediv(&self, vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint) {
		(self.version_4_5.getvertexarrayindexediv)(vaobj, index, pname, param)
	}
	#[inline(always)]
	fn glGetVertexArrayIndexed64iv(&self, vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint64) {
		(self.version_4_5.getvertexarrayindexed64iv)(vaobj, index, pname, param)
	}
	#[inline(always)]
	fn glCreateSamplers(&self, n: GLsizei, samplers: *mut GLuint) {
		(self.version_4_5.createsamplers)(n, samplers)
	}
	#[inline(always)]
	fn glCreateProgramPipelines(&self, n: GLsizei, pipelines: *mut GLuint) {
		(self.version_4_5.createprogrampipelines)(n, pipelines)
	}
	#[inline(always)]
	fn glCreateQueries(&self, target: GLenum, n: GLsizei, ids: *mut GLuint) {
		(self.version_4_5.createqueries)(target, n, ids)
	}
	#[inline(always)]
	fn glGetQueryBufferObjecti64v(&self, id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr) {
		(self.version_4_5.getquerybufferobjecti64v)(id, buffer, pname, offset)
	}
	#[inline(always)]
	fn glGetQueryBufferObjectiv(&self, id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr) {
		(self.version_4_5.getquerybufferobjectiv)(id, buffer, pname, offset)
	}
	#[inline(always)]
	fn glGetQueryBufferObjectui64v(&self, id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr) {
		(self.version_4_5.getquerybufferobjectui64v)(id, buffer, pname, offset)
	}
	#[inline(always)]
	fn glGetQueryBufferObjectuiv(&self, id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr) {
		(self.version_4_5.getquerybufferobjectuiv)(id, buffer, pname, offset)
	}
	#[inline(always)]
	fn glMemoryBarrierByRegion(&self, barriers: GLbitfield) {
		(self.version_4_5.memorybarrierbyregion)(barriers)
	}
	#[inline(always)]
	fn glGetTextureSubImage(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut c_void) {
		(self.version_4_5.gettexturesubimage)(texture, level, xoffset, yoffset, zoffset, width, height, depth, format, type_, bufSize, pixels)
	}
	#[inline(always)]
	fn glGetCompressedTextureSubImage(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, bufSize: GLsizei, pixels: *mut c_void) {
		(self.version_4_5.getcompressedtexturesubimage)(texture, level, xoffset, yoffset, zoffset, width, height, depth, bufSize, pixels)
	}
	#[inline(always)]
	fn glGetGraphicsResetStatus(&self) -> GLenum {
		(self.version_4_5.getgraphicsresetstatus)()
	}
	#[inline(always)]
	fn glGetnCompressedTexImage(&self, target: GLenum, lod: GLint, bufSize: GLsizei, pixels: *mut c_void) {
		(self.version_4_5.getncompressedteximage)(target, lod, bufSize, pixels)
	}
	#[inline(always)]
	fn glGetnTexImage(&self, target: GLenum, level: GLint, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut c_void) {
		(self.version_4_5.getnteximage)(target, level, format, type_, bufSize, pixels)
	}
	#[inline(always)]
	fn glGetnUniformdv(&self, program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLdouble) {
		(self.version_4_5.getnuniformdv)(program, location, bufSize, params)
	}
	#[inline(always)]
	fn glGetnUniformfv(&self, program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLfloat) {
		(self.version_4_5.getnuniformfv)(program, location, bufSize, params)
	}
	#[inline(always)]
	fn glGetnUniformiv(&self, program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint) {
		(self.version_4_5.getnuniformiv)(program, location, bufSize, params)
	}
	#[inline(always)]
	fn glGetnUniformuiv(&self, program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint) {
		(self.version_4_5.getnuniformuiv)(program, location, bufSize, params)
	}
	#[inline(always)]
	fn glReadnPixels(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, bufSize: GLsizei, data: *mut c_void) {
		(self.version_4_5.readnpixels)(x, y, width, height, format, type_, bufSize, data)
	}
	#[inline(always)]
	fn glGetnMapdv(&self, target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLdouble) {
		(self.version_4_5.getnmapdv)(target, query, bufSize, v)
	}
	#[inline(always)]
	fn glGetnMapfv(&self, target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLfloat) {
		(self.version_4_5.getnmapfv)(target, query, bufSize, v)
	}
	#[inline(always)]
	fn glGetnMapiv(&self, target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLint) {
		(self.version_4_5.getnmapiv)(target, query, bufSize, v)
	}
	#[inline(always)]
	fn glGetnPixelMapfv(&self, map: GLenum, bufSize: GLsizei, values: *mut GLfloat) {
		(self.version_4_5.getnpixelmapfv)(map, bufSize, values)
	}
	#[inline(always)]
	fn glGetnPixelMapuiv(&self, map: GLenum, bufSize: GLsizei, values: *mut GLuint) {
		(self.version_4_5.getnpixelmapuiv)(map, bufSize, values)
	}
	#[inline(always)]
	fn glGetnPixelMapusv(&self, map: GLenum, bufSize: GLsizei, values: *mut GLushort) {
		(self.version_4_5.getnpixelmapusv)(map, bufSize, values)
	}
	#[inline(always)]
	fn glGetnPolygonStipple(&self, bufSize: GLsizei, pattern: *mut GLubyte) {
		(self.version_4_5.getnpolygonstipple)(bufSize, pattern)
	}
	#[inline(always)]
	fn glGetnColorTable(&self, target: GLenum, format: GLenum, type_: GLenum, bufSize: GLsizei, table: *mut c_void) {
		(self.version_4_5.getncolortable)(target, format, type_, bufSize, table)
	}
	#[inline(always)]
	fn glGetnConvolutionFilter(&self, target: GLenum, format: GLenum, type_: GLenum, bufSize: GLsizei, image: *mut c_void) {
		(self.version_4_5.getnconvolutionfilter)(target, format, type_, bufSize, image)
	}
	#[inline(always)]
	fn glGetnSeparableFilter(&self, target: GLenum, format: GLenum, type_: GLenum, rowBufSize: GLsizei, row: *mut c_void, columnBufSize: GLsizei, column: *mut c_void, span: *mut c_void) {
		(self.version_4_5.getnseparablefilter)(target, format, type_, rowBufSize, row, columnBufSize, column, span)
	}
	#[inline(always)]
	fn glGetnHistogram(&self, target: GLenum, reset: GLboolean, format: GLenum, type_: GLenum, bufSize: GLsizei, values: *mut c_void) {
		(self.version_4_5.getnhistogram)(target, reset, format, type_, bufSize, values)
	}
	#[inline(always)]
	fn glGetnMinmax(&self, target: GLenum, reset: GLboolean, format: GLenum, type_: GLenum, bufSize: GLsizei, values: *mut c_void) {
		(self.version_4_5.getnminmax)(target, reset, format, type_, bufSize, values)
	}
	#[inline(always)]
	fn glTextureBarrier(&self) {
		(self.version_4_5.texturebarrier)()
	}
}

impl GL_4_6 for GLCore {
	#[inline(always)]
	fn glSpecializeShader(&self, shader: GLuint, pEntryPoint: *const GLchar, numSpecializationConstants: GLuint, pConstantIndex: *const GLuint, pConstantValue: *const GLuint) {
		(self.version_4_6.specializeshader)(shader, pEntryPoint, numSpecializationConstants, pConstantIndex, pConstantValue)
	}
	#[inline(always)]
	fn glMultiDrawArraysIndirectCount(&self, mode: GLenum, indirect: *const c_void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei) {
		(self.version_4_6.multidrawarraysindirectcount)(mode, indirect, drawcount, maxdrawcount, stride)
	}
	#[inline(always)]
	fn glMultiDrawElementsIndirectCount(&self, mode: GLenum, type_: GLenum, indirect: *const c_void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei) {
		(self.version_4_6.multidrawelementsindirectcount)(mode, type_, indirect, drawcount, maxdrawcount, stride)
	}
	#[inline(always)]
	fn glPolygonOffsetClamp(&self, factor: GLfloat, units: GLfloat, clamp: GLfloat) {
		(self.version_4_6.polygonoffsetclamp)(factor, units, clamp)
	}
}

impl GLCore {
	pub fn new(get_proc_address: &impl Fn(&'static str) -> *const c_void) -> Self {
		let version_1_0 = Version10::new(get_proc_address);
		if !version_1_0.available {
			return Self::default();
		}
		Self {
			version_1_0,
			version_1_1: Version11::new(version_1_0, get_proc_address),
			version_1_2: Version12::new(version_1_0, get_proc_address),
			version_1_3: Version13::new(version_1_0, get_proc_address),
			version_1_4: Version14::new(version_1_0, get_proc_address),
			version_1_5: Version15::new(version_1_0, get_proc_address),
			version_2_0: Version20::new(version_1_0, get_proc_address),
			version_2_1: Version21::new(version_1_0, get_proc_address),
			version_3_0: Version30::new(version_1_0, get_proc_address),
			version_3_1: Version31::new(version_1_0, get_proc_address),
			version_3_2: Version32::new(version_1_0, get_proc_address),
			version_3_3: Version33::new(version_1_0, get_proc_address),
			version_4_0: Version40::new(version_1_0, get_proc_address),
			version_4_1: Version41::new(version_1_0, get_proc_address),
			version_4_2: Version42::new(version_1_0, get_proc_address),
			version_4_3: Version43::new(version_1_0, get_proc_address),
			version_4_4: Version44::new(version_1_0, get_proc_address),
			version_4_5: Version45::new(version_1_0, get_proc_address),
			version_4_6: Version46::new(version_1_0, get_proc_address),
		}
	}
}

impl Default for GLCore {
	fn default() -> Self {
		Self {
			version_1_0: Version10::default(),
			version_1_1: Version11::default(),
			version_1_2: Version12::default(),
			version_1_3: Version13::default(),
			version_1_4: Version14::default(),
			version_1_5: Version15::default(),
			version_2_0: Version20::default(),
			version_2_1: Version21::default(),
			version_3_0: Version30::default(),
			version_3_1: Version31::default(),
			version_3_2: Version32::default(),
			version_3_3: Version33::default(),
			version_4_0: Version40::default(),
			version_4_1: Version41::default(),
			version_4_2: Version42::default(),
			version_4_3: Version43::default(),
			version_4_4: Version44::default(),
			version_4_5: Version45::default(),
			version_4_6: Version46::default(),
		}
	}
}

pub trait GL: Debug + Clone + Copy + Sized +
GL_1_0 +
GL_1_1 +
GL_1_2 +
GL_1_3 +
GL_1_4 +
GL_1_5 +
GL_2_0 +
GL_2_1 +
GL_3_0 +
GL_3_1 +
GL_3_2 +
GL_3_3 +
GL_4_0 +
GL_4_1 +
GL_4_2 +
GL_4_3 +
GL_4_4 +
GL_4_5 +
GL_4_6 {}

impl GL for GLCore {}

