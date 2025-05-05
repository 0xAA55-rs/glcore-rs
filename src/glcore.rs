#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
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
extern "system" fn dummy_pfnglcullfaceproc (_: GLenum) {
	panic!("OpenGL Function pointer of `glCullFace()` is NULL");
}
extern "system" fn dummy_pfnglfrontfaceproc (_: GLenum) {
	panic!("OpenGL Function pointer of `glFrontFace()` is NULL");
}
extern "system" fn dummy_pfnglhintproc (_: GLenum, _: GLenum) {
	panic!("OpenGL Function pointer of `glHint()` is NULL");
}
extern "system" fn dummy_pfngllinewidthproc (_: GLfloat) {
	panic!("OpenGL Function pointer of `glLineWidth()` is NULL");
}
extern "system" fn dummy_pfnglpointsizeproc (_: GLfloat) {
	panic!("OpenGL Function pointer of `glPointSize()` is NULL");
}
extern "system" fn dummy_pfnglpolygonmodeproc (_: GLenum, _: GLenum) {
	panic!("OpenGL Function pointer of `glPolygonMode()` is NULL");
}
extern "system" fn dummy_pfnglscissorproc (_: GLint, _: GLint, _: GLsizei, _: GLsizei) {
	panic!("OpenGL Function pointer of `glScissor()` is NULL");
}
extern "system" fn dummy_pfngltexparameterfproc (_: GLenum, _: GLenum, _: GLfloat) {
	panic!("OpenGL Function pointer of `glTexParameterf()` is NULL");
}
extern "system" fn dummy_pfngltexparameterfvproc (_: GLenum, _: GLenum, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glTexParameterfv()` is NULL");
}
extern "system" fn dummy_pfngltexparameteriproc (_: GLenum, _: GLenum, _: GLint) {
	panic!("OpenGL Function pointer of `glTexParameteri()` is NULL");
}
extern "system" fn dummy_pfngltexparameterivproc (_: GLenum, _: GLenum, _: *const GLint) {
	panic!("OpenGL Function pointer of `glTexParameteriv()` is NULL");
}
extern "system" fn dummy_pfnglteximage1dproc (_: GLenum, _: GLint, _: GLint, _: GLsizei, _: GLint, _: GLenum, _: GLenum, _: *const c_void) {
	panic!("OpenGL Function pointer of `glTexImage1D()` is NULL");
}
extern "system" fn dummy_pfnglteximage2dproc (_: GLenum, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLint, _: GLenum, _: GLenum, _: *const c_void) {
	panic!("OpenGL Function pointer of `glTexImage2D()` is NULL");
}
extern "system" fn dummy_pfngldrawbufferproc (_: GLenum) {
	panic!("OpenGL Function pointer of `glDrawBuffer()` is NULL");
}
extern "system" fn dummy_pfnglclearproc (_: GLbitfield) {
	panic!("OpenGL Function pointer of `glClear()` is NULL");
}
extern "system" fn dummy_pfnglclearcolorproc (_: GLfloat, _: GLfloat, _: GLfloat, _: GLfloat) {
	panic!("OpenGL Function pointer of `glClearColor()` is NULL");
}
extern "system" fn dummy_pfnglclearstencilproc (_: GLint) {
	panic!("OpenGL Function pointer of `glClearStencil()` is NULL");
}
extern "system" fn dummy_pfnglcleardepthproc (_: GLdouble) {
	panic!("OpenGL Function pointer of `glClearDepth()` is NULL");
}
extern "system" fn dummy_pfnglstencilmaskproc (_: GLuint) {
	panic!("OpenGL Function pointer of `glStencilMask()` is NULL");
}
extern "system" fn dummy_pfnglcolormaskproc (_: GLboolean, _: GLboolean, _: GLboolean, _: GLboolean) {
	panic!("OpenGL Function pointer of `glColorMask()` is NULL");
}
extern "system" fn dummy_pfngldepthmaskproc (_: GLboolean) {
	panic!("OpenGL Function pointer of `glDepthMask()` is NULL");
}
extern "system" fn dummy_pfngldisableproc (_: GLenum) {
	panic!("OpenGL Function pointer of `glDisable()` is NULL");
}
extern "system" fn dummy_pfnglenableproc (_: GLenum) {
	panic!("OpenGL Function pointer of `glEnable()` is NULL");
}
extern "system" fn dummy_pfnglfinishproc () {
	panic!("OpenGL Function pointer of `glFinish()` is NULL");
}
extern "system" fn dummy_pfnglflushproc () {
	panic!("OpenGL Function pointer of `glFlush()` is NULL");
}
extern "system" fn dummy_pfnglblendfuncproc (_: GLenum, _: GLenum) {
	panic!("OpenGL Function pointer of `glBlendFunc()` is NULL");
}
extern "system" fn dummy_pfngllogicopproc (_: GLenum) {
	panic!("OpenGL Function pointer of `glLogicOp()` is NULL");
}
extern "system" fn dummy_pfnglstencilfuncproc (_: GLenum, _: GLint, _: GLuint) {
	panic!("OpenGL Function pointer of `glStencilFunc()` is NULL");
}
extern "system" fn dummy_pfnglstencilopproc (_: GLenum, _: GLenum, _: GLenum) {
	panic!("OpenGL Function pointer of `glStencilOp()` is NULL");
}
extern "system" fn dummy_pfngldepthfuncproc (_: GLenum) {
	panic!("OpenGL Function pointer of `glDepthFunc()` is NULL");
}
extern "system" fn dummy_pfnglpixelstorefproc (_: GLenum, _: GLfloat) {
	panic!("OpenGL Function pointer of `glPixelStoref()` is NULL");
}
extern "system" fn dummy_pfnglpixelstoreiproc (_: GLenum, _: GLint) {
	panic!("OpenGL Function pointer of `glPixelStorei()` is NULL");
}
extern "system" fn dummy_pfnglreadbufferproc (_: GLenum) {
	panic!("OpenGL Function pointer of `glReadBuffer()` is NULL");
}
extern "system" fn dummy_pfnglreadpixelsproc (_: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLenum, _: GLenum, _: *mut c_void) {
	panic!("OpenGL Function pointer of `glReadPixels()` is NULL");
}
extern "system" fn dummy_pfnglgetbooleanvproc (_: GLenum, _: *mut GLboolean) {
	panic!("OpenGL Function pointer of `glGetBooleanv()` is NULL");
}
extern "system" fn dummy_pfnglgetdoublevproc (_: GLenum, _: *mut GLdouble) {
	panic!("OpenGL Function pointer of `glGetDoublev()` is NULL");
}
extern "system" fn dummy_pfnglgeterrorproc () -> GLenum {
	panic!("OpenGL Function pointer of `glGetError()` is NULL");
}
extern "system" fn dummy_pfnglgetfloatvproc (_: GLenum, _: *mut GLfloat) {
	panic!("OpenGL Function pointer of `glGetFloatv()` is NULL");
}
extern "system" fn dummy_pfnglgetintegervproc (_: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetIntegerv()` is NULL");
}
extern "system" fn dummy_pfnglgetstringproc (_: GLenum) -> *const GLubyte {
	panic!("OpenGL Function pointer of `glGetString()` is NULL");
}
extern "system" fn dummy_pfnglgetteximageproc (_: GLenum, _: GLint, _: GLenum, _: GLenum, _: *mut c_void) {
	panic!("OpenGL Function pointer of `glGetTexImage()` is NULL");
}
extern "system" fn dummy_pfnglgettexparameterfvproc (_: GLenum, _: GLenum, _: *mut GLfloat) {
	panic!("OpenGL Function pointer of `glGetTexParameterfv()` is NULL");
}
extern "system" fn dummy_pfnglgettexparameterivproc (_: GLenum, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetTexParameteriv()` is NULL");
}
extern "system" fn dummy_pfnglgettexlevelparameterfvproc (_: GLenum, _: GLint, _: GLenum, _: *mut GLfloat) {
	panic!("OpenGL Function pointer of `glGetTexLevelParameterfv()` is NULL");
}
extern "system" fn dummy_pfnglgettexlevelparameterivproc (_: GLenum, _: GLint, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetTexLevelParameteriv()` is NULL");
}
extern "system" fn dummy_pfnglisenabledproc (_: GLenum) -> GLboolean {
	panic!("OpenGL Function pointer of `glIsEnabled()` is NULL");
}
extern "system" fn dummy_pfngldepthrangeproc (_: GLdouble, _: GLdouble) {
	panic!("OpenGL Function pointer of `glDepthRange()` is NULL");
}
extern "system" fn dummy_pfnglviewportproc (_: GLint, _: GLint, _: GLsizei, _: GLsizei) {
	panic!("OpenGL Function pointer of `glViewport()` is NULL");
}
pub const GL_DEPTH_BUFFER_BIT: GLbitfield = 0x00000100;
pub const GL_STENCIL_BUFFER_BIT: GLbitfield = 0x00000400;
pub const GL_COLOR_BUFFER_BIT: GLbitfield = 0x00004000;
pub const GL_FALSE: GLenum = 0;
pub const GL_TRUE: GLenum = 1;
pub const GL_POINTS: GLenum = 0x0000;
pub const GL_LINES: GLenum = 0x0001;
pub const GL_LINE_LOOP: GLenum = 0x0002;
pub const GL_LINE_STRIP: GLenum = 0x0003;
pub const GL_TRIANGLES: GLenum = 0x0004;
pub const GL_TRIANGLE_STRIP: GLenum = 0x0005;
pub const GL_TRIANGLE_FAN: GLenum = 0x0006;
pub const GL_QUADS: GLenum = 0x0007;
pub const GL_NEVER: GLenum = 0x0200;
pub const GL_LESS: GLenum = 0x0201;
pub const GL_EQUAL: GLenum = 0x0202;
pub const GL_LEQUAL: GLenum = 0x0203;
pub const GL_GREATER: GLenum = 0x0204;
pub const GL_NOTEQUAL: GLenum = 0x0205;
pub const GL_GEQUAL: GLenum = 0x0206;
pub const GL_ALWAYS: GLenum = 0x0207;
pub const GL_ZERO: GLenum = 0;
pub const GL_ONE: GLenum = 1;
pub const GL_SRC_COLOR: GLenum = 0x0300;
pub const GL_ONE_MINUS_SRC_COLOR: GLenum = 0x0301;
pub const GL_SRC_ALPHA: GLenum = 0x0302;
pub const GL_ONE_MINUS_SRC_ALPHA: GLenum = 0x0303;
pub const GL_DST_ALPHA: GLenum = 0x0304;
pub const GL_ONE_MINUS_DST_ALPHA: GLenum = 0x0305;
pub const GL_DST_COLOR: GLenum = 0x0306;
pub const GL_ONE_MINUS_DST_COLOR: GLenum = 0x0307;
pub const GL_SRC_ALPHA_SATURATE: GLenum = 0x0308;
pub const GL_NONE: GLenum = 0;
pub const GL_FRONT_LEFT: GLenum = 0x0400;
pub const GL_FRONT_RIGHT: GLenum = 0x0401;
pub const GL_BACK_LEFT: GLenum = 0x0402;
pub const GL_BACK_RIGHT: GLenum = 0x0403;
pub const GL_FRONT: GLenum = 0x0404;
pub const GL_BACK: GLenum = 0x0405;
pub const GL_LEFT: GLenum = 0x0406;
pub const GL_RIGHT: GLenum = 0x0407;
pub const GL_FRONT_AND_BACK: GLenum = 0x0408;
pub const GL_NO_ERROR: GLenum = 0;
pub const GL_INVALID_ENUM: GLenum = 0x0500;
pub const GL_INVALID_VALUE: GLenum = 0x0501;
pub const GL_INVALID_OPERATION: GLenum = 0x0502;
pub const GL_OUT_OF_MEMORY: GLenum = 0x0505;
pub const GL_CW: GLenum = 0x0900;
pub const GL_CCW: GLenum = 0x0901;
pub const GL_POINT_SIZE: GLenum = 0x0B11;
pub const GL_POINT_SIZE_RANGE: GLenum = 0x0B12;
pub const GL_POINT_SIZE_GRANULARITY: GLenum = 0x0B13;
pub const GL_LINE_SMOOTH: GLenum = 0x0B20;
pub const GL_LINE_WIDTH: GLenum = 0x0B21;
pub const GL_LINE_WIDTH_RANGE: GLenum = 0x0B22;
pub const GL_LINE_WIDTH_GRANULARITY: GLenum = 0x0B23;
pub const GL_POLYGON_MODE: GLenum = 0x0B40;
pub const GL_POLYGON_SMOOTH: GLenum = 0x0B41;
pub const GL_CULL_FACE: GLenum = 0x0B44;
pub const GL_CULL_FACE_MODE: GLenum = 0x0B45;
pub const GL_FRONT_FACE: GLenum = 0x0B46;
pub const GL_DEPTH_RANGE: GLenum = 0x0B70;
pub const GL_DEPTH_TEST: GLenum = 0x0B71;
pub const GL_DEPTH_WRITEMASK: GLenum = 0x0B72;
pub const GL_DEPTH_CLEAR_VALUE: GLenum = 0x0B73;
pub const GL_DEPTH_FUNC: GLenum = 0x0B74;
pub const GL_STENCIL_TEST: GLenum = 0x0B90;
pub const GL_STENCIL_CLEAR_VALUE: GLenum = 0x0B91;
pub const GL_STENCIL_FUNC: GLenum = 0x0B92;
pub const GL_STENCIL_VALUE_MASK: GLenum = 0x0B93;
pub const GL_STENCIL_FAIL: GLenum = 0x0B94;
pub const GL_STENCIL_PASS_DEPTH_FAIL: GLenum = 0x0B95;
pub const GL_STENCIL_PASS_DEPTH_PASS: GLenum = 0x0B96;
pub const GL_STENCIL_REF: GLenum = 0x0B97;
pub const GL_STENCIL_WRITEMASK: GLenum = 0x0B98;
pub const GL_VIEWPORT: GLenum = 0x0BA2;
pub const GL_DITHER: GLenum = 0x0BD0;
pub const GL_BLEND_DST: GLenum = 0x0BE0;
pub const GL_BLEND_SRC: GLenum = 0x0BE1;
pub const GL_BLEND: GLenum = 0x0BE2;
pub const GL_LOGIC_OP_MODE: GLenum = 0x0BF0;
pub const GL_DRAW_BUFFER: GLenum = 0x0C01;
pub const GL_READ_BUFFER: GLenum = 0x0C02;
pub const GL_SCISSOR_BOX: GLenum = 0x0C10;
pub const GL_SCISSOR_TEST: GLenum = 0x0C11;
pub const GL_COLOR_CLEAR_VALUE: GLenum = 0x0C22;
pub const GL_COLOR_WRITEMASK: GLenum = 0x0C23;
pub const GL_DOUBLEBUFFER: GLenum = 0x0C32;
pub const GL_STEREO: GLenum = 0x0C33;
pub const GL_LINE_SMOOTH_HINT: GLenum = 0x0C52;
pub const GL_POLYGON_SMOOTH_HINT: GLenum = 0x0C53;
pub const GL_UNPACK_SWAP_BYTES: GLenum = 0x0CF0;
pub const GL_UNPACK_LSB_FIRST: GLenum = 0x0CF1;
pub const GL_UNPACK_ROW_LENGTH: GLenum = 0x0CF2;
pub const GL_UNPACK_SKIP_ROWS: GLenum = 0x0CF3;
pub const GL_UNPACK_SKIP_PIXELS: GLenum = 0x0CF4;
pub const GL_UNPACK_ALIGNMENT: GLenum = 0x0CF5;
pub const GL_PACK_SWAP_BYTES: GLenum = 0x0D00;
pub const GL_PACK_LSB_FIRST: GLenum = 0x0D01;
pub const GL_PACK_ROW_LENGTH: GLenum = 0x0D02;
pub const GL_PACK_SKIP_ROWS: GLenum = 0x0D03;
pub const GL_PACK_SKIP_PIXELS: GLenum = 0x0D04;
pub const GL_PACK_ALIGNMENT: GLenum = 0x0D05;
pub const GL_MAX_TEXTURE_SIZE: GLenum = 0x0D33;
pub const GL_MAX_VIEWPORT_DIMS: GLenum = 0x0D3A;
pub const GL_SUBPIXEL_BITS: GLenum = 0x0D50;
pub const GL_TEXTURE_1D: GLenum = 0x0DE0;
pub const GL_TEXTURE_2D: GLenum = 0x0DE1;
pub const GL_TEXTURE_WIDTH: GLenum = 0x1000;
pub const GL_TEXTURE_HEIGHT: GLenum = 0x1001;
pub const GL_TEXTURE_BORDER_COLOR: GLenum = 0x1004;
pub const GL_DONT_CARE: GLenum = 0x1100;
pub const GL_FASTEST: GLenum = 0x1101;
pub const GL_NICEST: GLenum = 0x1102;
pub const GL_BYTE: GLenum = 0x1400;
pub const GL_UNSIGNED_BYTE: GLenum = 0x1401;
pub const GL_SHORT: GLenum = 0x1402;
pub const GL_UNSIGNED_SHORT: GLenum = 0x1403;
pub const GL_INT: GLenum = 0x1404;
pub const GL_UNSIGNED_INT: GLenum = 0x1405;
pub const GL_FLOAT: GLenum = 0x1406;
pub const GL_STACK_OVERFLOW: GLenum = 0x0503;
pub const GL_STACK_UNDERFLOW: GLenum = 0x0504;
pub const GL_CLEAR: GLenum = 0x1500;
pub const GL_AND: GLenum = 0x1501;
pub const GL_AND_REVERSE: GLenum = 0x1502;
pub const GL_COPY: GLenum = 0x1503;
pub const GL_AND_INVERTED: GLenum = 0x1504;
pub const GL_NOOP: GLenum = 0x1505;
pub const GL_XOR: GLenum = 0x1506;
pub const GL_OR: GLenum = 0x1507;
pub const GL_NOR: GLenum = 0x1508;
pub const GL_EQUIV: GLenum = 0x1509;
pub const GL_INVERT: GLenum = 0x150A;
pub const GL_OR_REVERSE: GLenum = 0x150B;
pub const GL_COPY_INVERTED: GLenum = 0x150C;
pub const GL_OR_INVERTED: GLenum = 0x150D;
pub const GL_NAND: GLenum = 0x150E;
pub const GL_SET: GLenum = 0x150F;
pub const GL_TEXTURE: GLenum = 0x1702;
pub const GL_COLOR: GLenum = 0x1800;
pub const GL_DEPTH: GLenum = 0x1801;
pub const GL_STENCIL: GLenum = 0x1802;
pub const GL_STENCIL_INDEX: GLenum = 0x1901;
pub const GL_DEPTH_COMPONENT: GLenum = 0x1902;
pub const GL_RED: GLenum = 0x1903;
pub const GL_GREEN: GLenum = 0x1904;
pub const GL_BLUE: GLenum = 0x1905;
pub const GL_ALPHA: GLenum = 0x1906;
pub const GL_RGB: GLenum = 0x1907;
pub const GL_RGBA: GLenum = 0x1908;
pub const GL_POINT: GLenum = 0x1B00;
pub const GL_LINE: GLenum = 0x1B01;
pub const GL_FILL: GLenum = 0x1B02;
pub const GL_KEEP: GLenum = 0x1E00;
pub const GL_REPLACE: GLenum = 0x1E01;
pub const GL_INCR: GLenum = 0x1E02;
pub const GL_DECR: GLenum = 0x1E03;
pub const GL_VENDOR: GLenum = 0x1F00;
pub const GL_RENDERER: GLenum = 0x1F01;
pub const GL_VERSION: GLenum = 0x1F02;
pub const GL_EXTENSIONS: GLenum = 0x1F03;
pub const GL_NEAREST: GLint = 0x2600;
pub const GL_LINEAR: GLint = 0x2601;
pub const GL_NEAREST_MIPMAP_NEAREST: GLint = 0x2700;
pub const GL_LINEAR_MIPMAP_NEAREST: GLint = 0x2701;
pub const GL_NEAREST_MIPMAP_LINEAR: GLint = 0x2702;
pub const GL_LINEAR_MIPMAP_LINEAR: GLint = 0x2703;
pub const GL_TEXTURE_MAG_FILTER: GLenum = 0x2800;
pub const GL_TEXTURE_MIN_FILTER: GLenum = 0x2801;
pub const GL_TEXTURE_WRAP_S: GLenum = 0x2802;
pub const GL_TEXTURE_WRAP_T: GLenum = 0x2803;
pub const GL_REPEAT: GLint = 0x2901;

pub trait GL_1_0 {
	fn glCullFace(&self, mode: GLenum);
	fn glFrontFace(&self, mode: GLenum);
	fn glHint(&self, target: GLenum, mode: GLenum);
	fn glLineWidth(&self, width: GLfloat);
	fn glPointSize(&self, size: GLfloat);
	fn glPolygonMode(&self, face: GLenum, mode: GLenum);
	fn glScissor(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
	fn glTexParameterf(&self, target: GLenum, pname: GLenum, param: GLfloat);
	fn glTexParameterfv(&self, target: GLenum, pname: GLenum, params: *const GLfloat);
	fn glTexParameteri(&self, target: GLenum, pname: GLenum, param: GLint);
	fn glTexParameteriv(&self, target: GLenum, pname: GLenum, params: *const GLint);
	fn glTexImage1D(&self, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void);
	fn glTexImage2D(&self, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void);
	fn glDrawBuffer(&self, buf: GLenum);
	fn glClear(&self, mask: GLbitfield);
	fn glClearColor(&self, red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
	fn glClearStencil(&self, s: GLint);
	fn glClearDepth(&self, depth: GLdouble);
	fn glStencilMask(&self, mask: GLuint);
	fn glColorMask(&self, red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);
	fn glDepthMask(&self, flag: GLboolean);
	fn glDisable(&self, cap: GLenum);
	fn glEnable(&self, cap: GLenum);
	fn glFinish(&self);
	fn glFlush(&self);
	fn glBlendFunc(&self, sfactor: GLenum, dfactor: GLenum);
	fn glLogicOp(&self, opcode: GLenum);
	fn glStencilFunc(&self, func: GLenum, ref_: GLint, mask: GLuint);
	fn glStencilOp(&self, fail: GLenum, zfail: GLenum, zpass: GLenum);
	fn glDepthFunc(&self, func: GLenum);
	fn glPixelStoref(&self, pname: GLenum, param: GLfloat);
	fn glPixelStorei(&self, pname: GLenum, param: GLint);
	fn glReadBuffer(&self, src: GLenum);
	fn glReadPixels(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *mut c_void);
	fn glGetBooleanv(&self, pname: GLenum, data: *mut GLboolean);
	fn glGetDoublev(&self, pname: GLenum, data: *mut GLdouble);
	fn glGetError(&self) -> GLenum;
	fn glGetFloatv(&self, pname: GLenum, data: *mut GLfloat);
	fn glGetIntegerv(&self, pname: GLenum, data: *mut GLint);
	fn glGetString(&self, name: GLenum) -> *const GLubyte;
	fn glGetTexImage(&self, target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *mut c_void);
	fn glGetTexParameterfv(&self, target: GLenum, pname: GLenum, params: *mut GLfloat);
	fn glGetTexParameteriv(&self, target: GLenum, pname: GLenum, params: *mut GLint);
	fn glGetTexLevelParameterfv(&self, target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat);
	fn glGetTexLevelParameteriv(&self, target: GLenum, level: GLint, pname: GLenum, params: *mut GLint);
	fn glIsEnabled(&self, cap: GLenum) -> GLboolean;
	fn glDepthRange(&self, n: GLdouble, f: GLdouble);
	fn glViewport(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
	fn get_version(&self) -> (&'static str, u32, u32, u32);
	fn get_vendor(&self) -> &'static str;
	fn get_renderer(&self) -> &'static str;
	fn get_versionstr(&self) -> &'static str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
	pub fn new(mut get_proc_address: impl FnMut(&'static str) -> *const c_void) -> Self {
		let mut ret = Self {
			available: true,
			spec: "unknown",
			major_version: 0,
			minor_version: 0,
			release_version: 0,
			vendor: "unknown",
			renderer: "unknown",
			version: "unknown",
			cullface: {let proc = get_proc_address("glCullFace"); if proc == null() {dummy_pfnglcullfaceproc} else {unsafe{transmute(proc)}}},
			frontface: {let proc = get_proc_address("glFrontFace"); if proc == null() {dummy_pfnglfrontfaceproc} else {unsafe{transmute(proc)}}},
			hint: {let proc = get_proc_address("glHint"); if proc == null() {dummy_pfnglhintproc} else {unsafe{transmute(proc)}}},
			linewidth: {let proc = get_proc_address("glLineWidth"); if proc == null() {dummy_pfngllinewidthproc} else {unsafe{transmute(proc)}}},
			pointsize: {let proc = get_proc_address("glPointSize"); if proc == null() {dummy_pfnglpointsizeproc} else {unsafe{transmute(proc)}}},
			polygonmode: {let proc = get_proc_address("glPolygonMode"); if proc == null() {dummy_pfnglpolygonmodeproc} else {unsafe{transmute(proc)}}},
			scissor: {let proc = get_proc_address("glScissor"); if proc == null() {dummy_pfnglscissorproc} else {unsafe{transmute(proc)}}},
			texparameterf: {let proc = get_proc_address("glTexParameterf"); if proc == null() {dummy_pfngltexparameterfproc} else {unsafe{transmute(proc)}}},
			texparameterfv: {let proc = get_proc_address("glTexParameterfv"); if proc == null() {dummy_pfngltexparameterfvproc} else {unsafe{transmute(proc)}}},
			texparameteri: {let proc = get_proc_address("glTexParameteri"); if proc == null() {dummy_pfngltexparameteriproc} else {unsafe{transmute(proc)}}},
			texparameteriv: {let proc = get_proc_address("glTexParameteriv"); if proc == null() {dummy_pfngltexparameterivproc} else {unsafe{transmute(proc)}}},
			teximage1d: {let proc = get_proc_address("glTexImage1D"); if proc == null() {dummy_pfnglteximage1dproc} else {unsafe{transmute(proc)}}},
			teximage2d: {let proc = get_proc_address("glTexImage2D"); if proc == null() {dummy_pfnglteximage2dproc} else {unsafe{transmute(proc)}}},
			drawbuffer: {let proc = get_proc_address("glDrawBuffer"); if proc == null() {dummy_pfngldrawbufferproc} else {unsafe{transmute(proc)}}},
			clear: {let proc = get_proc_address("glClear"); if proc == null() {dummy_pfnglclearproc} else {unsafe{transmute(proc)}}},
			clearcolor: {let proc = get_proc_address("glClearColor"); if proc == null() {dummy_pfnglclearcolorproc} else {unsafe{transmute(proc)}}},
			clearstencil: {let proc = get_proc_address("glClearStencil"); if proc == null() {dummy_pfnglclearstencilproc} else {unsafe{transmute(proc)}}},
			cleardepth: {let proc = get_proc_address("glClearDepth"); if proc == null() {dummy_pfnglcleardepthproc} else {unsafe{transmute(proc)}}},
			stencilmask: {let proc = get_proc_address("glStencilMask"); if proc == null() {dummy_pfnglstencilmaskproc} else {unsafe{transmute(proc)}}},
			colormask: {let proc = get_proc_address("glColorMask"); if proc == null() {dummy_pfnglcolormaskproc} else {unsafe{transmute(proc)}}},
			depthmask: {let proc = get_proc_address("glDepthMask"); if proc == null() {dummy_pfngldepthmaskproc} else {unsafe{transmute(proc)}}},
			disable: {let proc = get_proc_address("glDisable"); if proc == null() {dummy_pfngldisableproc} else {unsafe{transmute(proc)}}},
			enable: {let proc = get_proc_address("glEnable"); if proc == null() {dummy_pfnglenableproc} else {unsafe{transmute(proc)}}},
			finish: {let proc = get_proc_address("glFinish"); if proc == null() {dummy_pfnglfinishproc} else {unsafe{transmute(proc)}}},
			flush: {let proc = get_proc_address("glFlush"); if proc == null() {dummy_pfnglflushproc} else {unsafe{transmute(proc)}}},
			blendfunc: {let proc = get_proc_address("glBlendFunc"); if proc == null() {dummy_pfnglblendfuncproc} else {unsafe{transmute(proc)}}},
			logicop: {let proc = get_proc_address("glLogicOp"); if proc == null() {dummy_pfngllogicopproc} else {unsafe{transmute(proc)}}},
			stencilfunc: {let proc = get_proc_address("glStencilFunc"); if proc == null() {dummy_pfnglstencilfuncproc} else {unsafe{transmute(proc)}}},
			stencilop: {let proc = get_proc_address("glStencilOp"); if proc == null() {dummy_pfnglstencilopproc} else {unsafe{transmute(proc)}}},
			depthfunc: {let proc = get_proc_address("glDepthFunc"); if proc == null() {dummy_pfngldepthfuncproc} else {unsafe{transmute(proc)}}},
			pixelstoref: {let proc = get_proc_address("glPixelStoref"); if proc == null() {dummy_pfnglpixelstorefproc} else {unsafe{transmute(proc)}}},
			pixelstorei: {let proc = get_proc_address("glPixelStorei"); if proc == null() {dummy_pfnglpixelstoreiproc} else {unsafe{transmute(proc)}}},
			readbuffer: {let proc = get_proc_address("glReadBuffer"); if proc == null() {dummy_pfnglreadbufferproc} else {unsafe{transmute(proc)}}},
			readpixels: {let proc = get_proc_address("glReadPixels"); if proc == null() {dummy_pfnglreadpixelsproc} else {unsafe{transmute(proc)}}},
			getbooleanv: {let proc = get_proc_address("glGetBooleanv"); if proc == null() {dummy_pfnglgetbooleanvproc} else {unsafe{transmute(proc)}}},
			getdoublev: {let proc = get_proc_address("glGetDoublev"); if proc == null() {dummy_pfnglgetdoublevproc} else {unsafe{transmute(proc)}}},
			geterror: {let proc = get_proc_address("glGetError"); if proc == null() {dummy_pfnglgeterrorproc} else {unsafe{transmute(proc)}}},
			getfloatv: {let proc = get_proc_address("glGetFloatv"); if proc == null() {dummy_pfnglgetfloatvproc} else {unsafe{transmute(proc)}}},
			getintegerv: {let proc = get_proc_address("glGetIntegerv"); if proc == null() {dummy_pfnglgetintegervproc} else {unsafe{transmute(proc)}}},
			getstring: {let proc = get_proc_address("glGetString"); if proc == null() {dummy_pfnglgetstringproc} else {unsafe{transmute(proc)}}},
			getteximage: {let proc = get_proc_address("glGetTexImage"); if proc == null() {dummy_pfnglgetteximageproc} else {unsafe{transmute(proc)}}},
			gettexparameterfv: {let proc = get_proc_address("glGetTexParameterfv"); if proc == null() {dummy_pfnglgettexparameterfvproc} else {unsafe{transmute(proc)}}},
			gettexparameteriv: {let proc = get_proc_address("glGetTexParameteriv"); if proc == null() {dummy_pfnglgettexparameterivproc} else {unsafe{transmute(proc)}}},
			gettexlevelparameterfv: {let proc = get_proc_address("glGetTexLevelParameterfv"); if proc == null() {dummy_pfnglgettexlevelparameterfvproc} else {unsafe{transmute(proc)}}},
			gettexlevelparameteriv: {let proc = get_proc_address("glGetTexLevelParameteriv"); if proc == null() {dummy_pfnglgettexlevelparameterivproc} else {unsafe{transmute(proc)}}},
			isenabled: {let proc = get_proc_address("glIsEnabled"); if proc == null() {dummy_pfnglisenabledproc} else {unsafe{transmute(proc)}}},
			depthrange: {let proc = get_proc_address("glDepthRange"); if proc == null() {dummy_pfngldepthrangeproc} else {unsafe{transmute(proc)}}},
			viewport: {let proc = get_proc_address("glViewport"); if proc == null() {dummy_pfnglviewportproc} else {unsafe{transmute(proc)}}},
		};
		ret.fetch_version();
		ret
	}
	#[inline(always)]
	fn fetch_version(&mut self) {
		self.vendor = unsafe{CStr::from_ptr(self.glGetString(GL_VENDOR) as *const i8)}.to_str().unwrap();
		self.renderer = unsafe{CStr::from_ptr(self.glGetString(GL_RENDERER) as *const i8)}.to_str().unwrap();
		self.version = unsafe{CStr::from_ptr(self.glGetString(GL_VERSION) as *const i8)}.to_str().unwrap();
		self.spec = "OpenGL";
		let mut verstr = self.version;
		if let Some((left, right)) = verstr.split_once(' ') {
			verstr = left;
			self.spec = right;
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
			cullface: dummy_pfnglcullfaceproc,
			frontface: dummy_pfnglfrontfaceproc,
			hint: dummy_pfnglhintproc,
			linewidth: dummy_pfngllinewidthproc,
			pointsize: dummy_pfnglpointsizeproc,
			polygonmode: dummy_pfnglpolygonmodeproc,
			scissor: dummy_pfnglscissorproc,
			texparameterf: dummy_pfngltexparameterfproc,
			texparameterfv: dummy_pfngltexparameterfvproc,
			texparameteri: dummy_pfngltexparameteriproc,
			texparameteriv: dummy_pfngltexparameterivproc,
			teximage1d: dummy_pfnglteximage1dproc,
			teximage2d: dummy_pfnglteximage2dproc,
			drawbuffer: dummy_pfngldrawbufferproc,
			clear: dummy_pfnglclearproc,
			clearcolor: dummy_pfnglclearcolorproc,
			clearstencil: dummy_pfnglclearstencilproc,
			cleardepth: dummy_pfnglcleardepthproc,
			stencilmask: dummy_pfnglstencilmaskproc,
			colormask: dummy_pfnglcolormaskproc,
			depthmask: dummy_pfngldepthmaskproc,
			disable: dummy_pfngldisableproc,
			enable: dummy_pfnglenableproc,
			finish: dummy_pfnglfinishproc,
			flush: dummy_pfnglflushproc,
			blendfunc: dummy_pfnglblendfuncproc,
			logicop: dummy_pfngllogicopproc,
			stencilfunc: dummy_pfnglstencilfuncproc,
			stencilop: dummy_pfnglstencilopproc,
			depthfunc: dummy_pfngldepthfuncproc,
			pixelstoref: dummy_pfnglpixelstorefproc,
			pixelstorei: dummy_pfnglpixelstoreiproc,
			readbuffer: dummy_pfnglreadbufferproc,
			readpixels: dummy_pfnglreadpixelsproc,
			getbooleanv: dummy_pfnglgetbooleanvproc,
			getdoublev: dummy_pfnglgetdoublevproc,
			geterror: dummy_pfnglgeterrorproc,
			getfloatv: dummy_pfnglgetfloatvproc,
			getintegerv: dummy_pfnglgetintegervproc,
			getstring: dummy_pfnglgetstringproc,
			getteximage: dummy_pfnglgetteximageproc,
			gettexparameterfv: dummy_pfnglgettexparameterfvproc,
			gettexparameteriv: dummy_pfnglgettexparameterivproc,
			gettexlevelparameterfv: dummy_pfnglgettexlevelparameterfvproc,
			gettexlevelparameteriv: dummy_pfnglgettexlevelparameterivproc,
			isenabled: dummy_pfnglisenabledproc,
			depthrange: dummy_pfngldepthrangeproc,
			viewport: dummy_pfnglviewportproc,
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
extern "system" fn dummy_pfngldrawarraysproc (_: GLenum, _: GLint, _: GLsizei) {
	panic!("OpenGL Function pointer of `glDrawArrays()` is NULL");
}
extern "system" fn dummy_pfngldrawelementsproc (_: GLenum, _: GLsizei, _: GLenum, _: *const c_void) {
	panic!("OpenGL Function pointer of `glDrawElements()` is NULL");
}
extern "system" fn dummy_pfnglgetpointervproc (_: GLenum, _: *mut *mut c_void) {
	panic!("OpenGL Function pointer of `glGetPointerv()` is NULL");
}
extern "system" fn dummy_pfnglpolygonoffsetproc (_: GLfloat, _: GLfloat) {
	panic!("OpenGL Function pointer of `glPolygonOffset()` is NULL");
}
extern "system" fn dummy_pfnglcopyteximage1dproc (_: GLenum, _: GLint, _: GLenum, _: GLint, _: GLint, _: GLsizei, _: GLint) {
	panic!("OpenGL Function pointer of `glCopyTexImage1D()` is NULL");
}
extern "system" fn dummy_pfnglcopyteximage2dproc (_: GLenum, _: GLint, _: GLenum, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLint) {
	panic!("OpenGL Function pointer of `glCopyTexImage2D()` is NULL");
}
extern "system" fn dummy_pfnglcopytexsubimage1dproc (_: GLenum, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei) {
	panic!("OpenGL Function pointer of `glCopyTexSubImage1D()` is NULL");
}
extern "system" fn dummy_pfnglcopytexsubimage2dproc (_: GLenum, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei) {
	panic!("OpenGL Function pointer of `glCopyTexSubImage2D()` is NULL");
}
extern "system" fn dummy_pfngltexsubimage1dproc (_: GLenum, _: GLint, _: GLint, _: GLsizei, _: GLenum, _: GLenum, _: *const c_void) {
	panic!("OpenGL Function pointer of `glTexSubImage1D()` is NULL");
}
extern "system" fn dummy_pfngltexsubimage2dproc (_: GLenum, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLenum, _: GLenum, _: *const c_void) {
	panic!("OpenGL Function pointer of `glTexSubImage2D()` is NULL");
}
extern "system" fn dummy_pfnglbindtextureproc (_: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glBindTexture()` is NULL");
}
extern "system" fn dummy_pfngldeletetexturesproc (_: GLsizei, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glDeleteTextures()` is NULL");
}
extern "system" fn dummy_pfnglgentexturesproc (_: GLsizei, _: *mut GLuint) {
	panic!("OpenGL Function pointer of `glGenTextures()` is NULL");
}
extern "system" fn dummy_pfnglistextureproc (_: GLuint) -> GLboolean {
	panic!("OpenGL Function pointer of `glIsTexture()` is NULL");
}
pub const GL_COLOR_LOGIC_OP: GLenum = 0x0BF2;
pub const GL_POLYGON_OFFSET_UNITS: GLenum = 0x2A00;
pub const GL_POLYGON_OFFSET_POINT: GLenum = 0x2A01;
pub const GL_POLYGON_OFFSET_LINE: GLenum = 0x2A02;
pub const GL_POLYGON_OFFSET_FILL: GLenum = 0x8037;
pub const GL_POLYGON_OFFSET_FACTOR: GLenum = 0x8038;
pub const GL_TEXTURE_BINDING_1D: GLenum = 0x8068;
pub const GL_TEXTURE_BINDING_2D: GLenum = 0x8069;
pub const GL_TEXTURE_INTERNAL_FORMAT: GLenum = 0x1003;
pub const GL_TEXTURE_RED_SIZE: GLenum = 0x805C;
pub const GL_TEXTURE_GREEN_SIZE: GLenum = 0x805D;
pub const GL_TEXTURE_BLUE_SIZE: GLenum = 0x805E;
pub const GL_TEXTURE_ALPHA_SIZE: GLenum = 0x805F;
pub const GL_DOUBLE: GLenum = 0x140A;
pub const GL_PROXY_TEXTURE_1D: GLenum = 0x8063;
pub const GL_PROXY_TEXTURE_2D: GLenum = 0x8064;
pub const GL_R3_G3_B2: GLenum = 0x2A10;
pub const GL_RGB4: GLenum = 0x804F;
pub const GL_RGB5: GLenum = 0x8050;
pub const GL_RGB8: GLenum = 0x8051;
pub const GL_RGB10: GLenum = 0x8052;
pub const GL_RGB12: GLenum = 0x8053;
pub const GL_RGB16: GLenum = 0x8054;
pub const GL_RGBA2: GLenum = 0x8055;
pub const GL_RGBA4: GLenum = 0x8056;
pub const GL_RGB5_A1: GLenum = 0x8057;
pub const GL_RGBA8: GLenum = 0x8058;
pub const GL_RGB10_A2: GLenum = 0x8059;
pub const GL_RGBA12: GLenum = 0x805A;
pub const GL_RGBA16: GLenum = 0x805B;
pub const GL_VERTEX_ARRAY: GLenum = 0x8074;

pub trait GL_1_1 {
	fn glDrawArrays(&self, mode: GLenum, first: GLint, count: GLsizei);
	fn glDrawElements(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void);
	fn glGetPointerv(&self, pname: GLenum, params: *mut *mut c_void);
	fn glPolygonOffset(&self, factor: GLfloat, units: GLfloat);
	fn glCopyTexImage1D(&self, target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint);
	fn glCopyTexImage2D(&self, target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint);
	fn glCopyTexSubImage1D(&self, target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);
	fn glCopyTexSubImage2D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
	fn glTexSubImage1D(&self, target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
	fn glTexSubImage2D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
	fn glBindTexture(&self, target: GLenum, texture: GLuint);
	fn glDeleteTextures(&self, n: GLsizei, textures: *const GLuint);
	fn glGenTextures(&self, n: GLsizei, textures: *mut GLuint);
	fn glIsTexture(&self, texture: GLuint) -> GLboolean;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
	pub fn new(base: impl GL_1_0, mut get_proc_address: impl FnMut(&'static str) -> *const c_void) -> Self {
		let (_spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 10100 {
			return Self::default();
		}
		Self {
			available: true,
			drawarrays: {let proc = get_proc_address("glDrawArrays"); if proc == null() {dummy_pfngldrawarraysproc} else {unsafe{transmute(proc)}}},
			drawelements: {let proc = get_proc_address("glDrawElements"); if proc == null() {dummy_pfngldrawelementsproc} else {unsafe{transmute(proc)}}},
			getpointerv: {let proc = get_proc_address("glGetPointerv"); if proc == null() {dummy_pfnglgetpointervproc} else {unsafe{transmute(proc)}}},
			polygonoffset: {let proc = get_proc_address("glPolygonOffset"); if proc == null() {dummy_pfnglpolygonoffsetproc} else {unsafe{transmute(proc)}}},
			copyteximage1d: {let proc = get_proc_address("glCopyTexImage1D"); if proc == null() {dummy_pfnglcopyteximage1dproc} else {unsafe{transmute(proc)}}},
			copyteximage2d: {let proc = get_proc_address("glCopyTexImage2D"); if proc == null() {dummy_pfnglcopyteximage2dproc} else {unsafe{transmute(proc)}}},
			copytexsubimage1d: {let proc = get_proc_address("glCopyTexSubImage1D"); if proc == null() {dummy_pfnglcopytexsubimage1dproc} else {unsafe{transmute(proc)}}},
			copytexsubimage2d: {let proc = get_proc_address("glCopyTexSubImage2D"); if proc == null() {dummy_pfnglcopytexsubimage2dproc} else {unsafe{transmute(proc)}}},
			texsubimage1d: {let proc = get_proc_address("glTexSubImage1D"); if proc == null() {dummy_pfngltexsubimage1dproc} else {unsafe{transmute(proc)}}},
			texsubimage2d: {let proc = get_proc_address("glTexSubImage2D"); if proc == null() {dummy_pfngltexsubimage2dproc} else {unsafe{transmute(proc)}}},
			bindtexture: {let proc = get_proc_address("glBindTexture"); if proc == null() {dummy_pfnglbindtextureproc} else {unsafe{transmute(proc)}}},
			deletetextures: {let proc = get_proc_address("glDeleteTextures"); if proc == null() {dummy_pfngldeletetexturesproc} else {unsafe{transmute(proc)}}},
			gentextures: {let proc = get_proc_address("glGenTextures"); if proc == null() {dummy_pfnglgentexturesproc} else {unsafe{transmute(proc)}}},
			istexture: {let proc = get_proc_address("glIsTexture"); if proc == null() {dummy_pfnglistextureproc} else {unsafe{transmute(proc)}}},
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
			drawarrays: dummy_pfngldrawarraysproc,
			drawelements: dummy_pfngldrawelementsproc,
			getpointerv: dummy_pfnglgetpointervproc,
			polygonoffset: dummy_pfnglpolygonoffsetproc,
			copyteximage1d: dummy_pfnglcopyteximage1dproc,
			copyteximage2d: dummy_pfnglcopyteximage2dproc,
			copytexsubimage1d: dummy_pfnglcopytexsubimage1dproc,
			copytexsubimage2d: dummy_pfnglcopytexsubimage2dproc,
			texsubimage1d: dummy_pfngltexsubimage1dproc,
			texsubimage2d: dummy_pfngltexsubimage2dproc,
			bindtexture: dummy_pfnglbindtextureproc,
			deletetextures: dummy_pfngldeletetexturesproc,
			gentextures: dummy_pfnglgentexturesproc,
			istexture: dummy_pfnglistextureproc,
		}
	}
}

type PFNGLDRAWRANGEELEMENTSPROC = extern "system" fn(GLenum, GLuint, GLuint, GLsizei, GLenum, *const c_void);
type PFNGLTEXIMAGE3DPROC = extern "system" fn(GLenum, GLint, GLint, GLsizei, GLsizei, GLsizei, GLint, GLenum, GLenum, *const c_void);
type PFNGLTEXSUBIMAGE3DPROC = extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLsizei, GLsizei, GLsizei, GLenum, GLenum, *const c_void);
type PFNGLCOPYTEXSUBIMAGE3DPROC = extern "system" fn(GLenum, GLint, GLint, GLint, GLint, GLint, GLint, GLsizei, GLsizei);
extern "system" fn dummy_pfngldrawrangeelementsproc (_: GLenum, _: GLuint, _: GLuint, _: GLsizei, _: GLenum, _: *const c_void) {
	panic!("OpenGL Function pointer of `glDrawRangeElements()` is NULL");
}
extern "system" fn dummy_pfnglteximage3dproc (_: GLenum, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLsizei, _: GLint, _: GLenum, _: GLenum, _: *const c_void) {
	panic!("OpenGL Function pointer of `glTexImage3D()` is NULL");
}
extern "system" fn dummy_pfngltexsubimage3dproc (_: GLenum, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLsizei, _: GLenum, _: GLenum, _: *const c_void) {
	panic!("OpenGL Function pointer of `glTexSubImage3D()` is NULL");
}
extern "system" fn dummy_pfnglcopytexsubimage3dproc (_: GLenum, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei) {
	panic!("OpenGL Function pointer of `glCopyTexSubImage3D()` is NULL");
}
pub const GL_UNSIGNED_BYTE_3_3_2: GLenum = 0x8032;
pub const GL_UNSIGNED_SHORT_4_4_4_4: GLenum = 0x8033;
pub const GL_UNSIGNED_SHORT_5_5_5_1: GLenum = 0x8034;
pub const GL_UNSIGNED_INT_8_8_8_8: GLenum = 0x8035;
pub const GL_UNSIGNED_INT_10_10_10_2: GLenum = 0x8036;
pub const GL_TEXTURE_BINDING_3D: GLenum = 0x806A;
pub const GL_PACK_SKIP_IMAGES: GLenum = 0x806B;
pub const GL_PACK_IMAGE_HEIGHT: GLenum = 0x806C;
pub const GL_UNPACK_SKIP_IMAGES: GLenum = 0x806D;
pub const GL_UNPACK_IMAGE_HEIGHT: GLenum = 0x806E;
pub const GL_TEXTURE_3D: GLenum = 0x806F;
pub const GL_PROXY_TEXTURE_3D: GLenum = 0x8070;
pub const GL_TEXTURE_DEPTH: GLenum = 0x8071;
pub const GL_TEXTURE_WRAP_R: GLenum = 0x8072;
pub const GL_MAX_3D_TEXTURE_SIZE: GLenum = 0x8073;
pub const GL_UNSIGNED_BYTE_2_3_3_REV: GLenum = 0x8362;
pub const GL_UNSIGNED_SHORT_5_6_5: GLenum = 0x8363;
pub const GL_UNSIGNED_SHORT_5_6_5_REV: GLenum = 0x8364;
pub const GL_UNSIGNED_SHORT_4_4_4_4_REV: GLenum = 0x8365;
pub const GL_UNSIGNED_SHORT_1_5_5_5_REV: GLenum = 0x8366;
pub const GL_UNSIGNED_INT_8_8_8_8_REV: GLenum = 0x8367;
pub const GL_UNSIGNED_INT_2_10_10_10_REV: GLenum = 0x8368;
pub const GL_BGR: GLenum = 0x80E0;
pub const GL_BGRA: GLenum = 0x80E1;
pub const GL_MAX_ELEMENTS_VERTICES: GLenum = 0x80E8;
pub const GL_MAX_ELEMENTS_INDICES: GLenum = 0x80E9;
pub const GL_CLAMP_TO_EDGE: GLint = 0x812F;
pub const GL_TEXTURE_MIN_LOD: GLenum = 0x813A;
pub const GL_TEXTURE_MAX_LOD: GLenum = 0x813B;
pub const GL_TEXTURE_BASE_LEVEL: GLenum = 0x813C;
pub const GL_TEXTURE_MAX_LEVEL: GLenum = 0x813D;
pub const GL_SMOOTH_POINT_SIZE_RANGE: GLenum = 0x0B12;
pub const GL_SMOOTH_POINT_SIZE_GRANULARITY: GLenum = 0x0B13;
pub const GL_SMOOTH_LINE_WIDTH_RANGE: GLenum = 0x0B22;
pub const GL_SMOOTH_LINE_WIDTH_GRANULARITY: GLenum = 0x0B23;
pub const GL_ALIASED_LINE_WIDTH_RANGE: GLenum = 0x846E;
pub const GL_RESCALE_NORMAL: GLenum = 0x803A;
pub const GL_LIGHT_MODEL_COLOR_CONTROL: GLenum = 0x81F8;
pub const GL_SINGLE_COLOR: GLenum = 0x81F9;
pub const GL_SEPARATE_SPECULAR_COLOR: GLenum = 0x81FA;
pub const GL_ALIASED_POINT_SIZE_RANGE: GLenum = 0x846D;

pub trait GL_1_2 {
	fn glDrawRangeElements(&self, mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const c_void);
	fn glTexImage3D(&self, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void);
	fn glTexSubImage3D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
	fn glCopyTexSubImage3D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
	pub fn new(base: impl GL_1_0, mut get_proc_address: impl FnMut(&'static str) -> *const c_void) -> Self {
		let (_spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 10200 {
			return Self::default();
		}
		Self {
			available: true,
			drawrangeelements: {let proc = get_proc_address("glDrawRangeElements"); if proc == null() {dummy_pfngldrawrangeelementsproc} else {unsafe{transmute(proc)}}},
			teximage3d: {let proc = get_proc_address("glTexImage3D"); if proc == null() {dummy_pfnglteximage3dproc} else {unsafe{transmute(proc)}}},
			texsubimage3d: {let proc = get_proc_address("glTexSubImage3D"); if proc == null() {dummy_pfngltexsubimage3dproc} else {unsafe{transmute(proc)}}},
			copytexsubimage3d: {let proc = get_proc_address("glCopyTexSubImage3D"); if proc == null() {dummy_pfnglcopytexsubimage3dproc} else {unsafe{transmute(proc)}}},
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
			drawrangeelements: dummy_pfngldrawrangeelementsproc,
			teximage3d: dummy_pfnglteximage3dproc,
			texsubimage3d: dummy_pfngltexsubimage3dproc,
			copytexsubimage3d: dummy_pfnglcopytexsubimage3dproc,
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
extern "system" fn dummy_pfnglactivetextureproc (_: GLenum) {
	panic!("OpenGL Function pointer of `glActiveTexture()` is NULL");
}
extern "system" fn dummy_pfnglsamplecoverageproc (_: GLfloat, _: GLboolean) {
	panic!("OpenGL Function pointer of `glSampleCoverage()` is NULL");
}
extern "system" fn dummy_pfnglcompressedteximage3dproc (_: GLenum, _: GLint, _: GLenum, _: GLsizei, _: GLsizei, _: GLsizei, _: GLint, _: GLsizei, _: *const c_void) {
	panic!("OpenGL Function pointer of `glCompressedTexImage3D()` is NULL");
}
extern "system" fn dummy_pfnglcompressedteximage2dproc (_: GLenum, _: GLint, _: GLenum, _: GLsizei, _: GLsizei, _: GLint, _: GLsizei, _: *const c_void) {
	panic!("OpenGL Function pointer of `glCompressedTexImage2D()` is NULL");
}
extern "system" fn dummy_pfnglcompressedteximage1dproc (_: GLenum, _: GLint, _: GLenum, _: GLsizei, _: GLint, _: GLsizei, _: *const c_void) {
	panic!("OpenGL Function pointer of `glCompressedTexImage1D()` is NULL");
}
extern "system" fn dummy_pfnglcompressedtexsubimage3dproc (_: GLenum, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLsizei, _: GLenum, _: GLsizei, _: *const c_void) {
	panic!("OpenGL Function pointer of `glCompressedTexSubImage3D()` is NULL");
}
extern "system" fn dummy_pfnglcompressedtexsubimage2dproc (_: GLenum, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLenum, _: GLsizei, _: *const c_void) {
	panic!("OpenGL Function pointer of `glCompressedTexSubImage2D()` is NULL");
}
extern "system" fn dummy_pfnglcompressedtexsubimage1dproc (_: GLenum, _: GLint, _: GLint, _: GLsizei, _: GLenum, _: GLsizei, _: *const c_void) {
	panic!("OpenGL Function pointer of `glCompressedTexSubImage1D()` is NULL");
}
extern "system" fn dummy_pfnglgetcompressedteximageproc (_: GLenum, _: GLint, _: *mut c_void) {
	panic!("OpenGL Function pointer of `glGetCompressedTexImage()` is NULL");
}
extern "system" fn dummy_pfnglclientactivetextureproc (_: GLenum) {
	panic!("OpenGL Function pointer of `glClientActiveTexture()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord1dproc (_: GLenum, _: GLdouble) {
	panic!("OpenGL Function pointer of `glMultiTexCoord1d()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord1dvproc (_: GLenum, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glMultiTexCoord1dv()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord1fproc (_: GLenum, _: GLfloat) {
	panic!("OpenGL Function pointer of `glMultiTexCoord1f()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord1fvproc (_: GLenum, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glMultiTexCoord1fv()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord1iproc (_: GLenum, _: GLint) {
	panic!("OpenGL Function pointer of `glMultiTexCoord1i()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord1ivproc (_: GLenum, _: *const GLint) {
	panic!("OpenGL Function pointer of `glMultiTexCoord1iv()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord1sproc (_: GLenum, _: GLshort) {
	panic!("OpenGL Function pointer of `glMultiTexCoord1s()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord1svproc (_: GLenum, _: *const GLshort) {
	panic!("OpenGL Function pointer of `glMultiTexCoord1sv()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord2dproc (_: GLenum, _: GLdouble, _: GLdouble) {
	panic!("OpenGL Function pointer of `glMultiTexCoord2d()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord2dvproc (_: GLenum, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glMultiTexCoord2dv()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord2fproc (_: GLenum, _: GLfloat, _: GLfloat) {
	panic!("OpenGL Function pointer of `glMultiTexCoord2f()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord2fvproc (_: GLenum, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glMultiTexCoord2fv()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord2iproc (_: GLenum, _: GLint, _: GLint) {
	panic!("OpenGL Function pointer of `glMultiTexCoord2i()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord2ivproc (_: GLenum, _: *const GLint) {
	panic!("OpenGL Function pointer of `glMultiTexCoord2iv()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord2sproc (_: GLenum, _: GLshort, _: GLshort) {
	panic!("OpenGL Function pointer of `glMultiTexCoord2s()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord2svproc (_: GLenum, _: *const GLshort) {
	panic!("OpenGL Function pointer of `glMultiTexCoord2sv()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord3dproc (_: GLenum, _: GLdouble, _: GLdouble, _: GLdouble) {
	panic!("OpenGL Function pointer of `glMultiTexCoord3d()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord3dvproc (_: GLenum, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glMultiTexCoord3dv()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord3fproc (_: GLenum, _: GLfloat, _: GLfloat, _: GLfloat) {
	panic!("OpenGL Function pointer of `glMultiTexCoord3f()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord3fvproc (_: GLenum, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glMultiTexCoord3fv()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord3iproc (_: GLenum, _: GLint, _: GLint, _: GLint) {
	panic!("OpenGL Function pointer of `glMultiTexCoord3i()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord3ivproc (_: GLenum, _: *const GLint) {
	panic!("OpenGL Function pointer of `glMultiTexCoord3iv()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord3sproc (_: GLenum, _: GLshort, _: GLshort, _: GLshort) {
	panic!("OpenGL Function pointer of `glMultiTexCoord3s()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord3svproc (_: GLenum, _: *const GLshort) {
	panic!("OpenGL Function pointer of `glMultiTexCoord3sv()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord4dproc (_: GLenum, _: GLdouble, _: GLdouble, _: GLdouble, _: GLdouble) {
	panic!("OpenGL Function pointer of `glMultiTexCoord4d()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord4dvproc (_: GLenum, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glMultiTexCoord4dv()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord4fproc (_: GLenum, _: GLfloat, _: GLfloat, _: GLfloat, _: GLfloat) {
	panic!("OpenGL Function pointer of `glMultiTexCoord4f()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord4fvproc (_: GLenum, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glMultiTexCoord4fv()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord4iproc (_: GLenum, _: GLint, _: GLint, _: GLint, _: GLint) {
	panic!("OpenGL Function pointer of `glMultiTexCoord4i()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord4ivproc (_: GLenum, _: *const GLint) {
	panic!("OpenGL Function pointer of `glMultiTexCoord4iv()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord4sproc (_: GLenum, _: GLshort, _: GLshort, _: GLshort, _: GLshort) {
	panic!("OpenGL Function pointer of `glMultiTexCoord4s()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoord4svproc (_: GLenum, _: *const GLshort) {
	panic!("OpenGL Function pointer of `glMultiTexCoord4sv()` is NULL");
}
extern "system" fn dummy_pfnglloadtransposematrixfproc (_: *const GLfloat) {
	panic!("OpenGL Function pointer of `glLoadTransposeMatrixf()` is NULL");
}
extern "system" fn dummy_pfnglloadtransposematrixdproc (_: *const GLdouble) {
	panic!("OpenGL Function pointer of `glLoadTransposeMatrixd()` is NULL");
}
extern "system" fn dummy_pfnglmulttransposematrixfproc (_: *const GLfloat) {
	panic!("OpenGL Function pointer of `glMultTransposeMatrixf()` is NULL");
}
extern "system" fn dummy_pfnglmulttransposematrixdproc (_: *const GLdouble) {
	panic!("OpenGL Function pointer of `glMultTransposeMatrixd()` is NULL");
}
pub const GL_TEXTURE0: GLenum = 0x84C0;
pub const GL_TEXTURE1: GLenum = 0x84C1;
pub const GL_TEXTURE2: GLenum = 0x84C2;
pub const GL_TEXTURE3: GLenum = 0x84C3;
pub const GL_TEXTURE4: GLenum = 0x84C4;
pub const GL_TEXTURE5: GLenum = 0x84C5;
pub const GL_TEXTURE6: GLenum = 0x84C6;
pub const GL_TEXTURE7: GLenum = 0x84C7;
pub const GL_TEXTURE8: GLenum = 0x84C8;
pub const GL_TEXTURE9: GLenum = 0x84C9;
pub const GL_TEXTURE10: GLenum = 0x84CA;
pub const GL_TEXTURE11: GLenum = 0x84CB;
pub const GL_TEXTURE12: GLenum = 0x84CC;
pub const GL_TEXTURE13: GLenum = 0x84CD;
pub const GL_TEXTURE14: GLenum = 0x84CE;
pub const GL_TEXTURE15: GLenum = 0x84CF;
pub const GL_TEXTURE16: GLenum = 0x84D0;
pub const GL_TEXTURE17: GLenum = 0x84D1;
pub const GL_TEXTURE18: GLenum = 0x84D2;
pub const GL_TEXTURE19: GLenum = 0x84D3;
pub const GL_TEXTURE20: GLenum = 0x84D4;
pub const GL_TEXTURE21: GLenum = 0x84D5;
pub const GL_TEXTURE22: GLenum = 0x84D6;
pub const GL_TEXTURE23: GLenum = 0x84D7;
pub const GL_TEXTURE24: GLenum = 0x84D8;
pub const GL_TEXTURE25: GLenum = 0x84D9;
pub const GL_TEXTURE26: GLenum = 0x84DA;
pub const GL_TEXTURE27: GLenum = 0x84DB;
pub const GL_TEXTURE28: GLenum = 0x84DC;
pub const GL_TEXTURE29: GLenum = 0x84DD;
pub const GL_TEXTURE30: GLenum = 0x84DE;
pub const GL_TEXTURE31: GLenum = 0x84DF;
pub const GL_ACTIVE_TEXTURE: GLenum = 0x84E0;
pub const GL_MULTISAMPLE: GLenum = 0x809D;
pub const GL_SAMPLE_ALPHA_TO_COVERAGE: GLenum = 0x809E;
pub const GL_SAMPLE_ALPHA_TO_ONE: GLenum = 0x809F;
pub const GL_SAMPLE_COVERAGE: GLenum = 0x80A0;
pub const GL_SAMPLE_BUFFERS: GLenum = 0x80A8;
pub const GL_SAMPLES: GLenum = 0x80A9;
pub const GL_SAMPLE_COVERAGE_VALUE: GLenum = 0x80AA;
pub const GL_SAMPLE_COVERAGE_INVERT: GLenum = 0x80AB;
pub const GL_TEXTURE_CUBE_MAP: GLenum = 0x8513;
pub const GL_TEXTURE_BINDING_CUBE_MAP: GLenum = 0x8514;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X: GLenum = 0x8515;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X: GLenum = 0x8516;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y: GLenum = 0x8517;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: GLenum = 0x8518;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z: GLenum = 0x8519;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: GLenum = 0x851A;
pub const GL_PROXY_TEXTURE_CUBE_MAP: GLenum = 0x851B;
pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE: GLenum = 0x851C;
pub const GL_COMPRESSED_RGB: GLenum = 0x84ED;
pub const GL_COMPRESSED_RGBA: GLenum = 0x84EE;
pub const GL_TEXTURE_COMPRESSION_HINT: GLenum = 0x84EF;
pub const GL_TEXTURE_COMPRESSED_IMAGE_SIZE: GLenum = 0x86A0;
pub const GL_TEXTURE_COMPRESSED: GLenum = 0x86A1;
pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A2;
pub const GL_COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A3;
pub const GL_CLAMP_TO_BORDER: GLint = 0x812D;
pub const GL_CLIENT_ACTIVE_TEXTURE: GLenum = 0x84E1;
pub const GL_MAX_TEXTURE_UNITS: GLenum = 0x84E2;
pub const GL_TRANSPOSE_MODELVIEW_MATRIX: GLenum = 0x84E3;
pub const GL_TRANSPOSE_PROJECTION_MATRIX: GLenum = 0x84E4;
pub const GL_TRANSPOSE_TEXTURE_MATRIX: GLenum = 0x84E5;
pub const GL_TRANSPOSE_COLOR_MATRIX: GLenum = 0x84E6;
pub const GL_MULTISAMPLE_BIT: GLbitfield = 0x20000000;
pub const GL_NORMAL_MAP: GLenum = 0x8511;
pub const GL_REFLECTION_MAP: GLenum = 0x8512;
pub const GL_COMPRESSED_ALPHA: GLenum = 0x84E9;
pub const GL_COMPRESSED_LUMINANCE: GLenum = 0x84EA;
pub const GL_COMPRESSED_LUMINANCE_ALPHA: GLenum = 0x84EB;
pub const GL_COMPRESSED_INTENSITY: GLenum = 0x84EC;
pub const GL_COMBINE: GLenum = 0x8570;
pub const GL_COMBINE_RGB: GLenum = 0x8571;
pub const GL_COMBINE_ALPHA: GLenum = 0x8572;
pub const GL_SOURCE0_RGB: GLenum = 0x8580;
pub const GL_SOURCE1_RGB: GLenum = 0x8581;
pub const GL_SOURCE2_RGB: GLenum = 0x8582;
pub const GL_SOURCE0_ALPHA: GLenum = 0x8588;
pub const GL_SOURCE1_ALPHA: GLenum = 0x8589;
pub const GL_SOURCE2_ALPHA: GLenum = 0x858A;
pub const GL_OPERAND0_RGB: GLenum = 0x8590;
pub const GL_OPERAND1_RGB: GLenum = 0x8591;
pub const GL_OPERAND2_RGB: GLenum = 0x8592;
pub const GL_OPERAND0_ALPHA: GLenum = 0x8598;
pub const GL_OPERAND1_ALPHA: GLenum = 0x8599;
pub const GL_OPERAND2_ALPHA: GLenum = 0x859A;
pub const GL_RGB_SCALE: GLenum = 0x8573;
pub const GL_ADD_SIGNED: GLenum = 0x8574;
pub const GL_INTERPOLATE: GLenum = 0x8575;
pub const GL_SUBTRACT: GLenum = 0x84E7;
pub const GL_CONSTANT: GLenum = 0x8576;
pub const GL_PRIMARY_COLOR: GLenum = 0x8577;
pub const GL_PREVIOUS: GLenum = 0x8578;
pub const GL_DOT3_RGB: GLenum = 0x86AE;
pub const GL_DOT3_RGBA: GLenum = 0x86AF;

pub trait GL_1_3 {
	fn glActiveTexture(&self, texture: GLenum);
	fn glSampleCoverage(&self, value: GLfloat, invert: GLboolean);
	fn glCompressedTexImage3D(&self, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void);
	fn glCompressedTexImage2D(&self, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void);
	fn glCompressedTexImage1D(&self, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void);
	fn glCompressedTexSubImage3D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
	fn glCompressedTexSubImage2D(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
	fn glCompressedTexSubImage1D(&self, target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
	fn glGetCompressedTexImage(&self, target: GLenum, level: GLint, img: *mut c_void);
	fn glClientActiveTexture(&self, texture: GLenum);
	fn glMultiTexCoord1d(&self, target: GLenum, s: GLdouble);
	fn glMultiTexCoord1dv(&self, target: GLenum, v: *const GLdouble);
	fn glMultiTexCoord1f(&self, target: GLenum, s: GLfloat);
	fn glMultiTexCoord1fv(&self, target: GLenum, v: *const GLfloat);
	fn glMultiTexCoord1i(&self, target: GLenum, s: GLint);
	fn glMultiTexCoord1iv(&self, target: GLenum, v: *const GLint);
	fn glMultiTexCoord1s(&self, target: GLenum, s: GLshort);
	fn glMultiTexCoord1sv(&self, target: GLenum, v: *const GLshort);
	fn glMultiTexCoord2d(&self, target: GLenum, s: GLdouble, t: GLdouble);
	fn glMultiTexCoord2dv(&self, target: GLenum, v: *const GLdouble);
	fn glMultiTexCoord2f(&self, target: GLenum, s: GLfloat, t: GLfloat);
	fn glMultiTexCoord2fv(&self, target: GLenum, v: *const GLfloat);
	fn glMultiTexCoord2i(&self, target: GLenum, s: GLint, t: GLint);
	fn glMultiTexCoord2iv(&self, target: GLenum, v: *const GLint);
	fn glMultiTexCoord2s(&self, target: GLenum, s: GLshort, t: GLshort);
	fn glMultiTexCoord2sv(&self, target: GLenum, v: *const GLshort);
	fn glMultiTexCoord3d(&self, target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble);
	fn glMultiTexCoord3dv(&self, target: GLenum, v: *const GLdouble);
	fn glMultiTexCoord3f(&self, target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat);
	fn glMultiTexCoord3fv(&self, target: GLenum, v: *const GLfloat);
	fn glMultiTexCoord3i(&self, target: GLenum, s: GLint, t: GLint, r: GLint);
	fn glMultiTexCoord3iv(&self, target: GLenum, v: *const GLint);
	fn glMultiTexCoord3s(&self, target: GLenum, s: GLshort, t: GLshort, r: GLshort);
	fn glMultiTexCoord3sv(&self, target: GLenum, v: *const GLshort);
	fn glMultiTexCoord4d(&self, target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble);
	fn glMultiTexCoord4dv(&self, target: GLenum, v: *const GLdouble);
	fn glMultiTexCoord4f(&self, target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat);
	fn glMultiTexCoord4fv(&self, target: GLenum, v: *const GLfloat);
	fn glMultiTexCoord4i(&self, target: GLenum, s: GLint, t: GLint, r: GLint, q: GLint);
	fn glMultiTexCoord4iv(&self, target: GLenum, v: *const GLint);
	fn glMultiTexCoord4s(&self, target: GLenum, s: GLshort, t: GLshort, r: GLshort, q: GLshort);
	fn glMultiTexCoord4sv(&self, target: GLenum, v: *const GLshort);
	fn glLoadTransposeMatrixf(&self, m: *const GLfloat);
	fn glLoadTransposeMatrixd(&self, m: *const GLdouble);
	fn glMultTransposeMatrixf(&self, m: *const GLfloat);
	fn glMultTransposeMatrixd(&self, m: *const GLdouble);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
	pub fn new(base: impl GL_1_0, mut get_proc_address: impl FnMut(&'static str) -> *const c_void) -> Self {
		let (_spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 10300 {
			return Self::default();
		}
		Self {
			available: true,
			activetexture: {let proc = get_proc_address("glActiveTexture"); if proc == null() {dummy_pfnglactivetextureproc} else {unsafe{transmute(proc)}}},
			samplecoverage: {let proc = get_proc_address("glSampleCoverage"); if proc == null() {dummy_pfnglsamplecoverageproc} else {unsafe{transmute(proc)}}},
			compressedteximage3d: {let proc = get_proc_address("glCompressedTexImage3D"); if proc == null() {dummy_pfnglcompressedteximage3dproc} else {unsafe{transmute(proc)}}},
			compressedteximage2d: {let proc = get_proc_address("glCompressedTexImage2D"); if proc == null() {dummy_pfnglcompressedteximage2dproc} else {unsafe{transmute(proc)}}},
			compressedteximage1d: {let proc = get_proc_address("glCompressedTexImage1D"); if proc == null() {dummy_pfnglcompressedteximage1dproc} else {unsafe{transmute(proc)}}},
			compressedtexsubimage3d: {let proc = get_proc_address("glCompressedTexSubImage3D"); if proc == null() {dummy_pfnglcompressedtexsubimage3dproc} else {unsafe{transmute(proc)}}},
			compressedtexsubimage2d: {let proc = get_proc_address("glCompressedTexSubImage2D"); if proc == null() {dummy_pfnglcompressedtexsubimage2dproc} else {unsafe{transmute(proc)}}},
			compressedtexsubimage1d: {let proc = get_proc_address("glCompressedTexSubImage1D"); if proc == null() {dummy_pfnglcompressedtexsubimage1dproc} else {unsafe{transmute(proc)}}},
			getcompressedteximage: {let proc = get_proc_address("glGetCompressedTexImage"); if proc == null() {dummy_pfnglgetcompressedteximageproc} else {unsafe{transmute(proc)}}},
			clientactivetexture: {let proc = get_proc_address("glClientActiveTexture"); if proc == null() {dummy_pfnglclientactivetextureproc} else {unsafe{transmute(proc)}}},
			multitexcoord1d: {let proc = get_proc_address("glMultiTexCoord1d"); if proc == null() {dummy_pfnglmultitexcoord1dproc} else {unsafe{transmute(proc)}}},
			multitexcoord1dv: {let proc = get_proc_address("glMultiTexCoord1dv"); if proc == null() {dummy_pfnglmultitexcoord1dvproc} else {unsafe{transmute(proc)}}},
			multitexcoord1f: {let proc = get_proc_address("glMultiTexCoord1f"); if proc == null() {dummy_pfnglmultitexcoord1fproc} else {unsafe{transmute(proc)}}},
			multitexcoord1fv: {let proc = get_proc_address("glMultiTexCoord1fv"); if proc == null() {dummy_pfnglmultitexcoord1fvproc} else {unsafe{transmute(proc)}}},
			multitexcoord1i: {let proc = get_proc_address("glMultiTexCoord1i"); if proc == null() {dummy_pfnglmultitexcoord1iproc} else {unsafe{transmute(proc)}}},
			multitexcoord1iv: {let proc = get_proc_address("glMultiTexCoord1iv"); if proc == null() {dummy_pfnglmultitexcoord1ivproc} else {unsafe{transmute(proc)}}},
			multitexcoord1s: {let proc = get_proc_address("glMultiTexCoord1s"); if proc == null() {dummy_pfnglmultitexcoord1sproc} else {unsafe{transmute(proc)}}},
			multitexcoord1sv: {let proc = get_proc_address("glMultiTexCoord1sv"); if proc == null() {dummy_pfnglmultitexcoord1svproc} else {unsafe{transmute(proc)}}},
			multitexcoord2d: {let proc = get_proc_address("glMultiTexCoord2d"); if proc == null() {dummy_pfnglmultitexcoord2dproc} else {unsafe{transmute(proc)}}},
			multitexcoord2dv: {let proc = get_proc_address("glMultiTexCoord2dv"); if proc == null() {dummy_pfnglmultitexcoord2dvproc} else {unsafe{transmute(proc)}}},
			multitexcoord2f: {let proc = get_proc_address("glMultiTexCoord2f"); if proc == null() {dummy_pfnglmultitexcoord2fproc} else {unsafe{transmute(proc)}}},
			multitexcoord2fv: {let proc = get_proc_address("glMultiTexCoord2fv"); if proc == null() {dummy_pfnglmultitexcoord2fvproc} else {unsafe{transmute(proc)}}},
			multitexcoord2i: {let proc = get_proc_address("glMultiTexCoord2i"); if proc == null() {dummy_pfnglmultitexcoord2iproc} else {unsafe{transmute(proc)}}},
			multitexcoord2iv: {let proc = get_proc_address("glMultiTexCoord2iv"); if proc == null() {dummy_pfnglmultitexcoord2ivproc} else {unsafe{transmute(proc)}}},
			multitexcoord2s: {let proc = get_proc_address("glMultiTexCoord2s"); if proc == null() {dummy_pfnglmultitexcoord2sproc} else {unsafe{transmute(proc)}}},
			multitexcoord2sv: {let proc = get_proc_address("glMultiTexCoord2sv"); if proc == null() {dummy_pfnglmultitexcoord2svproc} else {unsafe{transmute(proc)}}},
			multitexcoord3d: {let proc = get_proc_address("glMultiTexCoord3d"); if proc == null() {dummy_pfnglmultitexcoord3dproc} else {unsafe{transmute(proc)}}},
			multitexcoord3dv: {let proc = get_proc_address("glMultiTexCoord3dv"); if proc == null() {dummy_pfnglmultitexcoord3dvproc} else {unsafe{transmute(proc)}}},
			multitexcoord3f: {let proc = get_proc_address("glMultiTexCoord3f"); if proc == null() {dummy_pfnglmultitexcoord3fproc} else {unsafe{transmute(proc)}}},
			multitexcoord3fv: {let proc = get_proc_address("glMultiTexCoord3fv"); if proc == null() {dummy_pfnglmultitexcoord3fvproc} else {unsafe{transmute(proc)}}},
			multitexcoord3i: {let proc = get_proc_address("glMultiTexCoord3i"); if proc == null() {dummy_pfnglmultitexcoord3iproc} else {unsafe{transmute(proc)}}},
			multitexcoord3iv: {let proc = get_proc_address("glMultiTexCoord3iv"); if proc == null() {dummy_pfnglmultitexcoord3ivproc} else {unsafe{transmute(proc)}}},
			multitexcoord3s: {let proc = get_proc_address("glMultiTexCoord3s"); if proc == null() {dummy_pfnglmultitexcoord3sproc} else {unsafe{transmute(proc)}}},
			multitexcoord3sv: {let proc = get_proc_address("glMultiTexCoord3sv"); if proc == null() {dummy_pfnglmultitexcoord3svproc} else {unsafe{transmute(proc)}}},
			multitexcoord4d: {let proc = get_proc_address("glMultiTexCoord4d"); if proc == null() {dummy_pfnglmultitexcoord4dproc} else {unsafe{transmute(proc)}}},
			multitexcoord4dv: {let proc = get_proc_address("glMultiTexCoord4dv"); if proc == null() {dummy_pfnglmultitexcoord4dvproc} else {unsafe{transmute(proc)}}},
			multitexcoord4f: {let proc = get_proc_address("glMultiTexCoord4f"); if proc == null() {dummy_pfnglmultitexcoord4fproc} else {unsafe{transmute(proc)}}},
			multitexcoord4fv: {let proc = get_proc_address("glMultiTexCoord4fv"); if proc == null() {dummy_pfnglmultitexcoord4fvproc} else {unsafe{transmute(proc)}}},
			multitexcoord4i: {let proc = get_proc_address("glMultiTexCoord4i"); if proc == null() {dummy_pfnglmultitexcoord4iproc} else {unsafe{transmute(proc)}}},
			multitexcoord4iv: {let proc = get_proc_address("glMultiTexCoord4iv"); if proc == null() {dummy_pfnglmultitexcoord4ivproc} else {unsafe{transmute(proc)}}},
			multitexcoord4s: {let proc = get_proc_address("glMultiTexCoord4s"); if proc == null() {dummy_pfnglmultitexcoord4sproc} else {unsafe{transmute(proc)}}},
			multitexcoord4sv: {let proc = get_proc_address("glMultiTexCoord4sv"); if proc == null() {dummy_pfnglmultitexcoord4svproc} else {unsafe{transmute(proc)}}},
			loadtransposematrixf: {let proc = get_proc_address("glLoadTransposeMatrixf"); if proc == null() {dummy_pfnglloadtransposematrixfproc} else {unsafe{transmute(proc)}}},
			loadtransposematrixd: {let proc = get_proc_address("glLoadTransposeMatrixd"); if proc == null() {dummy_pfnglloadtransposematrixdproc} else {unsafe{transmute(proc)}}},
			multtransposematrixf: {let proc = get_proc_address("glMultTransposeMatrixf"); if proc == null() {dummy_pfnglmulttransposematrixfproc} else {unsafe{transmute(proc)}}},
			multtransposematrixd: {let proc = get_proc_address("glMultTransposeMatrixd"); if proc == null() {dummy_pfnglmulttransposematrixdproc} else {unsafe{transmute(proc)}}},
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
			activetexture: dummy_pfnglactivetextureproc,
			samplecoverage: dummy_pfnglsamplecoverageproc,
			compressedteximage3d: dummy_pfnglcompressedteximage3dproc,
			compressedteximage2d: dummy_pfnglcompressedteximage2dproc,
			compressedteximage1d: dummy_pfnglcompressedteximage1dproc,
			compressedtexsubimage3d: dummy_pfnglcompressedtexsubimage3dproc,
			compressedtexsubimage2d: dummy_pfnglcompressedtexsubimage2dproc,
			compressedtexsubimage1d: dummy_pfnglcompressedtexsubimage1dproc,
			getcompressedteximage: dummy_pfnglgetcompressedteximageproc,
			clientactivetexture: dummy_pfnglclientactivetextureproc,
			multitexcoord1d: dummy_pfnglmultitexcoord1dproc,
			multitexcoord1dv: dummy_pfnglmultitexcoord1dvproc,
			multitexcoord1f: dummy_pfnglmultitexcoord1fproc,
			multitexcoord1fv: dummy_pfnglmultitexcoord1fvproc,
			multitexcoord1i: dummy_pfnglmultitexcoord1iproc,
			multitexcoord1iv: dummy_pfnglmultitexcoord1ivproc,
			multitexcoord1s: dummy_pfnglmultitexcoord1sproc,
			multitexcoord1sv: dummy_pfnglmultitexcoord1svproc,
			multitexcoord2d: dummy_pfnglmultitexcoord2dproc,
			multitexcoord2dv: dummy_pfnglmultitexcoord2dvproc,
			multitexcoord2f: dummy_pfnglmultitexcoord2fproc,
			multitexcoord2fv: dummy_pfnglmultitexcoord2fvproc,
			multitexcoord2i: dummy_pfnglmultitexcoord2iproc,
			multitexcoord2iv: dummy_pfnglmultitexcoord2ivproc,
			multitexcoord2s: dummy_pfnglmultitexcoord2sproc,
			multitexcoord2sv: dummy_pfnglmultitexcoord2svproc,
			multitexcoord3d: dummy_pfnglmultitexcoord3dproc,
			multitexcoord3dv: dummy_pfnglmultitexcoord3dvproc,
			multitexcoord3f: dummy_pfnglmultitexcoord3fproc,
			multitexcoord3fv: dummy_pfnglmultitexcoord3fvproc,
			multitexcoord3i: dummy_pfnglmultitexcoord3iproc,
			multitexcoord3iv: dummy_pfnglmultitexcoord3ivproc,
			multitexcoord3s: dummy_pfnglmultitexcoord3sproc,
			multitexcoord3sv: dummy_pfnglmultitexcoord3svproc,
			multitexcoord4d: dummy_pfnglmultitexcoord4dproc,
			multitexcoord4dv: dummy_pfnglmultitexcoord4dvproc,
			multitexcoord4f: dummy_pfnglmultitexcoord4fproc,
			multitexcoord4fv: dummy_pfnglmultitexcoord4fvproc,
			multitexcoord4i: dummy_pfnglmultitexcoord4iproc,
			multitexcoord4iv: dummy_pfnglmultitexcoord4ivproc,
			multitexcoord4s: dummy_pfnglmultitexcoord4sproc,
			multitexcoord4sv: dummy_pfnglmultitexcoord4svproc,
			loadtransposematrixf: dummy_pfnglloadtransposematrixfproc,
			loadtransposematrixd: dummy_pfnglloadtransposematrixdproc,
			multtransposematrixf: dummy_pfnglmulttransposematrixfproc,
			multtransposematrixd: dummy_pfnglmulttransposematrixdproc,
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
extern "system" fn dummy_pfnglblendfuncseparateproc (_: GLenum, _: GLenum, _: GLenum, _: GLenum) {
	panic!("OpenGL Function pointer of `glBlendFuncSeparate()` is NULL");
}
extern "system" fn dummy_pfnglmultidrawarraysproc (_: GLenum, _: *const GLint, _: *const GLsizei, _: GLsizei) {
	panic!("OpenGL Function pointer of `glMultiDrawArrays()` is NULL");
}
extern "system" fn dummy_pfnglmultidrawelementsproc (_: GLenum, _: *const GLsizei, _: GLenum, _: *const *const c_void, _: GLsizei) {
	panic!("OpenGL Function pointer of `glMultiDrawElements()` is NULL");
}
extern "system" fn dummy_pfnglpointparameterfproc (_: GLenum, _: GLfloat) {
	panic!("OpenGL Function pointer of `glPointParameterf()` is NULL");
}
extern "system" fn dummy_pfnglpointparameterfvproc (_: GLenum, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glPointParameterfv()` is NULL");
}
extern "system" fn dummy_pfnglpointparameteriproc (_: GLenum, _: GLint) {
	panic!("OpenGL Function pointer of `glPointParameteri()` is NULL");
}
extern "system" fn dummy_pfnglpointparameterivproc (_: GLenum, _: *const GLint) {
	panic!("OpenGL Function pointer of `glPointParameteriv()` is NULL");
}
extern "system" fn dummy_pfnglfogcoordfproc (_: GLfloat) {
	panic!("OpenGL Function pointer of `glFogCoordf()` is NULL");
}
extern "system" fn dummy_pfnglfogcoordfvproc (_: *const GLfloat) {
	panic!("OpenGL Function pointer of `glFogCoordfv()` is NULL");
}
extern "system" fn dummy_pfnglfogcoorddproc (_: GLdouble) {
	panic!("OpenGL Function pointer of `glFogCoordd()` is NULL");
}
extern "system" fn dummy_pfnglfogcoorddvproc (_: *const GLdouble) {
	panic!("OpenGL Function pointer of `glFogCoorddv()` is NULL");
}
extern "system" fn dummy_pfnglfogcoordpointerproc (_: GLenum, _: GLsizei, _: *const c_void) {
	panic!("OpenGL Function pointer of `glFogCoordPointer()` is NULL");
}
extern "system" fn dummy_pfnglsecondarycolor3bproc (_: GLbyte, _: GLbyte, _: GLbyte) {
	panic!("OpenGL Function pointer of `glSecondaryColor3b()` is NULL");
}
extern "system" fn dummy_pfnglsecondarycolor3bvproc (_: *const GLbyte) {
	panic!("OpenGL Function pointer of `glSecondaryColor3bv()` is NULL");
}
extern "system" fn dummy_pfnglsecondarycolor3dproc (_: GLdouble, _: GLdouble, _: GLdouble) {
	panic!("OpenGL Function pointer of `glSecondaryColor3d()` is NULL");
}
extern "system" fn dummy_pfnglsecondarycolor3dvproc (_: *const GLdouble) {
	panic!("OpenGL Function pointer of `glSecondaryColor3dv()` is NULL");
}
extern "system" fn dummy_pfnglsecondarycolor3fproc (_: GLfloat, _: GLfloat, _: GLfloat) {
	panic!("OpenGL Function pointer of `glSecondaryColor3f()` is NULL");
}
extern "system" fn dummy_pfnglsecondarycolor3fvproc (_: *const GLfloat) {
	panic!("OpenGL Function pointer of `glSecondaryColor3fv()` is NULL");
}
extern "system" fn dummy_pfnglsecondarycolor3iproc (_: GLint, _: GLint, _: GLint) {
	panic!("OpenGL Function pointer of `glSecondaryColor3i()` is NULL");
}
extern "system" fn dummy_pfnglsecondarycolor3ivproc (_: *const GLint) {
	panic!("OpenGL Function pointer of `glSecondaryColor3iv()` is NULL");
}
extern "system" fn dummy_pfnglsecondarycolor3sproc (_: GLshort, _: GLshort, _: GLshort) {
	panic!("OpenGL Function pointer of `glSecondaryColor3s()` is NULL");
}
extern "system" fn dummy_pfnglsecondarycolor3svproc (_: *const GLshort) {
	panic!("OpenGL Function pointer of `glSecondaryColor3sv()` is NULL");
}
extern "system" fn dummy_pfnglsecondarycolor3ubproc (_: GLubyte, _: GLubyte, _: GLubyte) {
	panic!("OpenGL Function pointer of `glSecondaryColor3ub()` is NULL");
}
extern "system" fn dummy_pfnglsecondarycolor3ubvproc (_: *const GLubyte) {
	panic!("OpenGL Function pointer of `glSecondaryColor3ubv()` is NULL");
}
extern "system" fn dummy_pfnglsecondarycolor3uiproc (_: GLuint, _: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glSecondaryColor3ui()` is NULL");
}
extern "system" fn dummy_pfnglsecondarycolor3uivproc (_: *const GLuint) {
	panic!("OpenGL Function pointer of `glSecondaryColor3uiv()` is NULL");
}
extern "system" fn dummy_pfnglsecondarycolor3usproc (_: GLushort, _: GLushort, _: GLushort) {
	panic!("OpenGL Function pointer of `glSecondaryColor3us()` is NULL");
}
extern "system" fn dummy_pfnglsecondarycolor3usvproc (_: *const GLushort) {
	panic!("OpenGL Function pointer of `glSecondaryColor3usv()` is NULL");
}
extern "system" fn dummy_pfnglsecondarycolorpointerproc (_: GLint, _: GLenum, _: GLsizei, _: *const c_void) {
	panic!("OpenGL Function pointer of `glSecondaryColorPointer()` is NULL");
}
extern "system" fn dummy_pfnglwindowpos2dproc (_: GLdouble, _: GLdouble) {
	panic!("OpenGL Function pointer of `glWindowPos2d()` is NULL");
}
extern "system" fn dummy_pfnglwindowpos2dvproc (_: *const GLdouble) {
	panic!("OpenGL Function pointer of `glWindowPos2dv()` is NULL");
}
extern "system" fn dummy_pfnglwindowpos2fproc (_: GLfloat, _: GLfloat) {
	panic!("OpenGL Function pointer of `glWindowPos2f()` is NULL");
}
extern "system" fn dummy_pfnglwindowpos2fvproc (_: *const GLfloat) {
	panic!("OpenGL Function pointer of `glWindowPos2fv()` is NULL");
}
extern "system" fn dummy_pfnglwindowpos2iproc (_: GLint, _: GLint) {
	panic!("OpenGL Function pointer of `glWindowPos2i()` is NULL");
}
extern "system" fn dummy_pfnglwindowpos2ivproc (_: *const GLint) {
	panic!("OpenGL Function pointer of `glWindowPos2iv()` is NULL");
}
extern "system" fn dummy_pfnglwindowpos2sproc (_: GLshort, _: GLshort) {
	panic!("OpenGL Function pointer of `glWindowPos2s()` is NULL");
}
extern "system" fn dummy_pfnglwindowpos2svproc (_: *const GLshort) {
	panic!("OpenGL Function pointer of `glWindowPos2sv()` is NULL");
}
extern "system" fn dummy_pfnglwindowpos3dproc (_: GLdouble, _: GLdouble, _: GLdouble) {
	panic!("OpenGL Function pointer of `glWindowPos3d()` is NULL");
}
extern "system" fn dummy_pfnglwindowpos3dvproc (_: *const GLdouble) {
	panic!("OpenGL Function pointer of `glWindowPos3dv()` is NULL");
}
extern "system" fn dummy_pfnglwindowpos3fproc (_: GLfloat, _: GLfloat, _: GLfloat) {
	panic!("OpenGL Function pointer of `glWindowPos3f()` is NULL");
}
extern "system" fn dummy_pfnglwindowpos3fvproc (_: *const GLfloat) {
	panic!("OpenGL Function pointer of `glWindowPos3fv()` is NULL");
}
extern "system" fn dummy_pfnglwindowpos3iproc (_: GLint, _: GLint, _: GLint) {
	panic!("OpenGL Function pointer of `glWindowPos3i()` is NULL");
}
extern "system" fn dummy_pfnglwindowpos3ivproc (_: *const GLint) {
	panic!("OpenGL Function pointer of `glWindowPos3iv()` is NULL");
}
extern "system" fn dummy_pfnglwindowpos3sproc (_: GLshort, _: GLshort, _: GLshort) {
	panic!("OpenGL Function pointer of `glWindowPos3s()` is NULL");
}
extern "system" fn dummy_pfnglwindowpos3svproc (_: *const GLshort) {
	panic!("OpenGL Function pointer of `glWindowPos3sv()` is NULL");
}
extern "system" fn dummy_pfnglblendcolorproc (_: GLfloat, _: GLfloat, _: GLfloat, _: GLfloat) {
	panic!("OpenGL Function pointer of `glBlendColor()` is NULL");
}
extern "system" fn dummy_pfnglblendequationproc (_: GLenum) {
	panic!("OpenGL Function pointer of `glBlendEquation()` is NULL");
}
pub const GL_BLEND_DST_RGB: GLenum = 0x80C8;
pub const GL_BLEND_SRC_RGB: GLenum = 0x80C9;
pub const GL_BLEND_DST_ALPHA: GLenum = 0x80CA;
pub const GL_BLEND_SRC_ALPHA: GLenum = 0x80CB;
pub const GL_POINT_FADE_THRESHOLD_SIZE: GLenum = 0x8128;
pub const GL_DEPTH_COMPONENT16: GLenum = 0x81A5;
pub const GL_DEPTH_COMPONENT24: GLenum = 0x81A6;
pub const GL_DEPTH_COMPONENT32: GLenum = 0x81A7;
pub const GL_MIRRORED_REPEAT: GLint = 0x8370;
pub const GL_MAX_TEXTURE_LOD_BIAS: GLenum = 0x84FD;
pub const GL_TEXTURE_LOD_BIAS: GLenum = 0x8501;
pub const GL_INCR_WRAP: GLenum = 0x8507;
pub const GL_DECR_WRAP: GLenum = 0x8508;
pub const GL_TEXTURE_DEPTH_SIZE: GLenum = 0x884A;
pub const GL_TEXTURE_COMPARE_MODE: GLenum = 0x884C;
pub const GL_TEXTURE_COMPARE_FUNC: GLenum = 0x884D;
pub const GL_POINT_SIZE_MIN: GLenum = 0x8126;
pub const GL_POINT_SIZE_MAX: GLenum = 0x8127;
pub const GL_POINT_DISTANCE_ATTENUATION: GLenum = 0x8129;
pub const GL_GENERATE_MIPMAP: GLenum = 0x8191;
pub const GL_GENERATE_MIPMAP_HINT: GLenum = 0x8192;
pub const GL_FOG_COORDINATE_SOURCE: GLenum = 0x8450;
pub const GL_FOG_COORDINATE: GLenum = 0x8451;
pub const GL_FRAGMENT_DEPTH: GLenum = 0x8452;
pub const GL_CURRENT_FOG_COORDINATE: GLenum = 0x8453;
pub const GL_FOG_COORDINATE_ARRAY_TYPE: GLenum = 0x8454;
pub const GL_FOG_COORDINATE_ARRAY_STRIDE: GLenum = 0x8455;
pub const GL_FOG_COORDINATE_ARRAY_POINTER: GLenum = 0x8456;
pub const GL_FOG_COORDINATE_ARRAY: GLenum = 0x8457;
pub const GL_COLOR_SUM: GLenum = 0x8458;
pub const GL_CURRENT_SECONDARY_COLOR: GLenum = 0x8459;
pub const GL_SECONDARY_COLOR_ARRAY_SIZE: GLenum = 0x845A;
pub const GL_SECONDARY_COLOR_ARRAY_TYPE: GLenum = 0x845B;
pub const GL_SECONDARY_COLOR_ARRAY_STRIDE: GLenum = 0x845C;
pub const GL_SECONDARY_COLOR_ARRAY_POINTER: GLenum = 0x845D;
pub const GL_SECONDARY_COLOR_ARRAY: GLenum = 0x845E;
pub const GL_TEXTURE_FILTER_CONTROL: GLenum = 0x8500;
pub const GL_DEPTH_TEXTURE_MODE: GLenum = 0x884B;
pub const GL_COMPARE_R_TO_TEXTURE: GLenum = 0x884E;
pub const GL_BLEND_COLOR: GLenum = 0x8005;
pub const GL_BLEND_EQUATION: GLenum = 0x8009;
pub const GL_CONSTANT_COLOR: GLenum = 0x8001;
pub const GL_ONE_MINUS_CONSTANT_COLOR: GLenum = 0x8002;
pub const GL_CONSTANT_ALPHA: GLenum = 0x8003;
pub const GL_ONE_MINUS_CONSTANT_ALPHA: GLenum = 0x8004;
pub const GL_FUNC_ADD: GLenum = 0x8006;
pub const GL_FUNC_REVERSE_SUBTRACT: GLenum = 0x800B;
pub const GL_FUNC_SUBTRACT: GLenum = 0x800A;
pub const GL_MIN: GLenum = 0x8007;
pub const GL_MAX: GLenum = 0x8008;

pub trait GL_1_4 {
	fn glBlendFuncSeparate(&self, sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum);
	fn glMultiDrawArrays(&self, mode: GLenum, first: *const GLint, count: *const GLsizei, drawcount: GLsizei);
	fn glMultiDrawElements(&self, mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const c_void, drawcount: GLsizei);
	fn glPointParameterf(&self, pname: GLenum, param: GLfloat);
	fn glPointParameterfv(&self, pname: GLenum, params: *const GLfloat);
	fn glPointParameteri(&self, pname: GLenum, param: GLint);
	fn glPointParameteriv(&self, pname: GLenum, params: *const GLint);
	fn glFogCoordf(&self, coord: GLfloat);
	fn glFogCoordfv(&self, coord: *const GLfloat);
	fn glFogCoordd(&self, coord: GLdouble);
	fn glFogCoorddv(&self, coord: *const GLdouble);
	fn glFogCoordPointer(&self, type_: GLenum, stride: GLsizei, pointer: *const c_void);
	fn glSecondaryColor3b(&self, red: GLbyte, green: GLbyte, blue: GLbyte);
	fn glSecondaryColor3bv(&self, v: *const GLbyte);
	fn glSecondaryColor3d(&self, red: GLdouble, green: GLdouble, blue: GLdouble);
	fn glSecondaryColor3dv(&self, v: *const GLdouble);
	fn glSecondaryColor3f(&self, red: GLfloat, green: GLfloat, blue: GLfloat);
	fn glSecondaryColor3fv(&self, v: *const GLfloat);
	fn glSecondaryColor3i(&self, red: GLint, green: GLint, blue: GLint);
	fn glSecondaryColor3iv(&self, v: *const GLint);
	fn glSecondaryColor3s(&self, red: GLshort, green: GLshort, blue: GLshort);
	fn glSecondaryColor3sv(&self, v: *const GLshort);
	fn glSecondaryColor3ub(&self, red: GLubyte, green: GLubyte, blue: GLubyte);
	fn glSecondaryColor3ubv(&self, v: *const GLubyte);
	fn glSecondaryColor3ui(&self, red: GLuint, green: GLuint, blue: GLuint);
	fn glSecondaryColor3uiv(&self, v: *const GLuint);
	fn glSecondaryColor3us(&self, red: GLushort, green: GLushort, blue: GLushort);
	fn glSecondaryColor3usv(&self, v: *const GLushort);
	fn glSecondaryColorPointer(&self, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void);
	fn glWindowPos2d(&self, x: GLdouble, y: GLdouble);
	fn glWindowPos2dv(&self, v: *const GLdouble);
	fn glWindowPos2f(&self, x: GLfloat, y: GLfloat);
	fn glWindowPos2fv(&self, v: *const GLfloat);
	fn glWindowPos2i(&self, x: GLint, y: GLint);
	fn glWindowPos2iv(&self, v: *const GLint);
	fn glWindowPos2s(&self, x: GLshort, y: GLshort);
	fn glWindowPos2sv(&self, v: *const GLshort);
	fn glWindowPos3d(&self, x: GLdouble, y: GLdouble, z: GLdouble);
	fn glWindowPos3dv(&self, v: *const GLdouble);
	fn glWindowPos3f(&self, x: GLfloat, y: GLfloat, z: GLfloat);
	fn glWindowPos3fv(&self, v: *const GLfloat);
	fn glWindowPos3i(&self, x: GLint, y: GLint, z: GLint);
	fn glWindowPos3iv(&self, v: *const GLint);
	fn glWindowPos3s(&self, x: GLshort, y: GLshort, z: GLshort);
	fn glWindowPos3sv(&self, v: *const GLshort);
	fn glBlendColor(&self, red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
	fn glBlendEquation(&self, mode: GLenum);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
	pub fn new(base: impl GL_1_0, mut get_proc_address: impl FnMut(&'static str) -> *const c_void) -> Self {
		let (_spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 10400 {
			return Self::default();
		}
		Self {
			available: true,
			blendfuncseparate: {let proc = get_proc_address("glBlendFuncSeparate"); if proc == null() {dummy_pfnglblendfuncseparateproc} else {unsafe{transmute(proc)}}},
			multidrawarrays: {let proc = get_proc_address("glMultiDrawArrays"); if proc == null() {dummy_pfnglmultidrawarraysproc} else {unsafe{transmute(proc)}}},
			multidrawelements: {let proc = get_proc_address("glMultiDrawElements"); if proc == null() {dummy_pfnglmultidrawelementsproc} else {unsafe{transmute(proc)}}},
			pointparameterf: {let proc = get_proc_address("glPointParameterf"); if proc == null() {dummy_pfnglpointparameterfproc} else {unsafe{transmute(proc)}}},
			pointparameterfv: {let proc = get_proc_address("glPointParameterfv"); if proc == null() {dummy_pfnglpointparameterfvproc} else {unsafe{transmute(proc)}}},
			pointparameteri: {let proc = get_proc_address("glPointParameteri"); if proc == null() {dummy_pfnglpointparameteriproc} else {unsafe{transmute(proc)}}},
			pointparameteriv: {let proc = get_proc_address("glPointParameteriv"); if proc == null() {dummy_pfnglpointparameterivproc} else {unsafe{transmute(proc)}}},
			fogcoordf: {let proc = get_proc_address("glFogCoordf"); if proc == null() {dummy_pfnglfogcoordfproc} else {unsafe{transmute(proc)}}},
			fogcoordfv: {let proc = get_proc_address("glFogCoordfv"); if proc == null() {dummy_pfnglfogcoordfvproc} else {unsafe{transmute(proc)}}},
			fogcoordd: {let proc = get_proc_address("glFogCoordd"); if proc == null() {dummy_pfnglfogcoorddproc} else {unsafe{transmute(proc)}}},
			fogcoorddv: {let proc = get_proc_address("glFogCoorddv"); if proc == null() {dummy_pfnglfogcoorddvproc} else {unsafe{transmute(proc)}}},
			fogcoordpointer: {let proc = get_proc_address("glFogCoordPointer"); if proc == null() {dummy_pfnglfogcoordpointerproc} else {unsafe{transmute(proc)}}},
			secondarycolor3b: {let proc = get_proc_address("glSecondaryColor3b"); if proc == null() {dummy_pfnglsecondarycolor3bproc} else {unsafe{transmute(proc)}}},
			secondarycolor3bv: {let proc = get_proc_address("glSecondaryColor3bv"); if proc == null() {dummy_pfnglsecondarycolor3bvproc} else {unsafe{transmute(proc)}}},
			secondarycolor3d: {let proc = get_proc_address("glSecondaryColor3d"); if proc == null() {dummy_pfnglsecondarycolor3dproc} else {unsafe{transmute(proc)}}},
			secondarycolor3dv: {let proc = get_proc_address("glSecondaryColor3dv"); if proc == null() {dummy_pfnglsecondarycolor3dvproc} else {unsafe{transmute(proc)}}},
			secondarycolor3f: {let proc = get_proc_address("glSecondaryColor3f"); if proc == null() {dummy_pfnglsecondarycolor3fproc} else {unsafe{transmute(proc)}}},
			secondarycolor3fv: {let proc = get_proc_address("glSecondaryColor3fv"); if proc == null() {dummy_pfnglsecondarycolor3fvproc} else {unsafe{transmute(proc)}}},
			secondarycolor3i: {let proc = get_proc_address("glSecondaryColor3i"); if proc == null() {dummy_pfnglsecondarycolor3iproc} else {unsafe{transmute(proc)}}},
			secondarycolor3iv: {let proc = get_proc_address("glSecondaryColor3iv"); if proc == null() {dummy_pfnglsecondarycolor3ivproc} else {unsafe{transmute(proc)}}},
			secondarycolor3s: {let proc = get_proc_address("glSecondaryColor3s"); if proc == null() {dummy_pfnglsecondarycolor3sproc} else {unsafe{transmute(proc)}}},
			secondarycolor3sv: {let proc = get_proc_address("glSecondaryColor3sv"); if proc == null() {dummy_pfnglsecondarycolor3svproc} else {unsafe{transmute(proc)}}},
			secondarycolor3ub: {let proc = get_proc_address("glSecondaryColor3ub"); if proc == null() {dummy_pfnglsecondarycolor3ubproc} else {unsafe{transmute(proc)}}},
			secondarycolor3ubv: {let proc = get_proc_address("glSecondaryColor3ubv"); if proc == null() {dummy_pfnglsecondarycolor3ubvproc} else {unsafe{transmute(proc)}}},
			secondarycolor3ui: {let proc = get_proc_address("glSecondaryColor3ui"); if proc == null() {dummy_pfnglsecondarycolor3uiproc} else {unsafe{transmute(proc)}}},
			secondarycolor3uiv: {let proc = get_proc_address("glSecondaryColor3uiv"); if proc == null() {dummy_pfnglsecondarycolor3uivproc} else {unsafe{transmute(proc)}}},
			secondarycolor3us: {let proc = get_proc_address("glSecondaryColor3us"); if proc == null() {dummy_pfnglsecondarycolor3usproc} else {unsafe{transmute(proc)}}},
			secondarycolor3usv: {let proc = get_proc_address("glSecondaryColor3usv"); if proc == null() {dummy_pfnglsecondarycolor3usvproc} else {unsafe{transmute(proc)}}},
			secondarycolorpointer: {let proc = get_proc_address("glSecondaryColorPointer"); if proc == null() {dummy_pfnglsecondarycolorpointerproc} else {unsafe{transmute(proc)}}},
			windowpos2d: {let proc = get_proc_address("glWindowPos2d"); if proc == null() {dummy_pfnglwindowpos2dproc} else {unsafe{transmute(proc)}}},
			windowpos2dv: {let proc = get_proc_address("glWindowPos2dv"); if proc == null() {dummy_pfnglwindowpos2dvproc} else {unsafe{transmute(proc)}}},
			windowpos2f: {let proc = get_proc_address("glWindowPos2f"); if proc == null() {dummy_pfnglwindowpos2fproc} else {unsafe{transmute(proc)}}},
			windowpos2fv: {let proc = get_proc_address("glWindowPos2fv"); if proc == null() {dummy_pfnglwindowpos2fvproc} else {unsafe{transmute(proc)}}},
			windowpos2i: {let proc = get_proc_address("glWindowPos2i"); if proc == null() {dummy_pfnglwindowpos2iproc} else {unsafe{transmute(proc)}}},
			windowpos2iv: {let proc = get_proc_address("glWindowPos2iv"); if proc == null() {dummy_pfnglwindowpos2ivproc} else {unsafe{transmute(proc)}}},
			windowpos2s: {let proc = get_proc_address("glWindowPos2s"); if proc == null() {dummy_pfnglwindowpos2sproc} else {unsafe{transmute(proc)}}},
			windowpos2sv: {let proc = get_proc_address("glWindowPos2sv"); if proc == null() {dummy_pfnglwindowpos2svproc} else {unsafe{transmute(proc)}}},
			windowpos3d: {let proc = get_proc_address("glWindowPos3d"); if proc == null() {dummy_pfnglwindowpos3dproc} else {unsafe{transmute(proc)}}},
			windowpos3dv: {let proc = get_proc_address("glWindowPos3dv"); if proc == null() {dummy_pfnglwindowpos3dvproc} else {unsafe{transmute(proc)}}},
			windowpos3f: {let proc = get_proc_address("glWindowPos3f"); if proc == null() {dummy_pfnglwindowpos3fproc} else {unsafe{transmute(proc)}}},
			windowpos3fv: {let proc = get_proc_address("glWindowPos3fv"); if proc == null() {dummy_pfnglwindowpos3fvproc} else {unsafe{transmute(proc)}}},
			windowpos3i: {let proc = get_proc_address("glWindowPos3i"); if proc == null() {dummy_pfnglwindowpos3iproc} else {unsafe{transmute(proc)}}},
			windowpos3iv: {let proc = get_proc_address("glWindowPos3iv"); if proc == null() {dummy_pfnglwindowpos3ivproc} else {unsafe{transmute(proc)}}},
			windowpos3s: {let proc = get_proc_address("glWindowPos3s"); if proc == null() {dummy_pfnglwindowpos3sproc} else {unsafe{transmute(proc)}}},
			windowpos3sv: {let proc = get_proc_address("glWindowPos3sv"); if proc == null() {dummy_pfnglwindowpos3svproc} else {unsafe{transmute(proc)}}},
			blendcolor: {let proc = get_proc_address("glBlendColor"); if proc == null() {dummy_pfnglblendcolorproc} else {unsafe{transmute(proc)}}},
			blendequation: {let proc = get_proc_address("glBlendEquation"); if proc == null() {dummy_pfnglblendequationproc} else {unsafe{transmute(proc)}}},
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
			blendfuncseparate: dummy_pfnglblendfuncseparateproc,
			multidrawarrays: dummy_pfnglmultidrawarraysproc,
			multidrawelements: dummy_pfnglmultidrawelementsproc,
			pointparameterf: dummy_pfnglpointparameterfproc,
			pointparameterfv: dummy_pfnglpointparameterfvproc,
			pointparameteri: dummy_pfnglpointparameteriproc,
			pointparameteriv: dummy_pfnglpointparameterivproc,
			fogcoordf: dummy_pfnglfogcoordfproc,
			fogcoordfv: dummy_pfnglfogcoordfvproc,
			fogcoordd: dummy_pfnglfogcoorddproc,
			fogcoorddv: dummy_pfnglfogcoorddvproc,
			fogcoordpointer: dummy_pfnglfogcoordpointerproc,
			secondarycolor3b: dummy_pfnglsecondarycolor3bproc,
			secondarycolor3bv: dummy_pfnglsecondarycolor3bvproc,
			secondarycolor3d: dummy_pfnglsecondarycolor3dproc,
			secondarycolor3dv: dummy_pfnglsecondarycolor3dvproc,
			secondarycolor3f: dummy_pfnglsecondarycolor3fproc,
			secondarycolor3fv: dummy_pfnglsecondarycolor3fvproc,
			secondarycolor3i: dummy_pfnglsecondarycolor3iproc,
			secondarycolor3iv: dummy_pfnglsecondarycolor3ivproc,
			secondarycolor3s: dummy_pfnglsecondarycolor3sproc,
			secondarycolor3sv: dummy_pfnglsecondarycolor3svproc,
			secondarycolor3ub: dummy_pfnglsecondarycolor3ubproc,
			secondarycolor3ubv: dummy_pfnglsecondarycolor3ubvproc,
			secondarycolor3ui: dummy_pfnglsecondarycolor3uiproc,
			secondarycolor3uiv: dummy_pfnglsecondarycolor3uivproc,
			secondarycolor3us: dummy_pfnglsecondarycolor3usproc,
			secondarycolor3usv: dummy_pfnglsecondarycolor3usvproc,
			secondarycolorpointer: dummy_pfnglsecondarycolorpointerproc,
			windowpos2d: dummy_pfnglwindowpos2dproc,
			windowpos2dv: dummy_pfnglwindowpos2dvproc,
			windowpos2f: dummy_pfnglwindowpos2fproc,
			windowpos2fv: dummy_pfnglwindowpos2fvproc,
			windowpos2i: dummy_pfnglwindowpos2iproc,
			windowpos2iv: dummy_pfnglwindowpos2ivproc,
			windowpos2s: dummy_pfnglwindowpos2sproc,
			windowpos2sv: dummy_pfnglwindowpos2svproc,
			windowpos3d: dummy_pfnglwindowpos3dproc,
			windowpos3dv: dummy_pfnglwindowpos3dvproc,
			windowpos3f: dummy_pfnglwindowpos3fproc,
			windowpos3fv: dummy_pfnglwindowpos3fvproc,
			windowpos3i: dummy_pfnglwindowpos3iproc,
			windowpos3iv: dummy_pfnglwindowpos3ivproc,
			windowpos3s: dummy_pfnglwindowpos3sproc,
			windowpos3sv: dummy_pfnglwindowpos3svproc,
			blendcolor: dummy_pfnglblendcolorproc,
			blendequation: dummy_pfnglblendequationproc,
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
extern "system" fn dummy_pfnglgenqueriesproc (_: GLsizei, _: *mut GLuint) {
	panic!("OpenGL Function pointer of `glGenQueries()` is NULL");
}
extern "system" fn dummy_pfngldeletequeriesproc (_: GLsizei, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glDeleteQueries()` is NULL");
}
extern "system" fn dummy_pfnglisqueryproc (_: GLuint) -> GLboolean {
	panic!("OpenGL Function pointer of `glIsQuery()` is NULL");
}
extern "system" fn dummy_pfnglbeginqueryproc (_: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glBeginQuery()` is NULL");
}
extern "system" fn dummy_pfnglendqueryproc (_: GLenum) {
	panic!("OpenGL Function pointer of `glEndQuery()` is NULL");
}
extern "system" fn dummy_pfnglgetqueryivproc (_: GLenum, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetQueryiv()` is NULL");
}
extern "system" fn dummy_pfnglgetqueryobjectivproc (_: GLuint, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetQueryObjectiv()` is NULL");
}
extern "system" fn dummy_pfnglgetqueryobjectuivproc (_: GLuint, _: GLenum, _: *mut GLuint) {
	panic!("OpenGL Function pointer of `glGetQueryObjectuiv()` is NULL");
}
extern "system" fn dummy_pfnglbindbufferproc (_: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glBindBuffer()` is NULL");
}
extern "system" fn dummy_pfngldeletebuffersproc (_: GLsizei, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glDeleteBuffers()` is NULL");
}
extern "system" fn dummy_pfnglgenbuffersproc (_: GLsizei, _: *mut GLuint) {
	panic!("OpenGL Function pointer of `glGenBuffers()` is NULL");
}
extern "system" fn dummy_pfnglisbufferproc (_: GLuint) -> GLboolean {
	panic!("OpenGL Function pointer of `glIsBuffer()` is NULL");
}
extern "system" fn dummy_pfnglbufferdataproc (_: GLenum, _: GLsizeiptr, _: *const c_void, _: GLenum) {
	panic!("OpenGL Function pointer of `glBufferData()` is NULL");
}
extern "system" fn dummy_pfnglbuffersubdataproc (_: GLenum, _: GLintptr, _: GLsizeiptr, _: *const c_void) {
	panic!("OpenGL Function pointer of `glBufferSubData()` is NULL");
}
extern "system" fn dummy_pfnglgetbuffersubdataproc (_: GLenum, _: GLintptr, _: GLsizeiptr, _: *mut c_void) {
	panic!("OpenGL Function pointer of `glGetBufferSubData()` is NULL");
}
extern "system" fn dummy_pfnglmapbufferproc (_: GLenum, _: GLenum) -> *mut c_void {
	panic!("OpenGL Function pointer of `glMapBuffer()` is NULL");
}
extern "system" fn dummy_pfnglunmapbufferproc (_: GLenum) -> GLboolean {
	panic!("OpenGL Function pointer of `glUnmapBuffer()` is NULL");
}
extern "system" fn dummy_pfnglgetbufferparameterivproc (_: GLenum, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetBufferParameteriv()` is NULL");
}
extern "system" fn dummy_pfnglgetbufferpointervproc (_: GLenum, _: GLenum, _: *mut *mut c_void) {
	panic!("OpenGL Function pointer of `glGetBufferPointerv()` is NULL");
}
pub const GL_BUFFER_SIZE: GLenum = 0x8764;
pub const GL_BUFFER_USAGE: GLenum = 0x8765;
pub const GL_QUERY_COUNTER_BITS: GLenum = 0x8864;
pub const GL_CURRENT_QUERY: GLenum = 0x8865;
pub const GL_QUERY_RESULT: GLenum = 0x8866;
pub const GL_QUERY_RESULT_AVAILABLE: GLenum = 0x8867;
pub const GL_ARRAY_BUFFER: GLenum = 0x8892;
pub const GL_ELEMENT_ARRAY_BUFFER: GLenum = 0x8893;
pub const GL_ARRAY_BUFFER_BINDING: GLenum = 0x8894;
pub const GL_ELEMENT_ARRAY_BUFFER_BINDING: GLenum = 0x8895;
pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLenum = 0x889F;
pub const GL_READ_ONLY: GLenum = 0x88B8;
pub const GL_WRITE_ONLY: GLenum = 0x88B9;
pub const GL_READ_WRITE: GLenum = 0x88BA;
pub const GL_BUFFER_ACCESS: GLenum = 0x88BB;
pub const GL_BUFFER_MAPPED: GLenum = 0x88BC;
pub const GL_BUFFER_MAP_POINTER: GLenum = 0x88BD;
pub const GL_STREAM_DRAW: GLenum = 0x88E0;
pub const GL_STREAM_READ: GLenum = 0x88E1;
pub const GL_STREAM_COPY: GLenum = 0x88E2;
pub const GL_STATIC_DRAW: GLenum = 0x88E4;
pub const GL_STATIC_READ: GLenum = 0x88E5;
pub const GL_STATIC_COPY: GLenum = 0x88E6;
pub const GL_DYNAMIC_DRAW: GLenum = 0x88E8;
pub const GL_DYNAMIC_READ: GLenum = 0x88E9;
pub const GL_DYNAMIC_COPY: GLenum = 0x88EA;
pub const GL_SAMPLES_PASSED: GLenum = 0x8914;
pub const GL_SRC1_ALPHA: GLenum = 0x8589;
pub const GL_VERTEX_ARRAY_BUFFER_BINDING: GLenum = 0x8896;
pub const GL_NORMAL_ARRAY_BUFFER_BINDING: GLenum = 0x8897;
pub const GL_COLOR_ARRAY_BUFFER_BINDING: GLenum = 0x8898;
pub const GL_INDEX_ARRAY_BUFFER_BINDING: GLenum = 0x8899;
pub const GL_TEXTURE_COORD_ARRAY_BUFFER_BINDING: GLenum = 0x889A;
pub const GL_EDGE_FLAG_ARRAY_BUFFER_BINDING: GLenum = 0x889B;
pub const GL_SECONDARY_COLOR_ARRAY_BUFFER_BINDING: GLenum = 0x889C;
pub const GL_FOG_COORDINATE_ARRAY_BUFFER_BINDING: GLenum = 0x889D;
pub const GL_WEIGHT_ARRAY_BUFFER_BINDING: GLenum = 0x889E;
pub const GL_FOG_COORD_SRC: GLenum = 0x8450;
pub const GL_FOG_COORD: GLenum = 0x8451;
pub const GL_CURRENT_FOG_COORD: GLenum = 0x8453;
pub const GL_FOG_COORD_ARRAY_TYPE: GLenum = 0x8454;
pub const GL_FOG_COORD_ARRAY_STRIDE: GLenum = 0x8455;
pub const GL_FOG_COORD_ARRAY_POINTER: GLenum = 0x8456;
pub const GL_FOG_COORD_ARRAY: GLenum = 0x8457;
pub const GL_FOG_COORD_ARRAY_BUFFER_BINDING: GLenum = 0x889D;
pub const GL_SRC0_RGB: GLenum = 0x8580;
pub const GL_SRC1_RGB: GLenum = 0x8581;
pub const GL_SRC2_RGB: GLenum = 0x8582;
pub const GL_SRC0_ALPHA: GLenum = 0x8588;
pub const GL_SRC2_ALPHA: GLenum = 0x858A;

pub trait GL_1_5 {
	fn glGenQueries(&self, n: GLsizei, ids: *mut GLuint);
	fn glDeleteQueries(&self, n: GLsizei, ids: *const GLuint);
	fn glIsQuery(&self, id: GLuint) -> GLboolean;
	fn glBeginQuery(&self, target: GLenum, id: GLuint);
	fn glEndQuery(&self, target: GLenum);
	fn glGetQueryiv(&self, target: GLenum, pname: GLenum, params: *mut GLint);
	fn glGetQueryObjectiv(&self, id: GLuint, pname: GLenum, params: *mut GLint);
	fn glGetQueryObjectuiv(&self, id: GLuint, pname: GLenum, params: *mut GLuint);
	fn glBindBuffer(&self, target: GLenum, buffer: GLuint);
	fn glDeleteBuffers(&self, n: GLsizei, buffers: *const GLuint);
	fn glGenBuffers(&self, n: GLsizei, buffers: *mut GLuint);
	fn glIsBuffer(&self, buffer: GLuint) -> GLboolean;
	fn glBufferData(&self, target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum);
	fn glBufferSubData(&self, target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const c_void);
	fn glGetBufferSubData(&self, target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut c_void);
	fn glMapBuffer(&self, target: GLenum, access: GLenum) -> *mut c_void;
	fn glUnmapBuffer(&self, target: GLenum) -> GLboolean;
	fn glGetBufferParameteriv(&self, target: GLenum, pname: GLenum, params: *mut GLint);
	fn glGetBufferPointerv(&self, target: GLenum, pname: GLenum, params: *mut *mut c_void);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
	pub fn new(base: impl GL_1_0, mut get_proc_address: impl FnMut(&'static str) -> *const c_void) -> Self {
		let (_spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 10500 {
			return Self::default();
		}
		Self {
			available: true,
			genqueries: {let proc = get_proc_address("glGenQueries"); if proc == null() {dummy_pfnglgenqueriesproc} else {unsafe{transmute(proc)}}},
			deletequeries: {let proc = get_proc_address("glDeleteQueries"); if proc == null() {dummy_pfngldeletequeriesproc} else {unsafe{transmute(proc)}}},
			isquery: {let proc = get_proc_address("glIsQuery"); if proc == null() {dummy_pfnglisqueryproc} else {unsafe{transmute(proc)}}},
			beginquery: {let proc = get_proc_address("glBeginQuery"); if proc == null() {dummy_pfnglbeginqueryproc} else {unsafe{transmute(proc)}}},
			endquery: {let proc = get_proc_address("glEndQuery"); if proc == null() {dummy_pfnglendqueryproc} else {unsafe{transmute(proc)}}},
			getqueryiv: {let proc = get_proc_address("glGetQueryiv"); if proc == null() {dummy_pfnglgetqueryivproc} else {unsafe{transmute(proc)}}},
			getqueryobjectiv: {let proc = get_proc_address("glGetQueryObjectiv"); if proc == null() {dummy_pfnglgetqueryobjectivproc} else {unsafe{transmute(proc)}}},
			getqueryobjectuiv: {let proc = get_proc_address("glGetQueryObjectuiv"); if proc == null() {dummy_pfnglgetqueryobjectuivproc} else {unsafe{transmute(proc)}}},
			bindbuffer: {let proc = get_proc_address("glBindBuffer"); if proc == null() {dummy_pfnglbindbufferproc} else {unsafe{transmute(proc)}}},
			deletebuffers: {let proc = get_proc_address("glDeleteBuffers"); if proc == null() {dummy_pfngldeletebuffersproc} else {unsafe{transmute(proc)}}},
			genbuffers: {let proc = get_proc_address("glGenBuffers"); if proc == null() {dummy_pfnglgenbuffersproc} else {unsafe{transmute(proc)}}},
			isbuffer: {let proc = get_proc_address("glIsBuffer"); if proc == null() {dummy_pfnglisbufferproc} else {unsafe{transmute(proc)}}},
			bufferdata: {let proc = get_proc_address("glBufferData"); if proc == null() {dummy_pfnglbufferdataproc} else {unsafe{transmute(proc)}}},
			buffersubdata: {let proc = get_proc_address("glBufferSubData"); if proc == null() {dummy_pfnglbuffersubdataproc} else {unsafe{transmute(proc)}}},
			getbuffersubdata: {let proc = get_proc_address("glGetBufferSubData"); if proc == null() {dummy_pfnglgetbuffersubdataproc} else {unsafe{transmute(proc)}}},
			mapbuffer: {let proc = get_proc_address("glMapBuffer"); if proc == null() {dummy_pfnglmapbufferproc} else {unsafe{transmute(proc)}}},
			unmapbuffer: {let proc = get_proc_address("glUnmapBuffer"); if proc == null() {dummy_pfnglunmapbufferproc} else {unsafe{transmute(proc)}}},
			getbufferparameteriv: {let proc = get_proc_address("glGetBufferParameteriv"); if proc == null() {dummy_pfnglgetbufferparameterivproc} else {unsafe{transmute(proc)}}},
			getbufferpointerv: {let proc = get_proc_address("glGetBufferPointerv"); if proc == null() {dummy_pfnglgetbufferpointervproc} else {unsafe{transmute(proc)}}},
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
			genqueries: dummy_pfnglgenqueriesproc,
			deletequeries: dummy_pfngldeletequeriesproc,
			isquery: dummy_pfnglisqueryproc,
			beginquery: dummy_pfnglbeginqueryproc,
			endquery: dummy_pfnglendqueryproc,
			getqueryiv: dummy_pfnglgetqueryivproc,
			getqueryobjectiv: dummy_pfnglgetqueryobjectivproc,
			getqueryobjectuiv: dummy_pfnglgetqueryobjectuivproc,
			bindbuffer: dummy_pfnglbindbufferproc,
			deletebuffers: dummy_pfngldeletebuffersproc,
			genbuffers: dummy_pfnglgenbuffersproc,
			isbuffer: dummy_pfnglisbufferproc,
			bufferdata: dummy_pfnglbufferdataproc,
			buffersubdata: dummy_pfnglbuffersubdataproc,
			getbuffersubdata: dummy_pfnglgetbuffersubdataproc,
			mapbuffer: dummy_pfnglmapbufferproc,
			unmapbuffer: dummy_pfnglunmapbufferproc,
			getbufferparameteriv: dummy_pfnglgetbufferparameterivproc,
			getbufferpointerv: dummy_pfnglgetbufferpointervproc,
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
extern "system" fn dummy_pfnglblendequationseparateproc (_: GLenum, _: GLenum) {
	panic!("OpenGL Function pointer of `glBlendEquationSeparate()` is NULL");
}
extern "system" fn dummy_pfngldrawbuffersproc (_: GLsizei, _: *const GLenum) {
	panic!("OpenGL Function pointer of `glDrawBuffers()` is NULL");
}
extern "system" fn dummy_pfnglstencilopseparateproc (_: GLenum, _: GLenum, _: GLenum, _: GLenum) {
	panic!("OpenGL Function pointer of `glStencilOpSeparate()` is NULL");
}
extern "system" fn dummy_pfnglstencilfuncseparateproc (_: GLenum, _: GLenum, _: GLint, _: GLuint) {
	panic!("OpenGL Function pointer of `glStencilFuncSeparate()` is NULL");
}
extern "system" fn dummy_pfnglstencilmaskseparateproc (_: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glStencilMaskSeparate()` is NULL");
}
extern "system" fn dummy_pfnglattachshaderproc (_: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glAttachShader()` is NULL");
}
extern "system" fn dummy_pfnglbindattriblocationproc (_: GLuint, _: GLuint, _: *const GLchar) {
	panic!("OpenGL Function pointer of `glBindAttribLocation()` is NULL");
}
extern "system" fn dummy_pfnglcompileshaderproc (_: GLuint) {
	panic!("OpenGL Function pointer of `glCompileShader()` is NULL");
}
extern "system" fn dummy_pfnglcreateprogramproc () -> GLuint {
	panic!("OpenGL Function pointer of `glCreateProgram()` is NULL");
}
extern "system" fn dummy_pfnglcreateshaderproc (_: GLenum) -> GLuint {
	panic!("OpenGL Function pointer of `glCreateShader()` is NULL");
}
extern "system" fn dummy_pfngldeleteprogramproc (_: GLuint) {
	panic!("OpenGL Function pointer of `glDeleteProgram()` is NULL");
}
extern "system" fn dummy_pfngldeleteshaderproc (_: GLuint) {
	panic!("OpenGL Function pointer of `glDeleteShader()` is NULL");
}
extern "system" fn dummy_pfngldetachshaderproc (_: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glDetachShader()` is NULL");
}
extern "system" fn dummy_pfngldisablevertexattribarrayproc (_: GLuint) {
	panic!("OpenGL Function pointer of `glDisableVertexAttribArray()` is NULL");
}
extern "system" fn dummy_pfnglenablevertexattribarrayproc (_: GLuint) {
	panic!("OpenGL Function pointer of `glEnableVertexAttribArray()` is NULL");
}
extern "system" fn dummy_pfnglgetactiveattribproc (_: GLuint, _: GLuint, _: GLsizei, _: *mut GLsizei, _: *mut GLint, _: *mut GLenum, _: *mut GLchar) {
	panic!("OpenGL Function pointer of `glGetActiveAttrib()` is NULL");
}
extern "system" fn dummy_pfnglgetactiveuniformproc (_: GLuint, _: GLuint, _: GLsizei, _: *mut GLsizei, _: *mut GLint, _: *mut GLenum, _: *mut GLchar) {
	panic!("OpenGL Function pointer of `glGetActiveUniform()` is NULL");
}
extern "system" fn dummy_pfnglgetattachedshadersproc (_: GLuint, _: GLsizei, _: *mut GLsizei, _: *mut GLuint) {
	panic!("OpenGL Function pointer of `glGetAttachedShaders()` is NULL");
}
extern "system" fn dummy_pfnglgetattriblocationproc (_: GLuint, _: *const GLchar) -> GLint {
	panic!("OpenGL Function pointer of `glGetAttribLocation()` is NULL");
}
extern "system" fn dummy_pfnglgetprogramivproc (_: GLuint, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetProgramiv()` is NULL");
}
extern "system" fn dummy_pfnglgetprograminfologproc (_: GLuint, _: GLsizei, _: *mut GLsizei, _: *mut GLchar) {
	panic!("OpenGL Function pointer of `glGetProgramInfoLog()` is NULL");
}
extern "system" fn dummy_pfnglgetshaderivproc (_: GLuint, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetShaderiv()` is NULL");
}
extern "system" fn dummy_pfnglgetshaderinfologproc (_: GLuint, _: GLsizei, _: *mut GLsizei, _: *mut GLchar) {
	panic!("OpenGL Function pointer of `glGetShaderInfoLog()` is NULL");
}
extern "system" fn dummy_pfnglgetshadersourceproc (_: GLuint, _: GLsizei, _: *mut GLsizei, _: *mut GLchar) {
	panic!("OpenGL Function pointer of `glGetShaderSource()` is NULL");
}
extern "system" fn dummy_pfnglgetuniformlocationproc (_: GLuint, _: *const GLchar) -> GLint {
	panic!("OpenGL Function pointer of `glGetUniformLocation()` is NULL");
}
extern "system" fn dummy_pfnglgetuniformfvproc (_: GLuint, _: GLint, _: *mut GLfloat) {
	panic!("OpenGL Function pointer of `glGetUniformfv()` is NULL");
}
extern "system" fn dummy_pfnglgetuniformivproc (_: GLuint, _: GLint, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetUniformiv()` is NULL");
}
extern "system" fn dummy_pfnglgetvertexattribdvproc (_: GLuint, _: GLenum, _: *mut GLdouble) {
	panic!("OpenGL Function pointer of `glGetVertexAttribdv()` is NULL");
}
extern "system" fn dummy_pfnglgetvertexattribfvproc (_: GLuint, _: GLenum, _: *mut GLfloat) {
	panic!("OpenGL Function pointer of `glGetVertexAttribfv()` is NULL");
}
extern "system" fn dummy_pfnglgetvertexattribivproc (_: GLuint, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetVertexAttribiv()` is NULL");
}
extern "system" fn dummy_pfnglgetvertexattribpointervproc (_: GLuint, _: GLenum, _: *mut *mut c_void) {
	panic!("OpenGL Function pointer of `glGetVertexAttribPointerv()` is NULL");
}
extern "system" fn dummy_pfnglisprogramproc (_: GLuint) -> GLboolean {
	panic!("OpenGL Function pointer of `glIsProgram()` is NULL");
}
extern "system" fn dummy_pfnglisshaderproc (_: GLuint) -> GLboolean {
	panic!("OpenGL Function pointer of `glIsShader()` is NULL");
}
extern "system" fn dummy_pfngllinkprogramproc (_: GLuint) {
	panic!("OpenGL Function pointer of `glLinkProgram()` is NULL");
}
extern "system" fn dummy_pfnglshadersourceproc (_: GLuint, _: GLsizei, _: *const *const GLchar, _: *const GLint) {
	panic!("OpenGL Function pointer of `glShaderSource()` is NULL");
}
extern "system" fn dummy_pfngluseprogramproc (_: GLuint) {
	panic!("OpenGL Function pointer of `glUseProgram()` is NULL");
}
extern "system" fn dummy_pfngluniform1fproc (_: GLint, _: GLfloat) {
	panic!("OpenGL Function pointer of `glUniform1f()` is NULL");
}
extern "system" fn dummy_pfngluniform2fproc (_: GLint, _: GLfloat, _: GLfloat) {
	panic!("OpenGL Function pointer of `glUniform2f()` is NULL");
}
extern "system" fn dummy_pfngluniform3fproc (_: GLint, _: GLfloat, _: GLfloat, _: GLfloat) {
	panic!("OpenGL Function pointer of `glUniform3f()` is NULL");
}
extern "system" fn dummy_pfngluniform4fproc (_: GLint, _: GLfloat, _: GLfloat, _: GLfloat, _: GLfloat) {
	panic!("OpenGL Function pointer of `glUniform4f()` is NULL");
}
extern "system" fn dummy_pfngluniform1iproc (_: GLint, _: GLint) {
	panic!("OpenGL Function pointer of `glUniform1i()` is NULL");
}
extern "system" fn dummy_pfngluniform2iproc (_: GLint, _: GLint, _: GLint) {
	panic!("OpenGL Function pointer of `glUniform2i()` is NULL");
}
extern "system" fn dummy_pfngluniform3iproc (_: GLint, _: GLint, _: GLint, _: GLint) {
	panic!("OpenGL Function pointer of `glUniform3i()` is NULL");
}
extern "system" fn dummy_pfngluniform4iproc (_: GLint, _: GLint, _: GLint, _: GLint, _: GLint) {
	panic!("OpenGL Function pointer of `glUniform4i()` is NULL");
}
extern "system" fn dummy_pfngluniform1fvproc (_: GLint, _: GLsizei, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glUniform1fv()` is NULL");
}
extern "system" fn dummy_pfngluniform2fvproc (_: GLint, _: GLsizei, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glUniform2fv()` is NULL");
}
extern "system" fn dummy_pfngluniform3fvproc (_: GLint, _: GLsizei, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glUniform3fv()` is NULL");
}
extern "system" fn dummy_pfngluniform4fvproc (_: GLint, _: GLsizei, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glUniform4fv()` is NULL");
}
extern "system" fn dummy_pfngluniform1ivproc (_: GLint, _: GLsizei, _: *const GLint) {
	panic!("OpenGL Function pointer of `glUniform1iv()` is NULL");
}
extern "system" fn dummy_pfngluniform2ivproc (_: GLint, _: GLsizei, _: *const GLint) {
	panic!("OpenGL Function pointer of `glUniform2iv()` is NULL");
}
extern "system" fn dummy_pfngluniform3ivproc (_: GLint, _: GLsizei, _: *const GLint) {
	panic!("OpenGL Function pointer of `glUniform3iv()` is NULL");
}
extern "system" fn dummy_pfngluniform4ivproc (_: GLint, _: GLsizei, _: *const GLint) {
	panic!("OpenGL Function pointer of `glUniform4iv()` is NULL");
}
extern "system" fn dummy_pfngluniformmatrix2fvproc (_: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glUniformMatrix2fv()` is NULL");
}
extern "system" fn dummy_pfngluniformmatrix3fvproc (_: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glUniformMatrix3fv()` is NULL");
}
extern "system" fn dummy_pfngluniformmatrix4fvproc (_: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glUniformMatrix4fv()` is NULL");
}
extern "system" fn dummy_pfnglvalidateprogramproc (_: GLuint) {
	panic!("OpenGL Function pointer of `glValidateProgram()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib1dproc (_: GLuint, _: GLdouble) {
	panic!("OpenGL Function pointer of `glVertexAttrib1d()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib1dvproc (_: GLuint, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glVertexAttrib1dv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib1fproc (_: GLuint, _: GLfloat) {
	panic!("OpenGL Function pointer of `glVertexAttrib1f()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib1fvproc (_: GLuint, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glVertexAttrib1fv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib1sproc (_: GLuint, _: GLshort) {
	panic!("OpenGL Function pointer of `glVertexAttrib1s()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib1svproc (_: GLuint, _: *const GLshort) {
	panic!("OpenGL Function pointer of `glVertexAttrib1sv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib2dproc (_: GLuint, _: GLdouble, _: GLdouble) {
	panic!("OpenGL Function pointer of `glVertexAttrib2d()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib2dvproc (_: GLuint, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glVertexAttrib2dv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib2fproc (_: GLuint, _: GLfloat, _: GLfloat) {
	panic!("OpenGL Function pointer of `glVertexAttrib2f()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib2fvproc (_: GLuint, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glVertexAttrib2fv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib2sproc (_: GLuint, _: GLshort, _: GLshort) {
	panic!("OpenGL Function pointer of `glVertexAttrib2s()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib2svproc (_: GLuint, _: *const GLshort) {
	panic!("OpenGL Function pointer of `glVertexAttrib2sv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib3dproc (_: GLuint, _: GLdouble, _: GLdouble, _: GLdouble) {
	panic!("OpenGL Function pointer of `glVertexAttrib3d()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib3dvproc (_: GLuint, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glVertexAttrib3dv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib3fproc (_: GLuint, _: GLfloat, _: GLfloat, _: GLfloat) {
	panic!("OpenGL Function pointer of `glVertexAttrib3f()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib3fvproc (_: GLuint, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glVertexAttrib3fv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib3sproc (_: GLuint, _: GLshort, _: GLshort, _: GLshort) {
	panic!("OpenGL Function pointer of `glVertexAttrib3s()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib3svproc (_: GLuint, _: *const GLshort) {
	panic!("OpenGL Function pointer of `glVertexAttrib3sv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib4nbvproc (_: GLuint, _: *const GLbyte) {
	panic!("OpenGL Function pointer of `glVertexAttrib4Nbv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib4nivproc (_: GLuint, _: *const GLint) {
	panic!("OpenGL Function pointer of `glVertexAttrib4Niv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib4nsvproc (_: GLuint, _: *const GLshort) {
	panic!("OpenGL Function pointer of `glVertexAttrib4Nsv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib4nubproc (_: GLuint, _: GLubyte, _: GLubyte, _: GLubyte, _: GLubyte) {
	panic!("OpenGL Function pointer of `glVertexAttrib4Nub()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib4nubvproc (_: GLuint, _: *const GLubyte) {
	panic!("OpenGL Function pointer of `glVertexAttrib4Nubv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib4nuivproc (_: GLuint, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glVertexAttrib4Nuiv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib4nusvproc (_: GLuint, _: *const GLushort) {
	panic!("OpenGL Function pointer of `glVertexAttrib4Nusv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib4bvproc (_: GLuint, _: *const GLbyte) {
	panic!("OpenGL Function pointer of `glVertexAttrib4bv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib4dproc (_: GLuint, _: GLdouble, _: GLdouble, _: GLdouble, _: GLdouble) {
	panic!("OpenGL Function pointer of `glVertexAttrib4d()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib4dvproc (_: GLuint, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glVertexAttrib4dv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib4fproc (_: GLuint, _: GLfloat, _: GLfloat, _: GLfloat, _: GLfloat) {
	panic!("OpenGL Function pointer of `glVertexAttrib4f()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib4fvproc (_: GLuint, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glVertexAttrib4fv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib4ivproc (_: GLuint, _: *const GLint) {
	panic!("OpenGL Function pointer of `glVertexAttrib4iv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib4sproc (_: GLuint, _: GLshort, _: GLshort, _: GLshort, _: GLshort) {
	panic!("OpenGL Function pointer of `glVertexAttrib4s()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib4svproc (_: GLuint, _: *const GLshort) {
	panic!("OpenGL Function pointer of `glVertexAttrib4sv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib4ubvproc (_: GLuint, _: *const GLubyte) {
	panic!("OpenGL Function pointer of `glVertexAttrib4ubv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib4uivproc (_: GLuint, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glVertexAttrib4uiv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattrib4usvproc (_: GLuint, _: *const GLushort) {
	panic!("OpenGL Function pointer of `glVertexAttrib4usv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribpointerproc (_: GLuint, _: GLint, _: GLenum, _: GLboolean, _: GLsizei, _: *const c_void) {
	panic!("OpenGL Function pointer of `glVertexAttribPointer()` is NULL");
}
pub const GL_BLEND_EQUATION_RGB: GLenum = 0x8009;
pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED: GLenum = 0x8622;
pub const GL_VERTEX_ATTRIB_ARRAY_SIZE: GLenum = 0x8623;
pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE: GLenum = 0x8624;
pub const GL_VERTEX_ATTRIB_ARRAY_TYPE: GLenum = 0x8625;
pub const GL_CURRENT_VERTEX_ATTRIB: GLenum = 0x8626;
pub const GL_VERTEX_PROGRAM_POINT_SIZE: GLenum = 0x8642;
pub const GL_VERTEX_ATTRIB_ARRAY_POINTER: GLenum = 0x8645;
pub const GL_STENCIL_BACK_FUNC: GLenum = 0x8800;
pub const GL_STENCIL_BACK_FAIL: GLenum = 0x8801;
pub const GL_STENCIL_BACK_PASS_DEPTH_FAIL: GLenum = 0x8802;
pub const GL_STENCIL_BACK_PASS_DEPTH_PASS: GLenum = 0x8803;
pub const GL_MAX_DRAW_BUFFERS: GLenum = 0x8824;
pub const GL_DRAW_BUFFER0: GLenum = 0x8825;
pub const GL_DRAW_BUFFER1: GLenum = 0x8826;
pub const GL_DRAW_BUFFER2: GLenum = 0x8827;
pub const GL_DRAW_BUFFER3: GLenum = 0x8828;
pub const GL_DRAW_BUFFER4: GLenum = 0x8829;
pub const GL_DRAW_BUFFER5: GLenum = 0x882A;
pub const GL_DRAW_BUFFER6: GLenum = 0x882B;
pub const GL_DRAW_BUFFER7: GLenum = 0x882C;
pub const GL_DRAW_BUFFER8: GLenum = 0x882D;
pub const GL_DRAW_BUFFER9: GLenum = 0x882E;
pub const GL_DRAW_BUFFER10: GLenum = 0x882F;
pub const GL_DRAW_BUFFER11: GLenum = 0x8830;
pub const GL_DRAW_BUFFER12: GLenum = 0x8831;
pub const GL_DRAW_BUFFER13: GLenum = 0x8832;
pub const GL_DRAW_BUFFER14: GLenum = 0x8833;
pub const GL_DRAW_BUFFER15: GLenum = 0x8834;
pub const GL_BLEND_EQUATION_ALPHA: GLenum = 0x883D;
pub const GL_MAX_VERTEX_ATTRIBS: GLenum = 0x8869;
pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED: GLenum = 0x886A;
pub const GL_MAX_TEXTURE_IMAGE_UNITS: GLenum = 0x8872;
pub const GL_FRAGMENT_SHADER: GLenum = 0x8B30;
pub const GL_VERTEX_SHADER: GLenum = 0x8B31;
pub const GL_MAX_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8B49;
pub const GL_MAX_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8B4A;
pub const GL_MAX_VARYING_FLOATS: GLenum = 0x8B4B;
pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4C;
pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4D;
pub const GL_SHADER_TYPE: GLenum = 0x8B4F;
pub const GL_FLOAT_VEC2: GLenum = 0x8B50;
pub const GL_FLOAT_VEC3: GLenum = 0x8B51;
pub const GL_FLOAT_VEC4: GLenum = 0x8B52;
pub const GL_INT_VEC2: GLenum = 0x8B53;
pub const GL_INT_VEC3: GLenum = 0x8B54;
pub const GL_INT_VEC4: GLenum = 0x8B55;
pub const GL_BOOL: GLenum = 0x8B56;
pub const GL_BOOL_VEC2: GLenum = 0x8B57;
pub const GL_BOOL_VEC3: GLenum = 0x8B58;
pub const GL_BOOL_VEC4: GLenum = 0x8B59;
pub const GL_FLOAT_MAT2: GLenum = 0x8B5A;
pub const GL_FLOAT_MAT3: GLenum = 0x8B5B;
pub const GL_FLOAT_MAT4: GLenum = 0x8B5C;
pub const GL_SAMPLER_1D: GLenum = 0x8B5D;
pub const GL_SAMPLER_2D: GLenum = 0x8B5E;
pub const GL_SAMPLER_3D: GLenum = 0x8B5F;
pub const GL_SAMPLER_CUBE: GLenum = 0x8B60;
pub const GL_SAMPLER_1D_SHADOW: GLenum = 0x8B61;
pub const GL_SAMPLER_2D_SHADOW: GLenum = 0x8B62;
pub const GL_DELETE_STATUS: GLenum = 0x8B80;
pub const GL_COMPILE_STATUS: GLenum = 0x8B81;
pub const GL_LINK_STATUS: GLenum = 0x8B82;
pub const GL_VALIDATE_STATUS: GLenum = 0x8B83;
pub const GL_INFO_LOG_LENGTH: GLenum = 0x8B84;
pub const GL_ATTACHED_SHADERS: GLenum = 0x8B85;
pub const GL_ACTIVE_UNIFORMS: GLenum = 0x8B86;
pub const GL_ACTIVE_UNIFORM_MAX_LENGTH: GLenum = 0x8B87;
pub const GL_SHADER_SOURCE_LENGTH: GLenum = 0x8B88;
pub const GL_ACTIVE_ATTRIBUTES: GLenum = 0x8B89;
pub const GL_ACTIVE_ATTRIBUTE_MAX_LENGTH: GLenum = 0x8B8A;
pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT: GLenum = 0x8B8B;
pub const GL_SHADING_LANGUAGE_VERSION: GLenum = 0x8B8C;
pub const GL_CURRENT_PROGRAM: GLenum = 0x8B8D;
pub const GL_POINT_SPRITE_COORD_ORIGIN: GLenum = 0x8CA0;
pub const GL_LOWER_LEFT: GLenum = 0x8CA1;
pub const GL_UPPER_LEFT: GLenum = 0x8CA2;
pub const GL_STENCIL_BACK_REF: GLenum = 0x8CA3;
pub const GL_STENCIL_BACK_VALUE_MASK: GLenum = 0x8CA4;
pub const GL_STENCIL_BACK_WRITEMASK: GLenum = 0x8CA5;
pub const GL_VERTEX_PROGRAM_TWO_SIDE: GLenum = 0x8643;
pub const GL_POINT_SPRITE: GLenum = 0x8861;
pub const GL_COORD_REPLACE: GLenum = 0x8862;
pub const GL_MAX_TEXTURE_COORDS: GLenum = 0x8871;

pub trait GL_2_0 {
	fn glBlendEquationSeparate(&self, modeRGB: GLenum, modeAlpha: GLenum);
	fn glDrawBuffers(&self, n: GLsizei, bufs: *const GLenum);
	fn glStencilOpSeparate(&self, face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum);
	fn glStencilFuncSeparate(&self, face: GLenum, func: GLenum, ref_: GLint, mask: GLuint);
	fn glStencilMaskSeparate(&self, face: GLenum, mask: GLuint);
	fn glAttachShader(&self, program: GLuint, shader: GLuint);
	fn glBindAttribLocation(&self, program: GLuint, index: GLuint, name: *const GLchar);
	fn glCompileShader(&self, shader: GLuint);
	fn glCreateProgram(&self) -> GLuint;
	fn glCreateShader(&self, type_: GLenum) -> GLuint;
	fn glDeleteProgram(&self, program: GLuint);
	fn glDeleteShader(&self, shader: GLuint);
	fn glDetachShader(&self, program: GLuint, shader: GLuint);
	fn glDisableVertexAttribArray(&self, index: GLuint);
	fn glEnableVertexAttribArray(&self, index: GLuint);
	fn glGetActiveAttrib(&self, program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar);
	fn glGetActiveUniform(&self, program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar);
	fn glGetAttachedShaders(&self, program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint);
	fn glGetAttribLocation(&self, program: GLuint, name: *const GLchar) -> GLint;
	fn glGetProgramiv(&self, program: GLuint, pname: GLenum, params: *mut GLint);
	fn glGetProgramInfoLog(&self, program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);
	fn glGetShaderiv(&self, shader: GLuint, pname: GLenum, params: *mut GLint);
	fn glGetShaderInfoLog(&self, shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);
	fn glGetShaderSource(&self, shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar);
	fn glGetUniformLocation(&self, program: GLuint, name: *const GLchar) -> GLint;
	fn glGetUniformfv(&self, program: GLuint, location: GLint, params: *mut GLfloat);
	fn glGetUniformiv(&self, program: GLuint, location: GLint, params: *mut GLint);
	fn glGetVertexAttribdv(&self, index: GLuint, pname: GLenum, params: *mut GLdouble);
	fn glGetVertexAttribfv(&self, index: GLuint, pname: GLenum, params: *mut GLfloat);
	fn glGetVertexAttribiv(&self, index: GLuint, pname: GLenum, params: *mut GLint);
	fn glGetVertexAttribPointerv(&self, index: GLuint, pname: GLenum, pointer: *mut *mut c_void);
	fn glIsProgram(&self, program: GLuint) -> GLboolean;
	fn glIsShader(&self, shader: GLuint) -> GLboolean;
	fn glLinkProgram(&self, program: GLuint);
	fn glShaderSource(&self, shader: GLuint, count: GLsizei, string_: *const *const GLchar, length: *const GLint);
	fn glUseProgram(&self, program: GLuint);
	fn glUniform1f(&self, location: GLint, v0: GLfloat);
	fn glUniform2f(&self, location: GLint, v0: GLfloat, v1: GLfloat);
	fn glUniform3f(&self, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);
	fn glUniform4f(&self, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);
	fn glUniform1i(&self, location: GLint, v0: GLint);
	fn glUniform2i(&self, location: GLint, v0: GLint, v1: GLint);
	fn glUniform3i(&self, location: GLint, v0: GLint, v1: GLint, v2: GLint);
	fn glUniform4i(&self, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);
	fn glUniform1fv(&self, location: GLint, count: GLsizei, value: *const GLfloat);
	fn glUniform2fv(&self, location: GLint, count: GLsizei, value: *const GLfloat);
	fn glUniform3fv(&self, location: GLint, count: GLsizei, value: *const GLfloat);
	fn glUniform4fv(&self, location: GLint, count: GLsizei, value: *const GLfloat);
	fn glUniform1iv(&self, location: GLint, count: GLsizei, value: *const GLint);
	fn glUniform2iv(&self, location: GLint, count: GLsizei, value: *const GLint);
	fn glUniform3iv(&self, location: GLint, count: GLsizei, value: *const GLint);
	fn glUniform4iv(&self, location: GLint, count: GLsizei, value: *const GLint);
	fn glUniformMatrix2fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
	fn glUniformMatrix3fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
	fn glUniformMatrix4fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
	fn glValidateProgram(&self, program: GLuint);
	fn glVertexAttrib1d(&self, index: GLuint, x: GLdouble);
	fn glVertexAttrib1dv(&self, index: GLuint, v: *const GLdouble);
	fn glVertexAttrib1f(&self, index: GLuint, x: GLfloat);
	fn glVertexAttrib1fv(&self, index: GLuint, v: *const GLfloat);
	fn glVertexAttrib1s(&self, index: GLuint, x: GLshort);
	fn glVertexAttrib1sv(&self, index: GLuint, v: *const GLshort);
	fn glVertexAttrib2d(&self, index: GLuint, x: GLdouble, y: GLdouble);
	fn glVertexAttrib2dv(&self, index: GLuint, v: *const GLdouble);
	fn glVertexAttrib2f(&self, index: GLuint, x: GLfloat, y: GLfloat);
	fn glVertexAttrib2fv(&self, index: GLuint, v: *const GLfloat);
	fn glVertexAttrib2s(&self, index: GLuint, x: GLshort, y: GLshort);
	fn glVertexAttrib2sv(&self, index: GLuint, v: *const GLshort);
	fn glVertexAttrib3d(&self, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);
	fn glVertexAttrib3dv(&self, index: GLuint, v: *const GLdouble);
	fn glVertexAttrib3f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);
	fn glVertexAttrib3fv(&self, index: GLuint, v: *const GLfloat);
	fn glVertexAttrib3s(&self, index: GLuint, x: GLshort, y: GLshort, z: GLshort);
	fn glVertexAttrib3sv(&self, index: GLuint, v: *const GLshort);
	fn glVertexAttrib4Nbv(&self, index: GLuint, v: *const GLbyte);
	fn glVertexAttrib4Niv(&self, index: GLuint, v: *const GLint);
	fn glVertexAttrib4Nsv(&self, index: GLuint, v: *const GLshort);
	fn glVertexAttrib4Nub(&self, index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte);
	fn glVertexAttrib4Nubv(&self, index: GLuint, v: *const GLubyte);
	fn glVertexAttrib4Nuiv(&self, index: GLuint, v: *const GLuint);
	fn glVertexAttrib4Nusv(&self, index: GLuint, v: *const GLushort);
	fn glVertexAttrib4bv(&self, index: GLuint, v: *const GLbyte);
	fn glVertexAttrib4d(&self, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
	fn glVertexAttrib4dv(&self, index: GLuint, v: *const GLdouble);
	fn glVertexAttrib4f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
	fn glVertexAttrib4fv(&self, index: GLuint, v: *const GLfloat);
	fn glVertexAttrib4iv(&self, index: GLuint, v: *const GLint);
	fn glVertexAttrib4s(&self, index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort);
	fn glVertexAttrib4sv(&self, index: GLuint, v: *const GLshort);
	fn glVertexAttrib4ubv(&self, index: GLuint, v: *const GLubyte);
	fn glVertexAttrib4uiv(&self, index: GLuint, v: *const GLuint);
	fn glVertexAttrib4usv(&self, index: GLuint, v: *const GLushort);
	fn glVertexAttribPointer(&self, index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void);
	fn get_shading_language_version(&self) -> &'static str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
	pub fn new(base: impl GL_1_0, mut get_proc_address: impl FnMut(&'static str) -> *const c_void) -> Self {
		let (_spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 20000 {
			return Self::default();
		}
		Self {
			available: true,
			blendequationseparate: {let proc = get_proc_address("glBlendEquationSeparate"); if proc == null() {dummy_pfnglblendequationseparateproc} else {unsafe{transmute(proc)}}},
			drawbuffers: {let proc = get_proc_address("glDrawBuffers"); if proc == null() {dummy_pfngldrawbuffersproc} else {unsafe{transmute(proc)}}},
			stencilopseparate: {let proc = get_proc_address("glStencilOpSeparate"); if proc == null() {dummy_pfnglstencilopseparateproc} else {unsafe{transmute(proc)}}},
			stencilfuncseparate: {let proc = get_proc_address("glStencilFuncSeparate"); if proc == null() {dummy_pfnglstencilfuncseparateproc} else {unsafe{transmute(proc)}}},
			stencilmaskseparate: {let proc = get_proc_address("glStencilMaskSeparate"); if proc == null() {dummy_pfnglstencilmaskseparateproc} else {unsafe{transmute(proc)}}},
			attachshader: {let proc = get_proc_address("glAttachShader"); if proc == null() {dummy_pfnglattachshaderproc} else {unsafe{transmute(proc)}}},
			bindattriblocation: {let proc = get_proc_address("glBindAttribLocation"); if proc == null() {dummy_pfnglbindattriblocationproc} else {unsafe{transmute(proc)}}},
			compileshader: {let proc = get_proc_address("glCompileShader"); if proc == null() {dummy_pfnglcompileshaderproc} else {unsafe{transmute(proc)}}},
			createprogram: {let proc = get_proc_address("glCreateProgram"); if proc == null() {dummy_pfnglcreateprogramproc} else {unsafe{transmute(proc)}}},
			createshader: {let proc = get_proc_address("glCreateShader"); if proc == null() {dummy_pfnglcreateshaderproc} else {unsafe{transmute(proc)}}},
			deleteprogram: {let proc = get_proc_address("glDeleteProgram"); if proc == null() {dummy_pfngldeleteprogramproc} else {unsafe{transmute(proc)}}},
			deleteshader: {let proc = get_proc_address("glDeleteShader"); if proc == null() {dummy_pfngldeleteshaderproc} else {unsafe{transmute(proc)}}},
			detachshader: {let proc = get_proc_address("glDetachShader"); if proc == null() {dummy_pfngldetachshaderproc} else {unsafe{transmute(proc)}}},
			disablevertexattribarray: {let proc = get_proc_address("glDisableVertexAttribArray"); if proc == null() {dummy_pfngldisablevertexattribarrayproc} else {unsafe{transmute(proc)}}},
			enablevertexattribarray: {let proc = get_proc_address("glEnableVertexAttribArray"); if proc == null() {dummy_pfnglenablevertexattribarrayproc} else {unsafe{transmute(proc)}}},
			getactiveattrib: {let proc = get_proc_address("glGetActiveAttrib"); if proc == null() {dummy_pfnglgetactiveattribproc} else {unsafe{transmute(proc)}}},
			getactiveuniform: {let proc = get_proc_address("glGetActiveUniform"); if proc == null() {dummy_pfnglgetactiveuniformproc} else {unsafe{transmute(proc)}}},
			getattachedshaders: {let proc = get_proc_address("glGetAttachedShaders"); if proc == null() {dummy_pfnglgetattachedshadersproc} else {unsafe{transmute(proc)}}},
			getattriblocation: {let proc = get_proc_address("glGetAttribLocation"); if proc == null() {dummy_pfnglgetattriblocationproc} else {unsafe{transmute(proc)}}},
			getprogramiv: {let proc = get_proc_address("glGetProgramiv"); if proc == null() {dummy_pfnglgetprogramivproc} else {unsafe{transmute(proc)}}},
			getprograminfolog: {let proc = get_proc_address("glGetProgramInfoLog"); if proc == null() {dummy_pfnglgetprograminfologproc} else {unsafe{transmute(proc)}}},
			getshaderiv: {let proc = get_proc_address("glGetShaderiv"); if proc == null() {dummy_pfnglgetshaderivproc} else {unsafe{transmute(proc)}}},
			getshaderinfolog: {let proc = get_proc_address("glGetShaderInfoLog"); if proc == null() {dummy_pfnglgetshaderinfologproc} else {unsafe{transmute(proc)}}},
			getshadersource: {let proc = get_proc_address("glGetShaderSource"); if proc == null() {dummy_pfnglgetshadersourceproc} else {unsafe{transmute(proc)}}},
			getuniformlocation: {let proc = get_proc_address("glGetUniformLocation"); if proc == null() {dummy_pfnglgetuniformlocationproc} else {unsafe{transmute(proc)}}},
			getuniformfv: {let proc = get_proc_address("glGetUniformfv"); if proc == null() {dummy_pfnglgetuniformfvproc} else {unsafe{transmute(proc)}}},
			getuniformiv: {let proc = get_proc_address("glGetUniformiv"); if proc == null() {dummy_pfnglgetuniformivproc} else {unsafe{transmute(proc)}}},
			getvertexattribdv: {let proc = get_proc_address("glGetVertexAttribdv"); if proc == null() {dummy_pfnglgetvertexattribdvproc} else {unsafe{transmute(proc)}}},
			getvertexattribfv: {let proc = get_proc_address("glGetVertexAttribfv"); if proc == null() {dummy_pfnglgetvertexattribfvproc} else {unsafe{transmute(proc)}}},
			getvertexattribiv: {let proc = get_proc_address("glGetVertexAttribiv"); if proc == null() {dummy_pfnglgetvertexattribivproc} else {unsafe{transmute(proc)}}},
			getvertexattribpointerv: {let proc = get_proc_address("glGetVertexAttribPointerv"); if proc == null() {dummy_pfnglgetvertexattribpointervproc} else {unsafe{transmute(proc)}}},
			isprogram: {let proc = get_proc_address("glIsProgram"); if proc == null() {dummy_pfnglisprogramproc} else {unsafe{transmute(proc)}}},
			isshader: {let proc = get_proc_address("glIsShader"); if proc == null() {dummy_pfnglisshaderproc} else {unsafe{transmute(proc)}}},
			linkprogram: {let proc = get_proc_address("glLinkProgram"); if proc == null() {dummy_pfngllinkprogramproc} else {unsafe{transmute(proc)}}},
			shadersource: {let proc = get_proc_address("glShaderSource"); if proc == null() {dummy_pfnglshadersourceproc} else {unsafe{transmute(proc)}}},
			useprogram: {let proc = get_proc_address("glUseProgram"); if proc == null() {dummy_pfngluseprogramproc} else {unsafe{transmute(proc)}}},
			uniform1f: {let proc = get_proc_address("glUniform1f"); if proc == null() {dummy_pfngluniform1fproc} else {unsafe{transmute(proc)}}},
			uniform2f: {let proc = get_proc_address("glUniform2f"); if proc == null() {dummy_pfngluniform2fproc} else {unsafe{transmute(proc)}}},
			uniform3f: {let proc = get_proc_address("glUniform3f"); if proc == null() {dummy_pfngluniform3fproc} else {unsafe{transmute(proc)}}},
			uniform4f: {let proc = get_proc_address("glUniform4f"); if proc == null() {dummy_pfngluniform4fproc} else {unsafe{transmute(proc)}}},
			uniform1i: {let proc = get_proc_address("glUniform1i"); if proc == null() {dummy_pfngluniform1iproc} else {unsafe{transmute(proc)}}},
			uniform2i: {let proc = get_proc_address("glUniform2i"); if proc == null() {dummy_pfngluniform2iproc} else {unsafe{transmute(proc)}}},
			uniform3i: {let proc = get_proc_address("glUniform3i"); if proc == null() {dummy_pfngluniform3iproc} else {unsafe{transmute(proc)}}},
			uniform4i: {let proc = get_proc_address("glUniform4i"); if proc == null() {dummy_pfngluniform4iproc} else {unsafe{transmute(proc)}}},
			uniform1fv: {let proc = get_proc_address("glUniform1fv"); if proc == null() {dummy_pfngluniform1fvproc} else {unsafe{transmute(proc)}}},
			uniform2fv: {let proc = get_proc_address("glUniform2fv"); if proc == null() {dummy_pfngluniform2fvproc} else {unsafe{transmute(proc)}}},
			uniform3fv: {let proc = get_proc_address("glUniform3fv"); if proc == null() {dummy_pfngluniform3fvproc} else {unsafe{transmute(proc)}}},
			uniform4fv: {let proc = get_proc_address("glUniform4fv"); if proc == null() {dummy_pfngluniform4fvproc} else {unsafe{transmute(proc)}}},
			uniform1iv: {let proc = get_proc_address("glUniform1iv"); if proc == null() {dummy_pfngluniform1ivproc} else {unsafe{transmute(proc)}}},
			uniform2iv: {let proc = get_proc_address("glUniform2iv"); if proc == null() {dummy_pfngluniform2ivproc} else {unsafe{transmute(proc)}}},
			uniform3iv: {let proc = get_proc_address("glUniform3iv"); if proc == null() {dummy_pfngluniform3ivproc} else {unsafe{transmute(proc)}}},
			uniform4iv: {let proc = get_proc_address("glUniform4iv"); if proc == null() {dummy_pfngluniform4ivproc} else {unsafe{transmute(proc)}}},
			uniformmatrix2fv: {let proc = get_proc_address("glUniformMatrix2fv"); if proc == null() {dummy_pfngluniformmatrix2fvproc} else {unsafe{transmute(proc)}}},
			uniformmatrix3fv: {let proc = get_proc_address("glUniformMatrix3fv"); if proc == null() {dummy_pfngluniformmatrix3fvproc} else {unsafe{transmute(proc)}}},
			uniformmatrix4fv: {let proc = get_proc_address("glUniformMatrix4fv"); if proc == null() {dummy_pfngluniformmatrix4fvproc} else {unsafe{transmute(proc)}}},
			validateprogram: {let proc = get_proc_address("glValidateProgram"); if proc == null() {dummy_pfnglvalidateprogramproc} else {unsafe{transmute(proc)}}},
			vertexattrib1d: {let proc = get_proc_address("glVertexAttrib1d"); if proc == null() {dummy_pfnglvertexattrib1dproc} else {unsafe{transmute(proc)}}},
			vertexattrib1dv: {let proc = get_proc_address("glVertexAttrib1dv"); if proc == null() {dummy_pfnglvertexattrib1dvproc} else {unsafe{transmute(proc)}}},
			vertexattrib1f: {let proc = get_proc_address("glVertexAttrib1f"); if proc == null() {dummy_pfnglvertexattrib1fproc} else {unsafe{transmute(proc)}}},
			vertexattrib1fv: {let proc = get_proc_address("glVertexAttrib1fv"); if proc == null() {dummy_pfnglvertexattrib1fvproc} else {unsafe{transmute(proc)}}},
			vertexattrib1s: {let proc = get_proc_address("glVertexAttrib1s"); if proc == null() {dummy_pfnglvertexattrib1sproc} else {unsafe{transmute(proc)}}},
			vertexattrib1sv: {let proc = get_proc_address("glVertexAttrib1sv"); if proc == null() {dummy_pfnglvertexattrib1svproc} else {unsafe{transmute(proc)}}},
			vertexattrib2d: {let proc = get_proc_address("glVertexAttrib2d"); if proc == null() {dummy_pfnglvertexattrib2dproc} else {unsafe{transmute(proc)}}},
			vertexattrib2dv: {let proc = get_proc_address("glVertexAttrib2dv"); if proc == null() {dummy_pfnglvertexattrib2dvproc} else {unsafe{transmute(proc)}}},
			vertexattrib2f: {let proc = get_proc_address("glVertexAttrib2f"); if proc == null() {dummy_pfnglvertexattrib2fproc} else {unsafe{transmute(proc)}}},
			vertexattrib2fv: {let proc = get_proc_address("glVertexAttrib2fv"); if proc == null() {dummy_pfnglvertexattrib2fvproc} else {unsafe{transmute(proc)}}},
			vertexattrib2s: {let proc = get_proc_address("glVertexAttrib2s"); if proc == null() {dummy_pfnglvertexattrib2sproc} else {unsafe{transmute(proc)}}},
			vertexattrib2sv: {let proc = get_proc_address("glVertexAttrib2sv"); if proc == null() {dummy_pfnglvertexattrib2svproc} else {unsafe{transmute(proc)}}},
			vertexattrib3d: {let proc = get_proc_address("glVertexAttrib3d"); if proc == null() {dummy_pfnglvertexattrib3dproc} else {unsafe{transmute(proc)}}},
			vertexattrib3dv: {let proc = get_proc_address("glVertexAttrib3dv"); if proc == null() {dummy_pfnglvertexattrib3dvproc} else {unsafe{transmute(proc)}}},
			vertexattrib3f: {let proc = get_proc_address("glVertexAttrib3f"); if proc == null() {dummy_pfnglvertexattrib3fproc} else {unsafe{transmute(proc)}}},
			vertexattrib3fv: {let proc = get_proc_address("glVertexAttrib3fv"); if proc == null() {dummy_pfnglvertexattrib3fvproc} else {unsafe{transmute(proc)}}},
			vertexattrib3s: {let proc = get_proc_address("glVertexAttrib3s"); if proc == null() {dummy_pfnglvertexattrib3sproc} else {unsafe{transmute(proc)}}},
			vertexattrib3sv: {let proc = get_proc_address("glVertexAttrib3sv"); if proc == null() {dummy_pfnglvertexattrib3svproc} else {unsafe{transmute(proc)}}},
			vertexattrib4nbv: {let proc = get_proc_address("glVertexAttrib4Nbv"); if proc == null() {dummy_pfnglvertexattrib4nbvproc} else {unsafe{transmute(proc)}}},
			vertexattrib4niv: {let proc = get_proc_address("glVertexAttrib4Niv"); if proc == null() {dummy_pfnglvertexattrib4nivproc} else {unsafe{transmute(proc)}}},
			vertexattrib4nsv: {let proc = get_proc_address("glVertexAttrib4Nsv"); if proc == null() {dummy_pfnglvertexattrib4nsvproc} else {unsafe{transmute(proc)}}},
			vertexattrib4nub: {let proc = get_proc_address("glVertexAttrib4Nub"); if proc == null() {dummy_pfnglvertexattrib4nubproc} else {unsafe{transmute(proc)}}},
			vertexattrib4nubv: {let proc = get_proc_address("glVertexAttrib4Nubv"); if proc == null() {dummy_pfnglvertexattrib4nubvproc} else {unsafe{transmute(proc)}}},
			vertexattrib4nuiv: {let proc = get_proc_address("glVertexAttrib4Nuiv"); if proc == null() {dummy_pfnglvertexattrib4nuivproc} else {unsafe{transmute(proc)}}},
			vertexattrib4nusv: {let proc = get_proc_address("glVertexAttrib4Nusv"); if proc == null() {dummy_pfnglvertexattrib4nusvproc} else {unsafe{transmute(proc)}}},
			vertexattrib4bv: {let proc = get_proc_address("glVertexAttrib4bv"); if proc == null() {dummy_pfnglvertexattrib4bvproc} else {unsafe{transmute(proc)}}},
			vertexattrib4d: {let proc = get_proc_address("glVertexAttrib4d"); if proc == null() {dummy_pfnglvertexattrib4dproc} else {unsafe{transmute(proc)}}},
			vertexattrib4dv: {let proc = get_proc_address("glVertexAttrib4dv"); if proc == null() {dummy_pfnglvertexattrib4dvproc} else {unsafe{transmute(proc)}}},
			vertexattrib4f: {let proc = get_proc_address("glVertexAttrib4f"); if proc == null() {dummy_pfnglvertexattrib4fproc} else {unsafe{transmute(proc)}}},
			vertexattrib4fv: {let proc = get_proc_address("glVertexAttrib4fv"); if proc == null() {dummy_pfnglvertexattrib4fvproc} else {unsafe{transmute(proc)}}},
			vertexattrib4iv: {let proc = get_proc_address("glVertexAttrib4iv"); if proc == null() {dummy_pfnglvertexattrib4ivproc} else {unsafe{transmute(proc)}}},
			vertexattrib4s: {let proc = get_proc_address("glVertexAttrib4s"); if proc == null() {dummy_pfnglvertexattrib4sproc} else {unsafe{transmute(proc)}}},
			vertexattrib4sv: {let proc = get_proc_address("glVertexAttrib4sv"); if proc == null() {dummy_pfnglvertexattrib4svproc} else {unsafe{transmute(proc)}}},
			vertexattrib4ubv: {let proc = get_proc_address("glVertexAttrib4ubv"); if proc == null() {dummy_pfnglvertexattrib4ubvproc} else {unsafe{transmute(proc)}}},
			vertexattrib4uiv: {let proc = get_proc_address("glVertexAttrib4uiv"); if proc == null() {dummy_pfnglvertexattrib4uivproc} else {unsafe{transmute(proc)}}},
			vertexattrib4usv: {let proc = get_proc_address("glVertexAttrib4usv"); if proc == null() {dummy_pfnglvertexattrib4usvproc} else {unsafe{transmute(proc)}}},
			vertexattribpointer: {let proc = get_proc_address("glVertexAttribPointer"); if proc == null() {dummy_pfnglvertexattribpointerproc} else {unsafe{transmute(proc)}}},
			shading_language_version: unsafe{CStr::from_ptr(base.glGetString(GL_SHADING_LANGUAGE_VERSION) as *const i8)}.to_str().unwrap(),
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
			blendequationseparate: dummy_pfnglblendequationseparateproc,
			drawbuffers: dummy_pfngldrawbuffersproc,
			stencilopseparate: dummy_pfnglstencilopseparateproc,
			stencilfuncseparate: dummy_pfnglstencilfuncseparateproc,
			stencilmaskseparate: dummy_pfnglstencilmaskseparateproc,
			attachshader: dummy_pfnglattachshaderproc,
			bindattriblocation: dummy_pfnglbindattriblocationproc,
			compileshader: dummy_pfnglcompileshaderproc,
			createprogram: dummy_pfnglcreateprogramproc,
			createshader: dummy_pfnglcreateshaderproc,
			deleteprogram: dummy_pfngldeleteprogramproc,
			deleteshader: dummy_pfngldeleteshaderproc,
			detachshader: dummy_pfngldetachshaderproc,
			disablevertexattribarray: dummy_pfngldisablevertexattribarrayproc,
			enablevertexattribarray: dummy_pfnglenablevertexattribarrayproc,
			getactiveattrib: dummy_pfnglgetactiveattribproc,
			getactiveuniform: dummy_pfnglgetactiveuniformproc,
			getattachedshaders: dummy_pfnglgetattachedshadersproc,
			getattriblocation: dummy_pfnglgetattriblocationproc,
			getprogramiv: dummy_pfnglgetprogramivproc,
			getprograminfolog: dummy_pfnglgetprograminfologproc,
			getshaderiv: dummy_pfnglgetshaderivproc,
			getshaderinfolog: dummy_pfnglgetshaderinfologproc,
			getshadersource: dummy_pfnglgetshadersourceproc,
			getuniformlocation: dummy_pfnglgetuniformlocationproc,
			getuniformfv: dummy_pfnglgetuniformfvproc,
			getuniformiv: dummy_pfnglgetuniformivproc,
			getvertexattribdv: dummy_pfnglgetvertexattribdvproc,
			getvertexattribfv: dummy_pfnglgetvertexattribfvproc,
			getvertexattribiv: dummy_pfnglgetvertexattribivproc,
			getvertexattribpointerv: dummy_pfnglgetvertexattribpointervproc,
			isprogram: dummy_pfnglisprogramproc,
			isshader: dummy_pfnglisshaderproc,
			linkprogram: dummy_pfngllinkprogramproc,
			shadersource: dummy_pfnglshadersourceproc,
			useprogram: dummy_pfngluseprogramproc,
			uniform1f: dummy_pfngluniform1fproc,
			uniform2f: dummy_pfngluniform2fproc,
			uniform3f: dummy_pfngluniform3fproc,
			uniform4f: dummy_pfngluniform4fproc,
			uniform1i: dummy_pfngluniform1iproc,
			uniform2i: dummy_pfngluniform2iproc,
			uniform3i: dummy_pfngluniform3iproc,
			uniform4i: dummy_pfngluniform4iproc,
			uniform1fv: dummy_pfngluniform1fvproc,
			uniform2fv: dummy_pfngluniform2fvproc,
			uniform3fv: dummy_pfngluniform3fvproc,
			uniform4fv: dummy_pfngluniform4fvproc,
			uniform1iv: dummy_pfngluniform1ivproc,
			uniform2iv: dummy_pfngluniform2ivproc,
			uniform3iv: dummy_pfngluniform3ivproc,
			uniform4iv: dummy_pfngluniform4ivproc,
			uniformmatrix2fv: dummy_pfngluniformmatrix2fvproc,
			uniformmatrix3fv: dummy_pfngluniformmatrix3fvproc,
			uniformmatrix4fv: dummy_pfngluniformmatrix4fvproc,
			validateprogram: dummy_pfnglvalidateprogramproc,
			vertexattrib1d: dummy_pfnglvertexattrib1dproc,
			vertexattrib1dv: dummy_pfnglvertexattrib1dvproc,
			vertexattrib1f: dummy_pfnglvertexattrib1fproc,
			vertexattrib1fv: dummy_pfnglvertexattrib1fvproc,
			vertexattrib1s: dummy_pfnglvertexattrib1sproc,
			vertexattrib1sv: dummy_pfnglvertexattrib1svproc,
			vertexattrib2d: dummy_pfnglvertexattrib2dproc,
			vertexattrib2dv: dummy_pfnglvertexattrib2dvproc,
			vertexattrib2f: dummy_pfnglvertexattrib2fproc,
			vertexattrib2fv: dummy_pfnglvertexattrib2fvproc,
			vertexattrib2s: dummy_pfnglvertexattrib2sproc,
			vertexattrib2sv: dummy_pfnglvertexattrib2svproc,
			vertexattrib3d: dummy_pfnglvertexattrib3dproc,
			vertexattrib3dv: dummy_pfnglvertexattrib3dvproc,
			vertexattrib3f: dummy_pfnglvertexattrib3fproc,
			vertexattrib3fv: dummy_pfnglvertexattrib3fvproc,
			vertexattrib3s: dummy_pfnglvertexattrib3sproc,
			vertexattrib3sv: dummy_pfnglvertexattrib3svproc,
			vertexattrib4nbv: dummy_pfnglvertexattrib4nbvproc,
			vertexattrib4niv: dummy_pfnglvertexattrib4nivproc,
			vertexattrib4nsv: dummy_pfnglvertexattrib4nsvproc,
			vertexattrib4nub: dummy_pfnglvertexattrib4nubproc,
			vertexattrib4nubv: dummy_pfnglvertexattrib4nubvproc,
			vertexattrib4nuiv: dummy_pfnglvertexattrib4nuivproc,
			vertexattrib4nusv: dummy_pfnglvertexattrib4nusvproc,
			vertexattrib4bv: dummy_pfnglvertexattrib4bvproc,
			vertexattrib4d: dummy_pfnglvertexattrib4dproc,
			vertexattrib4dv: dummy_pfnglvertexattrib4dvproc,
			vertexattrib4f: dummy_pfnglvertexattrib4fproc,
			vertexattrib4fv: dummy_pfnglvertexattrib4fvproc,
			vertexattrib4iv: dummy_pfnglvertexattrib4ivproc,
			vertexattrib4s: dummy_pfnglvertexattrib4sproc,
			vertexattrib4sv: dummy_pfnglvertexattrib4svproc,
			vertexattrib4ubv: dummy_pfnglvertexattrib4ubvproc,
			vertexattrib4uiv: dummy_pfnglvertexattrib4uivproc,
			vertexattrib4usv: dummy_pfnglvertexattrib4usvproc,
			vertexattribpointer: dummy_pfnglvertexattribpointerproc,
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
extern "system" fn dummy_pfngluniformmatrix2x3fvproc (_: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glUniformMatrix2x3fv()` is NULL");
}
extern "system" fn dummy_pfngluniformmatrix3x2fvproc (_: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glUniformMatrix3x2fv()` is NULL");
}
extern "system" fn dummy_pfngluniformmatrix2x4fvproc (_: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glUniformMatrix2x4fv()` is NULL");
}
extern "system" fn dummy_pfngluniformmatrix4x2fvproc (_: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glUniformMatrix4x2fv()` is NULL");
}
extern "system" fn dummy_pfngluniformmatrix3x4fvproc (_: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glUniformMatrix3x4fv()` is NULL");
}
extern "system" fn dummy_pfngluniformmatrix4x3fvproc (_: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glUniformMatrix4x3fv()` is NULL");
}
pub const GL_PIXEL_PACK_BUFFER: GLenum = 0x88EB;
pub const GL_PIXEL_UNPACK_BUFFER: GLenum = 0x88EC;
pub const GL_PIXEL_PACK_BUFFER_BINDING: GLenum = 0x88ED;
pub const GL_PIXEL_UNPACK_BUFFER_BINDING: GLenum = 0x88EF;
pub const GL_FLOAT_MAT2x3: GLenum = 0x8B65;
pub const GL_FLOAT_MAT2x4: GLenum = 0x8B66;
pub const GL_FLOAT_MAT3x2: GLenum = 0x8B67;
pub const GL_FLOAT_MAT3x4: GLenum = 0x8B68;
pub const GL_FLOAT_MAT4x2: GLenum = 0x8B69;
pub const GL_FLOAT_MAT4x3: GLenum = 0x8B6A;
pub const GL_SRGB: GLenum = 0x8C40;
pub const GL_SRGB8: GLenum = 0x8C41;
pub const GL_SRGB_ALPHA: GLenum = 0x8C42;
pub const GL_SRGB8_ALPHA8: GLenum = 0x8C43;
pub const GL_COMPRESSED_SRGB: GLenum = 0x8C48;
pub const GL_COMPRESSED_SRGB_ALPHA: GLenum = 0x8C49;
pub const GL_CURRENT_RASTER_SECONDARY_COLOR: GLenum = 0x845F;
pub const GL_SLUMINANCE_ALPHA: GLenum = 0x8C44;
pub const GL_SLUMINANCE8_ALPHA8: GLenum = 0x8C45;
pub const GL_SLUMINANCE: GLenum = 0x8C46;
pub const GL_SLUMINANCE8: GLenum = 0x8C47;
pub const GL_COMPRESSED_SLUMINANCE: GLenum = 0x8C4A;
pub const GL_COMPRESSED_SLUMINANCE_ALPHA: GLenum = 0x8C4B;

pub trait GL_2_1 {
	fn glUniformMatrix2x3fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
	fn glUniformMatrix3x2fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
	fn glUniformMatrix2x4fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
	fn glUniformMatrix4x2fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
	fn glUniformMatrix3x4fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
	fn glUniformMatrix4x3fv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
	pub fn new(base: impl GL_1_0, mut get_proc_address: impl FnMut(&'static str) -> *const c_void) -> Self {
		let (_spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 20100 {
			return Self::default();
		}
		Self {
			available: true,
			uniformmatrix2x3fv: {let proc = get_proc_address("glUniformMatrix2x3fv"); if proc == null() {dummy_pfngluniformmatrix2x3fvproc} else {unsafe{transmute(proc)}}},
			uniformmatrix3x2fv: {let proc = get_proc_address("glUniformMatrix3x2fv"); if proc == null() {dummy_pfngluniformmatrix3x2fvproc} else {unsafe{transmute(proc)}}},
			uniformmatrix2x4fv: {let proc = get_proc_address("glUniformMatrix2x4fv"); if proc == null() {dummy_pfngluniformmatrix2x4fvproc} else {unsafe{transmute(proc)}}},
			uniformmatrix4x2fv: {let proc = get_proc_address("glUniformMatrix4x2fv"); if proc == null() {dummy_pfngluniformmatrix4x2fvproc} else {unsafe{transmute(proc)}}},
			uniformmatrix3x4fv: {let proc = get_proc_address("glUniformMatrix3x4fv"); if proc == null() {dummy_pfngluniformmatrix3x4fvproc} else {unsafe{transmute(proc)}}},
			uniformmatrix4x3fv: {let proc = get_proc_address("glUniformMatrix4x3fv"); if proc == null() {dummy_pfngluniformmatrix4x3fvproc} else {unsafe{transmute(proc)}}},
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
			uniformmatrix2x3fv: dummy_pfngluniformmatrix2x3fvproc,
			uniformmatrix3x2fv: dummy_pfngluniformmatrix3x2fvproc,
			uniformmatrix2x4fv: dummy_pfngluniformmatrix2x4fvproc,
			uniformmatrix4x2fv: dummy_pfngluniformmatrix4x2fvproc,
			uniformmatrix3x4fv: dummy_pfngluniformmatrix3x4fvproc,
			uniformmatrix4x3fv: dummy_pfngluniformmatrix4x3fvproc,
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
extern "system" fn dummy_pfnglcolormaskiproc (_: GLuint, _: GLboolean, _: GLboolean, _: GLboolean, _: GLboolean) {
	panic!("OpenGL Function pointer of `glColorMaski()` is NULL");
}
extern "system" fn dummy_pfnglgetbooleani_vproc (_: GLenum, _: GLuint, _: *mut GLboolean) {
	panic!("OpenGL Function pointer of `glGetBooleani_v()` is NULL");
}
extern "system" fn dummy_pfnglgetintegeri_vproc (_: GLenum, _: GLuint, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetIntegeri_v()` is NULL");
}
extern "system" fn dummy_pfnglenableiproc (_: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glEnablei()` is NULL");
}
extern "system" fn dummy_pfngldisableiproc (_: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glDisablei()` is NULL");
}
extern "system" fn dummy_pfnglisenablediproc (_: GLenum, _: GLuint) -> GLboolean {
	panic!("OpenGL Function pointer of `glIsEnabledi()` is NULL");
}
extern "system" fn dummy_pfnglbegintransformfeedbackproc (_: GLenum) {
	panic!("OpenGL Function pointer of `glBeginTransformFeedback()` is NULL");
}
extern "system" fn dummy_pfnglendtransformfeedbackproc () {
	panic!("OpenGL Function pointer of `glEndTransformFeedback()` is NULL");
}
extern "system" fn dummy_pfnglbindbufferrangeproc (_: GLenum, _: GLuint, _: GLuint, _: GLintptr, _: GLsizeiptr) {
	panic!("OpenGL Function pointer of `glBindBufferRange()` is NULL");
}
extern "system" fn dummy_pfnglbindbufferbaseproc (_: GLenum, _: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glBindBufferBase()` is NULL");
}
extern "system" fn dummy_pfngltransformfeedbackvaryingsproc (_: GLuint, _: GLsizei, _: *const *const GLchar, _: GLenum) {
	panic!("OpenGL Function pointer of `glTransformFeedbackVaryings()` is NULL");
}
extern "system" fn dummy_pfnglgettransformfeedbackvaryingproc (_: GLuint, _: GLuint, _: GLsizei, _: *mut GLsizei, _: *mut GLsizei, _: *mut GLenum, _: *mut GLchar) {
	panic!("OpenGL Function pointer of `glGetTransformFeedbackVarying()` is NULL");
}
extern "system" fn dummy_pfnglclampcolorproc (_: GLenum, _: GLenum) {
	panic!("OpenGL Function pointer of `glClampColor()` is NULL");
}
extern "system" fn dummy_pfnglbeginconditionalrenderproc (_: GLuint, _: GLenum) {
	panic!("OpenGL Function pointer of `glBeginConditionalRender()` is NULL");
}
extern "system" fn dummy_pfnglendconditionalrenderproc () {
	panic!("OpenGL Function pointer of `glEndConditionalRender()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribipointerproc (_: GLuint, _: GLint, _: GLenum, _: GLsizei, _: *const c_void) {
	panic!("OpenGL Function pointer of `glVertexAttribIPointer()` is NULL");
}
extern "system" fn dummy_pfnglgetvertexattribiivproc (_: GLuint, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetVertexAttribIiv()` is NULL");
}
extern "system" fn dummy_pfnglgetvertexattribiuivproc (_: GLuint, _: GLenum, _: *mut GLuint) {
	panic!("OpenGL Function pointer of `glGetVertexAttribIuiv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribi1iproc (_: GLuint, _: GLint) {
	panic!("OpenGL Function pointer of `glVertexAttribI1i()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribi2iproc (_: GLuint, _: GLint, _: GLint) {
	panic!("OpenGL Function pointer of `glVertexAttribI2i()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribi3iproc (_: GLuint, _: GLint, _: GLint, _: GLint) {
	panic!("OpenGL Function pointer of `glVertexAttribI3i()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribi4iproc (_: GLuint, _: GLint, _: GLint, _: GLint, _: GLint) {
	panic!("OpenGL Function pointer of `glVertexAttribI4i()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribi1uiproc (_: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glVertexAttribI1ui()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribi2uiproc (_: GLuint, _: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glVertexAttribI2ui()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribi3uiproc (_: GLuint, _: GLuint, _: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glVertexAttribI3ui()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribi4uiproc (_: GLuint, _: GLuint, _: GLuint, _: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glVertexAttribI4ui()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribi1ivproc (_: GLuint, _: *const GLint) {
	panic!("OpenGL Function pointer of `glVertexAttribI1iv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribi2ivproc (_: GLuint, _: *const GLint) {
	panic!("OpenGL Function pointer of `glVertexAttribI2iv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribi3ivproc (_: GLuint, _: *const GLint) {
	panic!("OpenGL Function pointer of `glVertexAttribI3iv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribi4ivproc (_: GLuint, _: *const GLint) {
	panic!("OpenGL Function pointer of `glVertexAttribI4iv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribi1uivproc (_: GLuint, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glVertexAttribI1uiv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribi2uivproc (_: GLuint, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glVertexAttribI2uiv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribi3uivproc (_: GLuint, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glVertexAttribI3uiv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribi4uivproc (_: GLuint, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glVertexAttribI4uiv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribi4bvproc (_: GLuint, _: *const GLbyte) {
	panic!("OpenGL Function pointer of `glVertexAttribI4bv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribi4svproc (_: GLuint, _: *const GLshort) {
	panic!("OpenGL Function pointer of `glVertexAttribI4sv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribi4ubvproc (_: GLuint, _: *const GLubyte) {
	panic!("OpenGL Function pointer of `glVertexAttribI4ubv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribi4usvproc (_: GLuint, _: *const GLushort) {
	panic!("OpenGL Function pointer of `glVertexAttribI4usv()` is NULL");
}
extern "system" fn dummy_pfnglgetuniformuivproc (_: GLuint, _: GLint, _: *mut GLuint) {
	panic!("OpenGL Function pointer of `glGetUniformuiv()` is NULL");
}
extern "system" fn dummy_pfnglbindfragdatalocationproc (_: GLuint, _: GLuint, _: *const GLchar) {
	panic!("OpenGL Function pointer of `glBindFragDataLocation()` is NULL");
}
extern "system" fn dummy_pfnglgetfragdatalocationproc (_: GLuint, _: *const GLchar) -> GLint {
	panic!("OpenGL Function pointer of `glGetFragDataLocation()` is NULL");
}
extern "system" fn dummy_pfngluniform1uiproc (_: GLint, _: GLuint) {
	panic!("OpenGL Function pointer of `glUniform1ui()` is NULL");
}
extern "system" fn dummy_pfngluniform2uiproc (_: GLint, _: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glUniform2ui()` is NULL");
}
extern "system" fn dummy_pfngluniform3uiproc (_: GLint, _: GLuint, _: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glUniform3ui()` is NULL");
}
extern "system" fn dummy_pfngluniform4uiproc (_: GLint, _: GLuint, _: GLuint, _: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glUniform4ui()` is NULL");
}
extern "system" fn dummy_pfngluniform1uivproc (_: GLint, _: GLsizei, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glUniform1uiv()` is NULL");
}
extern "system" fn dummy_pfngluniform2uivproc (_: GLint, _: GLsizei, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glUniform2uiv()` is NULL");
}
extern "system" fn dummy_pfngluniform3uivproc (_: GLint, _: GLsizei, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glUniform3uiv()` is NULL");
}
extern "system" fn dummy_pfngluniform4uivproc (_: GLint, _: GLsizei, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glUniform4uiv()` is NULL");
}
extern "system" fn dummy_pfngltexparameteriivproc (_: GLenum, _: GLenum, _: *const GLint) {
	panic!("OpenGL Function pointer of `glTexParameterIiv()` is NULL");
}
extern "system" fn dummy_pfngltexparameteriuivproc (_: GLenum, _: GLenum, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glTexParameterIuiv()` is NULL");
}
extern "system" fn dummy_pfnglgettexparameteriivproc (_: GLenum, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetTexParameterIiv()` is NULL");
}
extern "system" fn dummy_pfnglgettexparameteriuivproc (_: GLenum, _: GLenum, _: *mut GLuint) {
	panic!("OpenGL Function pointer of `glGetTexParameterIuiv()` is NULL");
}
extern "system" fn dummy_pfnglclearbufferivproc (_: GLenum, _: GLint, _: *const GLint) {
	panic!("OpenGL Function pointer of `glClearBufferiv()` is NULL");
}
extern "system" fn dummy_pfnglclearbufferuivproc (_: GLenum, _: GLint, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glClearBufferuiv()` is NULL");
}
extern "system" fn dummy_pfnglclearbufferfvproc (_: GLenum, _: GLint, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glClearBufferfv()` is NULL");
}
extern "system" fn dummy_pfnglclearbufferfiproc (_: GLenum, _: GLint, _: GLfloat, _: GLint) {
	panic!("OpenGL Function pointer of `glClearBufferfi()` is NULL");
}
extern "system" fn dummy_pfnglgetstringiproc (_: GLenum, _: GLuint) -> *const GLubyte {
	panic!("OpenGL Function pointer of `glGetStringi()` is NULL");
}
extern "system" fn dummy_pfnglisrenderbufferproc (_: GLuint) -> GLboolean {
	panic!("OpenGL Function pointer of `glIsRenderbuffer()` is NULL");
}
extern "system" fn dummy_pfnglbindrenderbufferproc (_: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glBindRenderbuffer()` is NULL");
}
extern "system" fn dummy_pfngldeleterenderbuffersproc (_: GLsizei, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glDeleteRenderbuffers()` is NULL");
}
extern "system" fn dummy_pfnglgenrenderbuffersproc (_: GLsizei, _: *mut GLuint) {
	panic!("OpenGL Function pointer of `glGenRenderbuffers()` is NULL");
}
extern "system" fn dummy_pfnglrenderbufferstorageproc (_: GLenum, _: GLenum, _: GLsizei, _: GLsizei) {
	panic!("OpenGL Function pointer of `glRenderbufferStorage()` is NULL");
}
extern "system" fn dummy_pfnglgetrenderbufferparameterivproc (_: GLenum, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetRenderbufferParameteriv()` is NULL");
}
extern "system" fn dummy_pfnglisframebufferproc (_: GLuint) -> GLboolean {
	panic!("OpenGL Function pointer of `glIsFramebuffer()` is NULL");
}
extern "system" fn dummy_pfnglbindframebufferproc (_: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glBindFramebuffer()` is NULL");
}
extern "system" fn dummy_pfngldeleteframebuffersproc (_: GLsizei, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glDeleteFramebuffers()` is NULL");
}
extern "system" fn dummy_pfnglgenframebuffersproc (_: GLsizei, _: *mut GLuint) {
	panic!("OpenGL Function pointer of `glGenFramebuffers()` is NULL");
}
extern "system" fn dummy_pfnglcheckframebufferstatusproc (_: GLenum) -> GLenum {
	panic!("OpenGL Function pointer of `glCheckFramebufferStatus()` is NULL");
}
extern "system" fn dummy_pfnglframebuffertexture1dproc (_: GLenum, _: GLenum, _: GLenum, _: GLuint, _: GLint) {
	panic!("OpenGL Function pointer of `glFramebufferTexture1D()` is NULL");
}
extern "system" fn dummy_pfnglframebuffertexture2dproc (_: GLenum, _: GLenum, _: GLenum, _: GLuint, _: GLint) {
	panic!("OpenGL Function pointer of `glFramebufferTexture2D()` is NULL");
}
extern "system" fn dummy_pfnglframebuffertexture3dproc (_: GLenum, _: GLenum, _: GLenum, _: GLuint, _: GLint, _: GLint) {
	panic!("OpenGL Function pointer of `glFramebufferTexture3D()` is NULL");
}
extern "system" fn dummy_pfnglframebufferrenderbufferproc (_: GLenum, _: GLenum, _: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glFramebufferRenderbuffer()` is NULL");
}
extern "system" fn dummy_pfnglgetframebufferattachmentparameterivproc (_: GLenum, _: GLenum, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetFramebufferAttachmentParameteriv()` is NULL");
}
extern "system" fn dummy_pfnglgeneratemipmapproc (_: GLenum) {
	panic!("OpenGL Function pointer of `glGenerateMipmap()` is NULL");
}
extern "system" fn dummy_pfnglblitframebufferproc (_: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLbitfield, _: GLenum) {
	panic!("OpenGL Function pointer of `glBlitFramebuffer()` is NULL");
}
extern "system" fn dummy_pfnglrenderbufferstoragemultisampleproc (_: GLenum, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei) {
	panic!("OpenGL Function pointer of `glRenderbufferStorageMultisample()` is NULL");
}
extern "system" fn dummy_pfnglframebuffertexturelayerproc (_: GLenum, _: GLenum, _: GLuint, _: GLint, _: GLint) {
	panic!("OpenGL Function pointer of `glFramebufferTextureLayer()` is NULL");
}
extern "system" fn dummy_pfnglmapbufferrangeproc (_: GLenum, _: GLintptr, _: GLsizeiptr, _: GLbitfield) -> *mut c_void {
	panic!("OpenGL Function pointer of `glMapBufferRange()` is NULL");
}
extern "system" fn dummy_pfnglflushmappedbufferrangeproc (_: GLenum, _: GLintptr, _: GLsizeiptr) {
	panic!("OpenGL Function pointer of `glFlushMappedBufferRange()` is NULL");
}
extern "system" fn dummy_pfnglbindvertexarrayproc (_: GLuint) {
	panic!("OpenGL Function pointer of `glBindVertexArray()` is NULL");
}
extern "system" fn dummy_pfngldeletevertexarraysproc (_: GLsizei, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glDeleteVertexArrays()` is NULL");
}
extern "system" fn dummy_pfnglgenvertexarraysproc (_: GLsizei, _: *mut GLuint) {
	panic!("OpenGL Function pointer of `glGenVertexArrays()` is NULL");
}
extern "system" fn dummy_pfnglisvertexarrayproc (_: GLuint) -> GLboolean {
	panic!("OpenGL Function pointer of `glIsVertexArray()` is NULL");
}
pub const GL_COMPARE_REF_TO_TEXTURE: GLenum = 0x884E;
pub const GL_CLIP_DISTANCE0: GLenum = 0x3000;
pub const GL_CLIP_DISTANCE1: GLenum = 0x3001;
pub const GL_CLIP_DISTANCE2: GLenum = 0x3002;
pub const GL_CLIP_DISTANCE3: GLenum = 0x3003;
pub const GL_CLIP_DISTANCE4: GLenum = 0x3004;
pub const GL_CLIP_DISTANCE5: GLenum = 0x3005;
pub const GL_CLIP_DISTANCE6: GLenum = 0x3006;
pub const GL_CLIP_DISTANCE7: GLenum = 0x3007;
pub const GL_MAX_CLIP_DISTANCES: GLenum = 0x0D32;
pub const GL_MAJOR_VERSION: GLenum = 0x821B;
pub const GL_MINOR_VERSION: GLenum = 0x821C;
pub const GL_NUM_EXTENSIONS: GLenum = 0x821D;
pub const GL_CONTEXT_FLAGS: GLenum = 0x821E;
pub const GL_COMPRESSED_RED: GLenum = 0x8225;
pub const GL_COMPRESSED_RG: GLenum = 0x8226;
pub const GL_CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: GLbitfield = 0x00000001;
pub const GL_RGBA32F: GLenum = 0x8814;
pub const GL_RGB32F: GLenum = 0x8815;
pub const GL_RGBA16F: GLenum = 0x881A;
pub const GL_RGB16F: GLenum = 0x881B;
pub const GL_VERTEX_ATTRIB_ARRAY_INTEGER: GLenum = 0x88FD;
pub const GL_MAX_ARRAY_TEXTURE_LAYERS: GLenum = 0x88FF;
pub const GL_MIN_PROGRAM_TEXEL_OFFSET: GLenum = 0x8904;
pub const GL_MAX_PROGRAM_TEXEL_OFFSET: GLenum = 0x8905;
pub const GL_CLAMP_READ_COLOR: GLenum = 0x891C;
pub const GL_FIXED_ONLY: GLenum = 0x891D;
pub const GL_MAX_VARYING_COMPONENTS: GLenum = 0x8B4B;
pub const GL_TEXTURE_1D_ARRAY: GLenum = 0x8C18;
pub const GL_PROXY_TEXTURE_1D_ARRAY: GLenum = 0x8C19;
pub const GL_TEXTURE_2D_ARRAY: GLenum = 0x8C1A;
pub const GL_PROXY_TEXTURE_2D_ARRAY: GLenum = 0x8C1B;
pub const GL_TEXTURE_BINDING_1D_ARRAY: GLenum = 0x8C1C;
pub const GL_TEXTURE_BINDING_2D_ARRAY: GLenum = 0x8C1D;
pub const GL_R11F_G11F_B10F: GLenum = 0x8C3A;
pub const GL_UNSIGNED_INT_10F_11F_11F_REV: GLenum = 0x8C3B;
pub const GL_RGB9_E5: GLenum = 0x8C3D;
pub const GL_UNSIGNED_INT_5_9_9_9_REV: GLenum = 0x8C3E;
pub const GL_TEXTURE_SHARED_SIZE: GLenum = 0x8C3F;
pub const GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: GLenum = 0x8C76;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_MODE: GLenum = 0x8C7F;
pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: GLenum = 0x8C80;
pub const GL_TRANSFORM_FEEDBACK_VARYINGS: GLenum = 0x8C83;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_START: GLenum = 0x8C84;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_SIZE: GLenum = 0x8C85;
pub const GL_PRIMITIVES_GENERATED: GLenum = 0x8C87;
pub const GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: GLenum = 0x8C88;
pub const GL_RASTERIZER_DISCARD: GLenum = 0x8C89;
pub const GL_MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: GLenum = 0x8C8A;
pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: GLenum = 0x8C8B;
pub const GL_INTERLEAVED_ATTRIBS: GLenum = 0x8C8C;
pub const GL_SEPARATE_ATTRIBS: GLenum = 0x8C8D;
pub const GL_TRANSFORM_FEEDBACK_BUFFER: GLenum = 0x8C8E;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_BINDING: GLenum = 0x8C8F;
pub const GL_RGBA32UI: GLenum = 0x8D70;
pub const GL_RGB32UI: GLenum = 0x8D71;
pub const GL_RGBA16UI: GLenum = 0x8D76;
pub const GL_RGB16UI: GLenum = 0x8D77;
pub const GL_RGBA8UI: GLenum = 0x8D7C;
pub const GL_RGB8UI: GLenum = 0x8D7D;
pub const GL_RGBA32I: GLenum = 0x8D82;
pub const GL_RGB32I: GLenum = 0x8D83;
pub const GL_RGBA16I: GLenum = 0x8D88;
pub const GL_RGB16I: GLenum = 0x8D89;
pub const GL_RGBA8I: GLenum = 0x8D8E;
pub const GL_RGB8I: GLenum = 0x8D8F;
pub const GL_RED_INTEGER: GLenum = 0x8D94;
pub const GL_GREEN_INTEGER: GLenum = 0x8D95;
pub const GL_BLUE_INTEGER: GLenum = 0x8D96;
pub const GL_RGB_INTEGER: GLenum = 0x8D98;
pub const GL_RGBA_INTEGER: GLenum = 0x8D99;
pub const GL_BGR_INTEGER: GLenum = 0x8D9A;
pub const GL_BGRA_INTEGER: GLenum = 0x8D9B;
pub const GL_SAMPLER_1D_ARRAY: GLenum = 0x8DC0;
pub const GL_SAMPLER_2D_ARRAY: GLenum = 0x8DC1;
pub const GL_SAMPLER_1D_ARRAY_SHADOW: GLenum = 0x8DC3;
pub const GL_SAMPLER_2D_ARRAY_SHADOW: GLenum = 0x8DC4;
pub const GL_SAMPLER_CUBE_SHADOW: GLenum = 0x8DC5;
pub const GL_UNSIGNED_INT_VEC2: GLenum = 0x8DC6;
pub const GL_UNSIGNED_INT_VEC3: GLenum = 0x8DC7;
pub const GL_UNSIGNED_INT_VEC4: GLenum = 0x8DC8;
pub const GL_INT_SAMPLER_1D: GLenum = 0x8DC9;
pub const GL_INT_SAMPLER_2D: GLenum = 0x8DCA;
pub const GL_INT_SAMPLER_3D: GLenum = 0x8DCB;
pub const GL_INT_SAMPLER_CUBE: GLenum = 0x8DCC;
pub const GL_INT_SAMPLER_1D_ARRAY: GLenum = 0x8DCE;
pub const GL_INT_SAMPLER_2D_ARRAY: GLenum = 0x8DCF;
pub const GL_UNSIGNED_INT_SAMPLER_1D: GLenum = 0x8DD1;
pub const GL_UNSIGNED_INT_SAMPLER_2D: GLenum = 0x8DD2;
pub const GL_UNSIGNED_INT_SAMPLER_3D: GLenum = 0x8DD3;
pub const GL_UNSIGNED_INT_SAMPLER_CUBE: GLenum = 0x8DD4;
pub const GL_UNSIGNED_INT_SAMPLER_1D_ARRAY: GLenum = 0x8DD6;
pub const GL_UNSIGNED_INT_SAMPLER_2D_ARRAY: GLenum = 0x8DD7;
pub const GL_QUERY_WAIT: GLenum = 0x8E13;
pub const GL_QUERY_NO_WAIT: GLenum = 0x8E14;
pub const GL_QUERY_BY_REGION_WAIT: GLenum = 0x8E15;
pub const GL_QUERY_BY_REGION_NO_WAIT: GLenum = 0x8E16;
pub const GL_BUFFER_ACCESS_FLAGS: GLenum = 0x911F;
pub const GL_BUFFER_MAP_LENGTH: GLenum = 0x9120;
pub const GL_BUFFER_MAP_OFFSET: GLenum = 0x9121;
pub const GL_DEPTH_COMPONENT32F: GLenum = 0x8CAC;
pub const GL_DEPTH32F_STENCIL8: GLenum = 0x8CAD;
pub const GL_FLOAT_32_UNSIGNED_INT_24_8_REV: GLenum = 0x8DAD;
pub const GL_INVALID_FRAMEBUFFER_OPERATION: GLenum = 0x0506;
pub const GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: GLenum = 0x8210;
pub const GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: GLenum = 0x8211;
pub const GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE: GLenum = 0x8212;
pub const GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: GLenum = 0x8213;
pub const GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: GLenum = 0x8214;
pub const GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: GLenum = 0x8215;
pub const GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: GLenum = 0x8216;
pub const GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: GLenum = 0x8217;
pub const GL_FRAMEBUFFER_DEFAULT: GLenum = 0x8218;
pub const GL_FRAMEBUFFER_UNDEFINED: GLenum = 0x8219;
pub const GL_DEPTH_STENCIL_ATTACHMENT: GLenum = 0x821A;
pub const GL_MAX_RENDERBUFFER_SIZE: GLenum = 0x84E8;
pub const GL_DEPTH_STENCIL: GLenum = 0x84F9;
pub const GL_UNSIGNED_INT_24_8: GLenum = 0x84FA;
pub const GL_DEPTH24_STENCIL8: GLenum = 0x88F0;
pub const GL_TEXTURE_STENCIL_SIZE: GLenum = 0x88F1;
pub const GL_TEXTURE_RED_TYPE: GLenum = 0x8C10;
pub const GL_TEXTURE_GREEN_TYPE: GLenum = 0x8C11;
pub const GL_TEXTURE_BLUE_TYPE: GLenum = 0x8C12;
pub const GL_TEXTURE_ALPHA_TYPE: GLenum = 0x8C13;
pub const GL_TEXTURE_DEPTH_TYPE: GLenum = 0x8C16;
pub const GL_UNSIGNED_NORMALIZED: GLenum = 0x8C17;
pub const GL_FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
pub const GL_DRAW_FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
pub const GL_RENDERBUFFER_BINDING: GLenum = 0x8CA7;
pub const GL_READ_FRAMEBUFFER: GLenum = 0x8CA8;
pub const GL_DRAW_FRAMEBUFFER: GLenum = 0x8CA9;
pub const GL_READ_FRAMEBUFFER_BINDING: GLenum = 0x8CAA;
pub const GL_RENDERBUFFER_SAMPLES: GLenum = 0x8CAB;
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: GLenum = 0x8CD0;
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: GLenum = 0x8CD1;
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: GLenum = 0x8CD2;
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLenum = 0x8CD3;
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: GLenum = 0x8CD4;
pub const GL_FRAMEBUFFER_COMPLETE: GLenum = 0x8CD5;
pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GLenum = 0x8CD6;
pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLenum = 0x8CD7;
pub const GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: GLenum = 0x8CDB;
pub const GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER: GLenum = 0x8CDC;
pub const GL_FRAMEBUFFER_UNSUPPORTED: GLenum = 0x8CDD;
pub const GL_MAX_COLOR_ATTACHMENTS: GLenum = 0x8CDF;
pub const GL_COLOR_ATTACHMENT0: GLenum = 0x8CE0;
pub const GL_COLOR_ATTACHMENT1: GLenum = 0x8CE1;
pub const GL_COLOR_ATTACHMENT2: GLenum = 0x8CE2;
pub const GL_COLOR_ATTACHMENT3: GLenum = 0x8CE3;
pub const GL_COLOR_ATTACHMENT4: GLenum = 0x8CE4;
pub const GL_COLOR_ATTACHMENT5: GLenum = 0x8CE5;
pub const GL_COLOR_ATTACHMENT6: GLenum = 0x8CE6;
pub const GL_COLOR_ATTACHMENT7: GLenum = 0x8CE7;
pub const GL_COLOR_ATTACHMENT8: GLenum = 0x8CE8;
pub const GL_COLOR_ATTACHMENT9: GLenum = 0x8CE9;
pub const GL_COLOR_ATTACHMENT10: GLenum = 0x8CEA;
pub const GL_COLOR_ATTACHMENT11: GLenum = 0x8CEB;
pub const GL_COLOR_ATTACHMENT12: GLenum = 0x8CEC;
pub const GL_COLOR_ATTACHMENT13: GLenum = 0x8CED;
pub const GL_COLOR_ATTACHMENT14: GLenum = 0x8CEE;
pub const GL_COLOR_ATTACHMENT15: GLenum = 0x8CEF;
pub const GL_COLOR_ATTACHMENT16: GLenum = 0x8CF0;
pub const GL_COLOR_ATTACHMENT17: GLenum = 0x8CF1;
pub const GL_COLOR_ATTACHMENT18: GLenum = 0x8CF2;
pub const GL_COLOR_ATTACHMENT19: GLenum = 0x8CF3;
pub const GL_COLOR_ATTACHMENT20: GLenum = 0x8CF4;
pub const GL_COLOR_ATTACHMENT21: GLenum = 0x8CF5;
pub const GL_COLOR_ATTACHMENT22: GLenum = 0x8CF6;
pub const GL_COLOR_ATTACHMENT23: GLenum = 0x8CF7;
pub const GL_COLOR_ATTACHMENT24: GLenum = 0x8CF8;
pub const GL_COLOR_ATTACHMENT25: GLenum = 0x8CF9;
pub const GL_COLOR_ATTACHMENT26: GLenum = 0x8CFA;
pub const GL_COLOR_ATTACHMENT27: GLenum = 0x8CFB;
pub const GL_COLOR_ATTACHMENT28: GLenum = 0x8CFC;
pub const GL_COLOR_ATTACHMENT29: GLenum = 0x8CFD;
pub const GL_COLOR_ATTACHMENT30: GLenum = 0x8CFE;
pub const GL_COLOR_ATTACHMENT31: GLenum = 0x8CFF;
pub const GL_DEPTH_ATTACHMENT: GLenum = 0x8D00;
pub const GL_STENCIL_ATTACHMENT: GLenum = 0x8D20;
pub const GL_FRAMEBUFFER: GLenum = 0x8D40;
pub const GL_RENDERBUFFER: GLenum = 0x8D41;
pub const GL_RENDERBUFFER_WIDTH: GLenum = 0x8D42;
pub const GL_RENDERBUFFER_HEIGHT: GLenum = 0x8D43;
pub const GL_RENDERBUFFER_INTERNAL_FORMAT: GLenum = 0x8D44;
pub const GL_STENCIL_INDEX1: GLenum = 0x8D46;
pub const GL_STENCIL_INDEX4: GLenum = 0x8D47;
pub const GL_STENCIL_INDEX8: GLenum = 0x8D48;
pub const GL_STENCIL_INDEX16: GLenum = 0x8D49;
pub const GL_RENDERBUFFER_RED_SIZE: GLenum = 0x8D50;
pub const GL_RENDERBUFFER_GREEN_SIZE: GLenum = 0x8D51;
pub const GL_RENDERBUFFER_BLUE_SIZE: GLenum = 0x8D52;
pub const GL_RENDERBUFFER_ALPHA_SIZE: GLenum = 0x8D53;
pub const GL_RENDERBUFFER_DEPTH_SIZE: GLenum = 0x8D54;
pub const GL_RENDERBUFFER_STENCIL_SIZE: GLenum = 0x8D55;
pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: GLenum = 0x8D56;
pub const GL_MAX_SAMPLES: GLenum = 0x8D57;
pub const GL_INDEX: GLenum = 0x8222;
pub const GL_TEXTURE_LUMINANCE_TYPE: GLenum = 0x8C14;
pub const GL_TEXTURE_INTENSITY_TYPE: GLenum = 0x8C15;
pub const GL_FRAMEBUFFER_SRGB: GLenum = 0x8DB9;
pub const GL_HALF_FLOAT: GLenum = 0x140B;
pub const GL_MAP_READ_BIT: GLbitfield = 0x0001;
pub const GL_MAP_WRITE_BIT: GLbitfield = 0x0002;
pub const GL_MAP_INVALIDATE_RANGE_BIT: GLbitfield = 0x0004;
pub const GL_MAP_INVALIDATE_BUFFER_BIT: GLbitfield = 0x0008;
pub const GL_MAP_FLUSH_EXPLICIT_BIT: GLbitfield = 0x0010;
pub const GL_MAP_UNSYNCHRONIZED_BIT: GLbitfield = 0x0020;
pub const GL_COMPRESSED_RED_RGTC1: GLenum = 0x8DBB;
pub const GL_COMPRESSED_SIGNED_RED_RGTC1: GLenum = 0x8DBC;
pub const GL_COMPRESSED_RG_RGTC2: GLenum = 0x8DBD;
pub const GL_COMPRESSED_SIGNED_RG_RGTC2: GLenum = 0x8DBE;
pub const GL_RG: GLenum = 0x8227;
pub const GL_RG_INTEGER: GLenum = 0x8228;
pub const GL_R8: GLenum = 0x8229;
pub const GL_R16: GLenum = 0x822A;
pub const GL_RG8: GLenum = 0x822B;
pub const GL_RG16: GLenum = 0x822C;
pub const GL_R16F: GLenum = 0x822D;
pub const GL_R32F: GLenum = 0x822E;
pub const GL_RG16F: GLenum = 0x822F;
pub const GL_RG32F: GLenum = 0x8230;
pub const GL_R8I: GLenum = 0x8231;
pub const GL_R8UI: GLenum = 0x8232;
pub const GL_R16I: GLenum = 0x8233;
pub const GL_R16UI: GLenum = 0x8234;
pub const GL_R32I: GLenum = 0x8235;
pub const GL_R32UI: GLenum = 0x8236;
pub const GL_RG8I: GLenum = 0x8237;
pub const GL_RG8UI: GLenum = 0x8238;
pub const GL_RG16I: GLenum = 0x8239;
pub const GL_RG16UI: GLenum = 0x823A;
pub const GL_RG32I: GLenum = 0x823B;
pub const GL_RG32UI: GLenum = 0x823C;
pub const GL_VERTEX_ARRAY_BINDING: GLenum = 0x85B5;
pub const GL_CLAMP_VERTEX_COLOR: GLenum = 0x891A;
pub const GL_CLAMP_FRAGMENT_COLOR: GLenum = 0x891B;
pub const GL_ALPHA_INTEGER: GLenum = 0x8D97;

pub trait GL_3_0 {
	fn glColorMaski(&self, index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean);
	fn glGetBooleani_v(&self, target: GLenum, index: GLuint, data: *mut GLboolean);
	fn glGetIntegeri_v(&self, target: GLenum, index: GLuint, data: *mut GLint);
	fn glEnablei(&self, target: GLenum, index: GLuint);
	fn glDisablei(&self, target: GLenum, index: GLuint);
	fn glIsEnabledi(&self, target: GLenum, index: GLuint) -> GLboolean;
	fn glBeginTransformFeedback(&self, primitiveMode: GLenum);
	fn glEndTransformFeedback(&self);
	fn glBindBufferRange(&self, target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);
	fn glBindBufferBase(&self, target: GLenum, index: GLuint, buffer: GLuint);
	fn glTransformFeedbackVaryings(&self, program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: GLenum);
	fn glGetTransformFeedbackVarying(&self, program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut GLenum, name: *mut GLchar);
	fn glClampColor(&self, target: GLenum, clamp: GLenum);
	fn glBeginConditionalRender(&self, id: GLuint, mode: GLenum);
	fn glEndConditionalRender(&self);
	fn glVertexAttribIPointer(&self, index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void);
	fn glGetVertexAttribIiv(&self, index: GLuint, pname: GLenum, params: *mut GLint);
	fn glGetVertexAttribIuiv(&self, index: GLuint, pname: GLenum, params: *mut GLuint);
	fn glVertexAttribI1i(&self, index: GLuint, x: GLint);
	fn glVertexAttribI2i(&self, index: GLuint, x: GLint, y: GLint);
	fn glVertexAttribI3i(&self, index: GLuint, x: GLint, y: GLint, z: GLint);
	fn glVertexAttribI4i(&self, index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);
	fn glVertexAttribI1ui(&self, index: GLuint, x: GLuint);
	fn glVertexAttribI2ui(&self, index: GLuint, x: GLuint, y: GLuint);
	fn glVertexAttribI3ui(&self, index: GLuint, x: GLuint, y: GLuint, z: GLuint);
	fn glVertexAttribI4ui(&self, index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);
	fn glVertexAttribI1iv(&self, index: GLuint, v: *const GLint);
	fn glVertexAttribI2iv(&self, index: GLuint, v: *const GLint);
	fn glVertexAttribI3iv(&self, index: GLuint, v: *const GLint);
	fn glVertexAttribI4iv(&self, index: GLuint, v: *const GLint);
	fn glVertexAttribI1uiv(&self, index: GLuint, v: *const GLuint);
	fn glVertexAttribI2uiv(&self, index: GLuint, v: *const GLuint);
	fn glVertexAttribI3uiv(&self, index: GLuint, v: *const GLuint);
	fn glVertexAttribI4uiv(&self, index: GLuint, v: *const GLuint);
	fn glVertexAttribI4bv(&self, index: GLuint, v: *const GLbyte);
	fn glVertexAttribI4sv(&self, index: GLuint, v: *const GLshort);
	fn glVertexAttribI4ubv(&self, index: GLuint, v: *const GLubyte);
	fn glVertexAttribI4usv(&self, index: GLuint, v: *const GLushort);
	fn glGetUniformuiv(&self, program: GLuint, location: GLint, params: *mut GLuint);
	fn glBindFragDataLocation(&self, program: GLuint, color: GLuint, name: *const GLchar);
	fn glGetFragDataLocation(&self, program: GLuint, name: *const GLchar) -> GLint;
	fn glUniform1ui(&self, location: GLint, v0: GLuint);
	fn glUniform2ui(&self, location: GLint, v0: GLuint, v1: GLuint);
	fn glUniform3ui(&self, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);
	fn glUniform4ui(&self, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);
	fn glUniform1uiv(&self, location: GLint, count: GLsizei, value: *const GLuint);
	fn glUniform2uiv(&self, location: GLint, count: GLsizei, value: *const GLuint);
	fn glUniform3uiv(&self, location: GLint, count: GLsizei, value: *const GLuint);
	fn glUniform4uiv(&self, location: GLint, count: GLsizei, value: *const GLuint);
	fn glTexParameterIiv(&self, target: GLenum, pname: GLenum, params: *const GLint);
	fn glTexParameterIuiv(&self, target: GLenum, pname: GLenum, params: *const GLuint);
	fn glGetTexParameterIiv(&self, target: GLenum, pname: GLenum, params: *mut GLint);
	fn glGetTexParameterIuiv(&self, target: GLenum, pname: GLenum, params: *mut GLuint);
	fn glClearBufferiv(&self, buffer: GLenum, drawbuffer: GLint, value: *const GLint);
	fn glClearBufferuiv(&self, buffer: GLenum, drawbuffer: GLint, value: *const GLuint);
	fn glClearBufferfv(&self, buffer: GLenum, drawbuffer: GLint, value: *const GLfloat);
	fn glClearBufferfi(&self, buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint);
	fn glGetStringi(&self, name: GLenum, index: GLuint) -> *const GLubyte;
	fn glIsRenderbuffer(&self, renderbuffer: GLuint) -> GLboolean;
	fn glBindRenderbuffer(&self, target: GLenum, renderbuffer: GLuint);
	fn glDeleteRenderbuffers(&self, n: GLsizei, renderbuffers: *const GLuint);
	fn glGenRenderbuffers(&self, n: GLsizei, renderbuffers: *mut GLuint);
	fn glRenderbufferStorage(&self, target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei);
	fn glGetRenderbufferParameteriv(&self, target: GLenum, pname: GLenum, params: *mut GLint);
	fn glIsFramebuffer(&self, framebuffer: GLuint) -> GLboolean;
	fn glBindFramebuffer(&self, target: GLenum, framebuffer: GLuint);
	fn glDeleteFramebuffers(&self, n: GLsizei, framebuffers: *const GLuint);
	fn glGenFramebuffers(&self, n: GLsizei, framebuffers: *mut GLuint);
	fn glCheckFramebufferStatus(&self, target: GLenum) -> GLenum;
	fn glFramebufferTexture1D(&self, target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint);
	fn glFramebufferTexture2D(&self, target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint);
	fn glFramebufferTexture3D(&self, target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint);
	fn glFramebufferRenderbuffer(&self, target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint);
	fn glGetFramebufferAttachmentParameteriv(&self, target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint);
	fn glGenerateMipmap(&self, target: GLenum);
	fn glBlitFramebuffer(&self, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum);
	fn glRenderbufferStorageMultisample(&self, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
	fn glFramebufferTextureLayer(&self, target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint);
	fn glMapBufferRange(&self, target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut c_void;
	fn glFlushMappedBufferRange(&self, target: GLenum, offset: GLintptr, length: GLsizeiptr);
	fn glBindVertexArray(&self, array: GLuint);
	fn glDeleteVertexArrays(&self, n: GLsizei, arrays: *const GLuint);
	fn glGenVertexArrays(&self, n: GLsizei, arrays: *mut GLuint);
	fn glIsVertexArray(&self, array: GLuint) -> GLboolean;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
	pub fn new(base: impl GL_1_0, mut get_proc_address: impl FnMut(&'static str) -> *const c_void) -> Self {
		let (_spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 30000 {
			return Self::default();
		}
		Self {
			available: true,
			colormaski: {let proc = get_proc_address("glColorMaski"); if proc == null() {dummy_pfnglcolormaskiproc} else {unsafe{transmute(proc)}}},
			getbooleani_v: {let proc = get_proc_address("glGetBooleani_v"); if proc == null() {dummy_pfnglgetbooleani_vproc} else {unsafe{transmute(proc)}}},
			getintegeri_v: {let proc = get_proc_address("glGetIntegeri_v"); if proc == null() {dummy_pfnglgetintegeri_vproc} else {unsafe{transmute(proc)}}},
			enablei: {let proc = get_proc_address("glEnablei"); if proc == null() {dummy_pfnglenableiproc} else {unsafe{transmute(proc)}}},
			disablei: {let proc = get_proc_address("glDisablei"); if proc == null() {dummy_pfngldisableiproc} else {unsafe{transmute(proc)}}},
			isenabledi: {let proc = get_proc_address("glIsEnabledi"); if proc == null() {dummy_pfnglisenablediproc} else {unsafe{transmute(proc)}}},
			begintransformfeedback: {let proc = get_proc_address("glBeginTransformFeedback"); if proc == null() {dummy_pfnglbegintransformfeedbackproc} else {unsafe{transmute(proc)}}},
			endtransformfeedback: {let proc = get_proc_address("glEndTransformFeedback"); if proc == null() {dummy_pfnglendtransformfeedbackproc} else {unsafe{transmute(proc)}}},
			bindbufferrange: {let proc = get_proc_address("glBindBufferRange"); if proc == null() {dummy_pfnglbindbufferrangeproc} else {unsafe{transmute(proc)}}},
			bindbufferbase: {let proc = get_proc_address("glBindBufferBase"); if proc == null() {dummy_pfnglbindbufferbaseproc} else {unsafe{transmute(proc)}}},
			transformfeedbackvaryings: {let proc = get_proc_address("glTransformFeedbackVaryings"); if proc == null() {dummy_pfngltransformfeedbackvaryingsproc} else {unsafe{transmute(proc)}}},
			gettransformfeedbackvarying: {let proc = get_proc_address("glGetTransformFeedbackVarying"); if proc == null() {dummy_pfnglgettransformfeedbackvaryingproc} else {unsafe{transmute(proc)}}},
			clampcolor: {let proc = get_proc_address("glClampColor"); if proc == null() {dummy_pfnglclampcolorproc} else {unsafe{transmute(proc)}}},
			beginconditionalrender: {let proc = get_proc_address("glBeginConditionalRender"); if proc == null() {dummy_pfnglbeginconditionalrenderproc} else {unsafe{transmute(proc)}}},
			endconditionalrender: {let proc = get_proc_address("glEndConditionalRender"); if proc == null() {dummy_pfnglendconditionalrenderproc} else {unsafe{transmute(proc)}}},
			vertexattribipointer: {let proc = get_proc_address("glVertexAttribIPointer"); if proc == null() {dummy_pfnglvertexattribipointerproc} else {unsafe{transmute(proc)}}},
			getvertexattribiiv: {let proc = get_proc_address("glGetVertexAttribIiv"); if proc == null() {dummy_pfnglgetvertexattribiivproc} else {unsafe{transmute(proc)}}},
			getvertexattribiuiv: {let proc = get_proc_address("glGetVertexAttribIuiv"); if proc == null() {dummy_pfnglgetvertexattribiuivproc} else {unsafe{transmute(proc)}}},
			vertexattribi1i: {let proc = get_proc_address("glVertexAttribI1i"); if proc == null() {dummy_pfnglvertexattribi1iproc} else {unsafe{transmute(proc)}}},
			vertexattribi2i: {let proc = get_proc_address("glVertexAttribI2i"); if proc == null() {dummy_pfnglvertexattribi2iproc} else {unsafe{transmute(proc)}}},
			vertexattribi3i: {let proc = get_proc_address("glVertexAttribI3i"); if proc == null() {dummy_pfnglvertexattribi3iproc} else {unsafe{transmute(proc)}}},
			vertexattribi4i: {let proc = get_proc_address("glVertexAttribI4i"); if proc == null() {dummy_pfnglvertexattribi4iproc} else {unsafe{transmute(proc)}}},
			vertexattribi1ui: {let proc = get_proc_address("glVertexAttribI1ui"); if proc == null() {dummy_pfnglvertexattribi1uiproc} else {unsafe{transmute(proc)}}},
			vertexattribi2ui: {let proc = get_proc_address("glVertexAttribI2ui"); if proc == null() {dummy_pfnglvertexattribi2uiproc} else {unsafe{transmute(proc)}}},
			vertexattribi3ui: {let proc = get_proc_address("glVertexAttribI3ui"); if proc == null() {dummy_pfnglvertexattribi3uiproc} else {unsafe{transmute(proc)}}},
			vertexattribi4ui: {let proc = get_proc_address("glVertexAttribI4ui"); if proc == null() {dummy_pfnglvertexattribi4uiproc} else {unsafe{transmute(proc)}}},
			vertexattribi1iv: {let proc = get_proc_address("glVertexAttribI1iv"); if proc == null() {dummy_pfnglvertexattribi1ivproc} else {unsafe{transmute(proc)}}},
			vertexattribi2iv: {let proc = get_proc_address("glVertexAttribI2iv"); if proc == null() {dummy_pfnglvertexattribi2ivproc} else {unsafe{transmute(proc)}}},
			vertexattribi3iv: {let proc = get_proc_address("glVertexAttribI3iv"); if proc == null() {dummy_pfnglvertexattribi3ivproc} else {unsafe{transmute(proc)}}},
			vertexattribi4iv: {let proc = get_proc_address("glVertexAttribI4iv"); if proc == null() {dummy_pfnglvertexattribi4ivproc} else {unsafe{transmute(proc)}}},
			vertexattribi1uiv: {let proc = get_proc_address("glVertexAttribI1uiv"); if proc == null() {dummy_pfnglvertexattribi1uivproc} else {unsafe{transmute(proc)}}},
			vertexattribi2uiv: {let proc = get_proc_address("glVertexAttribI2uiv"); if proc == null() {dummy_pfnglvertexattribi2uivproc} else {unsafe{transmute(proc)}}},
			vertexattribi3uiv: {let proc = get_proc_address("glVertexAttribI3uiv"); if proc == null() {dummy_pfnglvertexattribi3uivproc} else {unsafe{transmute(proc)}}},
			vertexattribi4uiv: {let proc = get_proc_address("glVertexAttribI4uiv"); if proc == null() {dummy_pfnglvertexattribi4uivproc} else {unsafe{transmute(proc)}}},
			vertexattribi4bv: {let proc = get_proc_address("glVertexAttribI4bv"); if proc == null() {dummy_pfnglvertexattribi4bvproc} else {unsafe{transmute(proc)}}},
			vertexattribi4sv: {let proc = get_proc_address("glVertexAttribI4sv"); if proc == null() {dummy_pfnglvertexattribi4svproc} else {unsafe{transmute(proc)}}},
			vertexattribi4ubv: {let proc = get_proc_address("glVertexAttribI4ubv"); if proc == null() {dummy_pfnglvertexattribi4ubvproc} else {unsafe{transmute(proc)}}},
			vertexattribi4usv: {let proc = get_proc_address("glVertexAttribI4usv"); if proc == null() {dummy_pfnglvertexattribi4usvproc} else {unsafe{transmute(proc)}}},
			getuniformuiv: {let proc = get_proc_address("glGetUniformuiv"); if proc == null() {dummy_pfnglgetuniformuivproc} else {unsafe{transmute(proc)}}},
			bindfragdatalocation: {let proc = get_proc_address("glBindFragDataLocation"); if proc == null() {dummy_pfnglbindfragdatalocationproc} else {unsafe{transmute(proc)}}},
			getfragdatalocation: {let proc = get_proc_address("glGetFragDataLocation"); if proc == null() {dummy_pfnglgetfragdatalocationproc} else {unsafe{transmute(proc)}}},
			uniform1ui: {let proc = get_proc_address("glUniform1ui"); if proc == null() {dummy_pfngluniform1uiproc} else {unsafe{transmute(proc)}}},
			uniform2ui: {let proc = get_proc_address("glUniform2ui"); if proc == null() {dummy_pfngluniform2uiproc} else {unsafe{transmute(proc)}}},
			uniform3ui: {let proc = get_proc_address("glUniform3ui"); if proc == null() {dummy_pfngluniform3uiproc} else {unsafe{transmute(proc)}}},
			uniform4ui: {let proc = get_proc_address("glUniform4ui"); if proc == null() {dummy_pfngluniform4uiproc} else {unsafe{transmute(proc)}}},
			uniform1uiv: {let proc = get_proc_address("glUniform1uiv"); if proc == null() {dummy_pfngluniform1uivproc} else {unsafe{transmute(proc)}}},
			uniform2uiv: {let proc = get_proc_address("glUniform2uiv"); if proc == null() {dummy_pfngluniform2uivproc} else {unsafe{transmute(proc)}}},
			uniform3uiv: {let proc = get_proc_address("glUniform3uiv"); if proc == null() {dummy_pfngluniform3uivproc} else {unsafe{transmute(proc)}}},
			uniform4uiv: {let proc = get_proc_address("glUniform4uiv"); if proc == null() {dummy_pfngluniform4uivproc} else {unsafe{transmute(proc)}}},
			texparameteriiv: {let proc = get_proc_address("glTexParameterIiv"); if proc == null() {dummy_pfngltexparameteriivproc} else {unsafe{transmute(proc)}}},
			texparameteriuiv: {let proc = get_proc_address("glTexParameterIuiv"); if proc == null() {dummy_pfngltexparameteriuivproc} else {unsafe{transmute(proc)}}},
			gettexparameteriiv: {let proc = get_proc_address("glGetTexParameterIiv"); if proc == null() {dummy_pfnglgettexparameteriivproc} else {unsafe{transmute(proc)}}},
			gettexparameteriuiv: {let proc = get_proc_address("glGetTexParameterIuiv"); if proc == null() {dummy_pfnglgettexparameteriuivproc} else {unsafe{transmute(proc)}}},
			clearbufferiv: {let proc = get_proc_address("glClearBufferiv"); if proc == null() {dummy_pfnglclearbufferivproc} else {unsafe{transmute(proc)}}},
			clearbufferuiv: {let proc = get_proc_address("glClearBufferuiv"); if proc == null() {dummy_pfnglclearbufferuivproc} else {unsafe{transmute(proc)}}},
			clearbufferfv: {let proc = get_proc_address("glClearBufferfv"); if proc == null() {dummy_pfnglclearbufferfvproc} else {unsafe{transmute(proc)}}},
			clearbufferfi: {let proc = get_proc_address("glClearBufferfi"); if proc == null() {dummy_pfnglclearbufferfiproc} else {unsafe{transmute(proc)}}},
			getstringi: {let proc = get_proc_address("glGetStringi"); if proc == null() {dummy_pfnglgetstringiproc} else {unsafe{transmute(proc)}}},
			isrenderbuffer: {let proc = get_proc_address("glIsRenderbuffer"); if proc == null() {dummy_pfnglisrenderbufferproc} else {unsafe{transmute(proc)}}},
			bindrenderbuffer: {let proc = get_proc_address("glBindRenderbuffer"); if proc == null() {dummy_pfnglbindrenderbufferproc} else {unsafe{transmute(proc)}}},
			deleterenderbuffers: {let proc = get_proc_address("glDeleteRenderbuffers"); if proc == null() {dummy_pfngldeleterenderbuffersproc} else {unsafe{transmute(proc)}}},
			genrenderbuffers: {let proc = get_proc_address("glGenRenderbuffers"); if proc == null() {dummy_pfnglgenrenderbuffersproc} else {unsafe{transmute(proc)}}},
			renderbufferstorage: {let proc = get_proc_address("glRenderbufferStorage"); if proc == null() {dummy_pfnglrenderbufferstorageproc} else {unsafe{transmute(proc)}}},
			getrenderbufferparameteriv: {let proc = get_proc_address("glGetRenderbufferParameteriv"); if proc == null() {dummy_pfnglgetrenderbufferparameterivproc} else {unsafe{transmute(proc)}}},
			isframebuffer: {let proc = get_proc_address("glIsFramebuffer"); if proc == null() {dummy_pfnglisframebufferproc} else {unsafe{transmute(proc)}}},
			bindframebuffer: {let proc = get_proc_address("glBindFramebuffer"); if proc == null() {dummy_pfnglbindframebufferproc} else {unsafe{transmute(proc)}}},
			deleteframebuffers: {let proc = get_proc_address("glDeleteFramebuffers"); if proc == null() {dummy_pfngldeleteframebuffersproc} else {unsafe{transmute(proc)}}},
			genframebuffers: {let proc = get_proc_address("glGenFramebuffers"); if proc == null() {dummy_pfnglgenframebuffersproc} else {unsafe{transmute(proc)}}},
			checkframebufferstatus: {let proc = get_proc_address("glCheckFramebufferStatus"); if proc == null() {dummy_pfnglcheckframebufferstatusproc} else {unsafe{transmute(proc)}}},
			framebuffertexture1d: {let proc = get_proc_address("glFramebufferTexture1D"); if proc == null() {dummy_pfnglframebuffertexture1dproc} else {unsafe{transmute(proc)}}},
			framebuffertexture2d: {let proc = get_proc_address("glFramebufferTexture2D"); if proc == null() {dummy_pfnglframebuffertexture2dproc} else {unsafe{transmute(proc)}}},
			framebuffertexture3d: {let proc = get_proc_address("glFramebufferTexture3D"); if proc == null() {dummy_pfnglframebuffertexture3dproc} else {unsafe{transmute(proc)}}},
			framebufferrenderbuffer: {let proc = get_proc_address("glFramebufferRenderbuffer"); if proc == null() {dummy_pfnglframebufferrenderbufferproc} else {unsafe{transmute(proc)}}},
			getframebufferattachmentparameteriv: {let proc = get_proc_address("glGetFramebufferAttachmentParameteriv"); if proc == null() {dummy_pfnglgetframebufferattachmentparameterivproc} else {unsafe{transmute(proc)}}},
			generatemipmap: {let proc = get_proc_address("glGenerateMipmap"); if proc == null() {dummy_pfnglgeneratemipmapproc} else {unsafe{transmute(proc)}}},
			blitframebuffer: {let proc = get_proc_address("glBlitFramebuffer"); if proc == null() {dummy_pfnglblitframebufferproc} else {unsafe{transmute(proc)}}},
			renderbufferstoragemultisample: {let proc = get_proc_address("glRenderbufferStorageMultisample"); if proc == null() {dummy_pfnglrenderbufferstoragemultisampleproc} else {unsafe{transmute(proc)}}},
			framebuffertexturelayer: {let proc = get_proc_address("glFramebufferTextureLayer"); if proc == null() {dummy_pfnglframebuffertexturelayerproc} else {unsafe{transmute(proc)}}},
			mapbufferrange: {let proc = get_proc_address("glMapBufferRange"); if proc == null() {dummy_pfnglmapbufferrangeproc} else {unsafe{transmute(proc)}}},
			flushmappedbufferrange: {let proc = get_proc_address("glFlushMappedBufferRange"); if proc == null() {dummy_pfnglflushmappedbufferrangeproc} else {unsafe{transmute(proc)}}},
			bindvertexarray: {let proc = get_proc_address("glBindVertexArray"); if proc == null() {dummy_pfnglbindvertexarrayproc} else {unsafe{transmute(proc)}}},
			deletevertexarrays: {let proc = get_proc_address("glDeleteVertexArrays"); if proc == null() {dummy_pfngldeletevertexarraysproc} else {unsafe{transmute(proc)}}},
			genvertexarrays: {let proc = get_proc_address("glGenVertexArrays"); if proc == null() {dummy_pfnglgenvertexarraysproc} else {unsafe{transmute(proc)}}},
			isvertexarray: {let proc = get_proc_address("glIsVertexArray"); if proc == null() {dummy_pfnglisvertexarrayproc} else {unsafe{transmute(proc)}}},
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
			colormaski: dummy_pfnglcolormaskiproc,
			getbooleani_v: dummy_pfnglgetbooleani_vproc,
			getintegeri_v: dummy_pfnglgetintegeri_vproc,
			enablei: dummy_pfnglenableiproc,
			disablei: dummy_pfngldisableiproc,
			isenabledi: dummy_pfnglisenablediproc,
			begintransformfeedback: dummy_pfnglbegintransformfeedbackproc,
			endtransformfeedback: dummy_pfnglendtransformfeedbackproc,
			bindbufferrange: dummy_pfnglbindbufferrangeproc,
			bindbufferbase: dummy_pfnglbindbufferbaseproc,
			transformfeedbackvaryings: dummy_pfngltransformfeedbackvaryingsproc,
			gettransformfeedbackvarying: dummy_pfnglgettransformfeedbackvaryingproc,
			clampcolor: dummy_pfnglclampcolorproc,
			beginconditionalrender: dummy_pfnglbeginconditionalrenderproc,
			endconditionalrender: dummy_pfnglendconditionalrenderproc,
			vertexattribipointer: dummy_pfnglvertexattribipointerproc,
			getvertexattribiiv: dummy_pfnglgetvertexattribiivproc,
			getvertexattribiuiv: dummy_pfnglgetvertexattribiuivproc,
			vertexattribi1i: dummy_pfnglvertexattribi1iproc,
			vertexattribi2i: dummy_pfnglvertexattribi2iproc,
			vertexattribi3i: dummy_pfnglvertexattribi3iproc,
			vertexattribi4i: dummy_pfnglvertexattribi4iproc,
			vertexattribi1ui: dummy_pfnglvertexattribi1uiproc,
			vertexattribi2ui: dummy_pfnglvertexattribi2uiproc,
			vertexattribi3ui: dummy_pfnglvertexattribi3uiproc,
			vertexattribi4ui: dummy_pfnglvertexattribi4uiproc,
			vertexattribi1iv: dummy_pfnglvertexattribi1ivproc,
			vertexattribi2iv: dummy_pfnglvertexattribi2ivproc,
			vertexattribi3iv: dummy_pfnglvertexattribi3ivproc,
			vertexattribi4iv: dummy_pfnglvertexattribi4ivproc,
			vertexattribi1uiv: dummy_pfnglvertexattribi1uivproc,
			vertexattribi2uiv: dummy_pfnglvertexattribi2uivproc,
			vertexattribi3uiv: dummy_pfnglvertexattribi3uivproc,
			vertexattribi4uiv: dummy_pfnglvertexattribi4uivproc,
			vertexattribi4bv: dummy_pfnglvertexattribi4bvproc,
			vertexattribi4sv: dummy_pfnglvertexattribi4svproc,
			vertexattribi4ubv: dummy_pfnglvertexattribi4ubvproc,
			vertexattribi4usv: dummy_pfnglvertexattribi4usvproc,
			getuniformuiv: dummy_pfnglgetuniformuivproc,
			bindfragdatalocation: dummy_pfnglbindfragdatalocationproc,
			getfragdatalocation: dummy_pfnglgetfragdatalocationproc,
			uniform1ui: dummy_pfngluniform1uiproc,
			uniform2ui: dummy_pfngluniform2uiproc,
			uniform3ui: dummy_pfngluniform3uiproc,
			uniform4ui: dummy_pfngluniform4uiproc,
			uniform1uiv: dummy_pfngluniform1uivproc,
			uniform2uiv: dummy_pfngluniform2uivproc,
			uniform3uiv: dummy_pfngluniform3uivproc,
			uniform4uiv: dummy_pfngluniform4uivproc,
			texparameteriiv: dummy_pfngltexparameteriivproc,
			texparameteriuiv: dummy_pfngltexparameteriuivproc,
			gettexparameteriiv: dummy_pfnglgettexparameteriivproc,
			gettexparameteriuiv: dummy_pfnglgettexparameteriuivproc,
			clearbufferiv: dummy_pfnglclearbufferivproc,
			clearbufferuiv: dummy_pfnglclearbufferuivproc,
			clearbufferfv: dummy_pfnglclearbufferfvproc,
			clearbufferfi: dummy_pfnglclearbufferfiproc,
			getstringi: dummy_pfnglgetstringiproc,
			isrenderbuffer: dummy_pfnglisrenderbufferproc,
			bindrenderbuffer: dummy_pfnglbindrenderbufferproc,
			deleterenderbuffers: dummy_pfngldeleterenderbuffersproc,
			genrenderbuffers: dummy_pfnglgenrenderbuffersproc,
			renderbufferstorage: dummy_pfnglrenderbufferstorageproc,
			getrenderbufferparameteriv: dummy_pfnglgetrenderbufferparameterivproc,
			isframebuffer: dummy_pfnglisframebufferproc,
			bindframebuffer: dummy_pfnglbindframebufferproc,
			deleteframebuffers: dummy_pfngldeleteframebuffersproc,
			genframebuffers: dummy_pfnglgenframebuffersproc,
			checkframebufferstatus: dummy_pfnglcheckframebufferstatusproc,
			framebuffertexture1d: dummy_pfnglframebuffertexture1dproc,
			framebuffertexture2d: dummy_pfnglframebuffertexture2dproc,
			framebuffertexture3d: dummy_pfnglframebuffertexture3dproc,
			framebufferrenderbuffer: dummy_pfnglframebufferrenderbufferproc,
			getframebufferattachmentparameteriv: dummy_pfnglgetframebufferattachmentparameterivproc,
			generatemipmap: dummy_pfnglgeneratemipmapproc,
			blitframebuffer: dummy_pfnglblitframebufferproc,
			renderbufferstoragemultisample: dummy_pfnglrenderbufferstoragemultisampleproc,
			framebuffertexturelayer: dummy_pfnglframebuffertexturelayerproc,
			mapbufferrange: dummy_pfnglmapbufferrangeproc,
			flushmappedbufferrange: dummy_pfnglflushmappedbufferrangeproc,
			bindvertexarray: dummy_pfnglbindvertexarrayproc,
			deletevertexarrays: dummy_pfngldeletevertexarraysproc,
			genvertexarrays: dummy_pfnglgenvertexarraysproc,
			isvertexarray: dummy_pfnglisvertexarrayproc,
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
extern "system" fn dummy_pfngldrawarraysinstancedproc (_: GLenum, _: GLint, _: GLsizei, _: GLsizei) {
	panic!("OpenGL Function pointer of `glDrawArraysInstanced()` is NULL");
}
extern "system" fn dummy_pfngldrawelementsinstancedproc (_: GLenum, _: GLsizei, _: GLenum, _: *const c_void, _: GLsizei) {
	panic!("OpenGL Function pointer of `glDrawElementsInstanced()` is NULL");
}
extern "system" fn dummy_pfngltexbufferproc (_: GLenum, _: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glTexBuffer()` is NULL");
}
extern "system" fn dummy_pfnglprimitiverestartindexproc (_: GLuint) {
	panic!("OpenGL Function pointer of `glPrimitiveRestartIndex()` is NULL");
}
extern "system" fn dummy_pfnglcopybuffersubdataproc (_: GLenum, _: GLenum, _: GLintptr, _: GLintptr, _: GLsizeiptr) {
	panic!("OpenGL Function pointer of `glCopyBufferSubData()` is NULL");
}
extern "system" fn dummy_pfnglgetuniformindicesproc (_: GLuint, _: GLsizei, _: *const *const GLchar, _: *mut GLuint) {
	panic!("OpenGL Function pointer of `glGetUniformIndices()` is NULL");
}
extern "system" fn dummy_pfnglgetactiveuniformsivproc (_: GLuint, _: GLsizei, _: *const GLuint, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetActiveUniformsiv()` is NULL");
}
extern "system" fn dummy_pfnglgetactiveuniformnameproc (_: GLuint, _: GLuint, _: GLsizei, _: *mut GLsizei, _: *mut GLchar) {
	panic!("OpenGL Function pointer of `glGetActiveUniformName()` is NULL");
}
extern "system" fn dummy_pfnglgetuniformblockindexproc (_: GLuint, _: *const GLchar) -> GLuint {
	panic!("OpenGL Function pointer of `glGetUniformBlockIndex()` is NULL");
}
extern "system" fn dummy_pfnglgetactiveuniformblockivproc (_: GLuint, _: GLuint, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetActiveUniformBlockiv()` is NULL");
}
extern "system" fn dummy_pfnglgetactiveuniformblocknameproc (_: GLuint, _: GLuint, _: GLsizei, _: *mut GLsizei, _: *mut GLchar) {
	panic!("OpenGL Function pointer of `glGetActiveUniformBlockName()` is NULL");
}
extern "system" fn dummy_pfngluniformblockbindingproc (_: GLuint, _: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glUniformBlockBinding()` is NULL");
}
pub const GL_SAMPLER_2D_RECT: GLenum = 0x8B63;
pub const GL_SAMPLER_2D_RECT_SHADOW: GLenum = 0x8B64;
pub const GL_SAMPLER_BUFFER: GLenum = 0x8DC2;
pub const GL_INT_SAMPLER_2D_RECT: GLenum = 0x8DCD;
pub const GL_INT_SAMPLER_BUFFER: GLenum = 0x8DD0;
pub const GL_UNSIGNED_INT_SAMPLER_2D_RECT: GLenum = 0x8DD5;
pub const GL_UNSIGNED_INT_SAMPLER_BUFFER: GLenum = 0x8DD8;
pub const GL_TEXTURE_BUFFER: GLenum = 0x8C2A;
pub const GL_MAX_TEXTURE_BUFFER_SIZE: GLenum = 0x8C2B;
pub const GL_TEXTURE_BINDING_BUFFER: GLenum = 0x8C2C;
pub const GL_TEXTURE_BUFFER_DATA_STORE_BINDING: GLenum = 0x8C2D;
pub const GL_TEXTURE_RECTANGLE: GLenum = 0x84F5;
pub const GL_TEXTURE_BINDING_RECTANGLE: GLenum = 0x84F6;
pub const GL_PROXY_TEXTURE_RECTANGLE: GLenum = 0x84F7;
pub const GL_MAX_RECTANGLE_TEXTURE_SIZE: GLenum = 0x84F8;
pub const GL_R8_SNORM: GLenum = 0x8F94;
pub const GL_RG8_SNORM: GLenum = 0x8F95;
pub const GL_RGB8_SNORM: GLenum = 0x8F96;
pub const GL_RGBA8_SNORM: GLenum = 0x8F97;
pub const GL_R16_SNORM: GLenum = 0x8F98;
pub const GL_RG16_SNORM: GLenum = 0x8F99;
pub const GL_RGB16_SNORM: GLenum = 0x8F9A;
pub const GL_RGBA16_SNORM: GLenum = 0x8F9B;
pub const GL_SIGNED_NORMALIZED: GLenum = 0x8F9C;
pub const GL_PRIMITIVE_RESTART: GLenum = 0x8F9D;
pub const GL_PRIMITIVE_RESTART_INDEX: GLenum = 0x8F9E;
pub const GL_COPY_READ_BUFFER: GLenum = 0x8F36;
pub const GL_COPY_WRITE_BUFFER: GLenum = 0x8F37;
pub const GL_UNIFORM_BUFFER: GLenum = 0x8A11;
pub const GL_UNIFORM_BUFFER_BINDING: GLenum = 0x8A28;
pub const GL_UNIFORM_BUFFER_START: GLenum = 0x8A29;
pub const GL_UNIFORM_BUFFER_SIZE: GLenum = 0x8A2A;
pub const GL_MAX_VERTEX_UNIFORM_BLOCKS: GLenum = 0x8A2B;
pub const GL_MAX_GEOMETRY_UNIFORM_BLOCKS: GLenum = 0x8A2C;
pub const GL_MAX_FRAGMENT_UNIFORM_BLOCKS: GLenum = 0x8A2D;
pub const GL_MAX_COMBINED_UNIFORM_BLOCKS: GLenum = 0x8A2E;
pub const GL_MAX_UNIFORM_BUFFER_BINDINGS: GLenum = 0x8A2F;
pub const GL_MAX_UNIFORM_BLOCK_SIZE: GLenum = 0x8A30;
pub const GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8A31;
pub const GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 0x8A32;
pub const GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8A33;
pub const GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x8A34;
pub const GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: GLenum = 0x8A35;
pub const GL_ACTIVE_UNIFORM_BLOCKS: GLenum = 0x8A36;
pub const GL_UNIFORM_TYPE: GLenum = 0x8A37;
pub const GL_UNIFORM_SIZE: GLenum = 0x8A38;
pub const GL_UNIFORM_NAME_LENGTH: GLenum = 0x8A39;
pub const GL_UNIFORM_BLOCK_INDEX: GLenum = 0x8A3A;
pub const GL_UNIFORM_OFFSET: GLenum = 0x8A3B;
pub const GL_UNIFORM_ARRAY_STRIDE: GLenum = 0x8A3C;
pub const GL_UNIFORM_MATRIX_STRIDE: GLenum = 0x8A3D;
pub const GL_UNIFORM_IS_ROW_MAJOR: GLenum = 0x8A3E;
pub const GL_UNIFORM_BLOCK_BINDING: GLenum = 0x8A3F;
pub const GL_UNIFORM_BLOCK_DATA_SIZE: GLenum = 0x8A40;
pub const GL_UNIFORM_BLOCK_NAME_LENGTH: GLenum = 0x8A41;
pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS: GLenum = 0x8A42;
pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: GLenum = 0x8A43;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x8A44;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x8A45;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x8A46;
pub const GL_INVALID_INDEX: GLuint = 0xFFFFFFFFu32;

pub trait GL_3_1 {
	fn glDrawArraysInstanced(&self, mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei);
	fn glDrawElementsInstanced(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei);
	fn glTexBuffer(&self, target: GLenum, internalformat: GLenum, buffer: GLuint);
	fn glPrimitiveRestartIndex(&self, index: GLuint);
	fn glCopyBufferSubData(&self, readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);
	fn glGetUniformIndices(&self, program: GLuint, uniformCount: GLsizei, uniformNames: *const *const GLchar, uniformIndices: *mut GLuint);
	fn glGetActiveUniformsiv(&self, program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint, pname: GLenum, params: *mut GLint);
	fn glGetActiveUniformName(&self, program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar);
	fn glGetUniformBlockIndex(&self, program: GLuint, uniformBlockName: *const GLchar) -> GLuint;
	fn glGetActiveUniformBlockiv(&self, program: GLuint, uniformBlockIndex: GLuint, pname: GLenum, params: *mut GLint);
	fn glGetActiveUniformBlockName(&self, program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar);
	fn glUniformBlockBinding(&self, program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
	pub fn new(base: impl GL_1_0, mut get_proc_address: impl FnMut(&'static str) -> *const c_void) -> Self {
		let (_spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 30100 {
			return Self::default();
		}
		Self {
			available: true,
			drawarraysinstanced: {let proc = get_proc_address("glDrawArraysInstanced"); if proc == null() {dummy_pfngldrawarraysinstancedproc} else {unsafe{transmute(proc)}}},
			drawelementsinstanced: {let proc = get_proc_address("glDrawElementsInstanced"); if proc == null() {dummy_pfngldrawelementsinstancedproc} else {unsafe{transmute(proc)}}},
			texbuffer: {let proc = get_proc_address("glTexBuffer"); if proc == null() {dummy_pfngltexbufferproc} else {unsafe{transmute(proc)}}},
			primitiverestartindex: {let proc = get_proc_address("glPrimitiveRestartIndex"); if proc == null() {dummy_pfnglprimitiverestartindexproc} else {unsafe{transmute(proc)}}},
			copybuffersubdata: {let proc = get_proc_address("glCopyBufferSubData"); if proc == null() {dummy_pfnglcopybuffersubdataproc} else {unsafe{transmute(proc)}}},
			getuniformindices: {let proc = get_proc_address("glGetUniformIndices"); if proc == null() {dummy_pfnglgetuniformindicesproc} else {unsafe{transmute(proc)}}},
			getactiveuniformsiv: {let proc = get_proc_address("glGetActiveUniformsiv"); if proc == null() {dummy_pfnglgetactiveuniformsivproc} else {unsafe{transmute(proc)}}},
			getactiveuniformname: {let proc = get_proc_address("glGetActiveUniformName"); if proc == null() {dummy_pfnglgetactiveuniformnameproc} else {unsafe{transmute(proc)}}},
			getuniformblockindex: {let proc = get_proc_address("glGetUniformBlockIndex"); if proc == null() {dummy_pfnglgetuniformblockindexproc} else {unsafe{transmute(proc)}}},
			getactiveuniformblockiv: {let proc = get_proc_address("glGetActiveUniformBlockiv"); if proc == null() {dummy_pfnglgetactiveuniformblockivproc} else {unsafe{transmute(proc)}}},
			getactiveuniformblockname: {let proc = get_proc_address("glGetActiveUniformBlockName"); if proc == null() {dummy_pfnglgetactiveuniformblocknameproc} else {unsafe{transmute(proc)}}},
			uniformblockbinding: {let proc = get_proc_address("glUniformBlockBinding"); if proc == null() {dummy_pfngluniformblockbindingproc} else {unsafe{transmute(proc)}}},
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
			drawarraysinstanced: dummy_pfngldrawarraysinstancedproc,
			drawelementsinstanced: dummy_pfngldrawelementsinstancedproc,
			texbuffer: dummy_pfngltexbufferproc,
			primitiverestartindex: dummy_pfnglprimitiverestartindexproc,
			copybuffersubdata: dummy_pfnglcopybuffersubdataproc,
			getuniformindices: dummy_pfnglgetuniformindicesproc,
			getactiveuniformsiv: dummy_pfnglgetactiveuniformsivproc,
			getactiveuniformname: dummy_pfnglgetactiveuniformnameproc,
			getuniformblockindex: dummy_pfnglgetuniformblockindexproc,
			getactiveuniformblockiv: dummy_pfnglgetactiveuniformblockivproc,
			getactiveuniformblockname: dummy_pfnglgetactiveuniformblocknameproc,
			uniformblockbinding: dummy_pfngluniformblockbindingproc,
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
extern "system" fn dummy_pfngldrawelementsbasevertexproc (_: GLenum, _: GLsizei, _: GLenum, _: *const c_void, _: GLint) {
	panic!("OpenGL Function pointer of `glDrawElementsBaseVertex()` is NULL");
}
extern "system" fn dummy_pfngldrawrangeelementsbasevertexproc (_: GLenum, _: GLuint, _: GLuint, _: GLsizei, _: GLenum, _: *const c_void, _: GLint) {
	panic!("OpenGL Function pointer of `glDrawRangeElementsBaseVertex()` is NULL");
}
extern "system" fn dummy_pfngldrawelementsinstancedbasevertexproc (_: GLenum, _: GLsizei, _: GLenum, _: *const c_void, _: GLsizei, _: GLint) {
	panic!("OpenGL Function pointer of `glDrawElementsInstancedBaseVertex()` is NULL");
}
extern "system" fn dummy_pfnglmultidrawelementsbasevertexproc (_: GLenum, _: *const GLsizei, _: GLenum, _: *const *const c_void, _: GLsizei, _: *const GLint) {
	panic!("OpenGL Function pointer of `glMultiDrawElementsBaseVertex()` is NULL");
}
extern "system" fn dummy_pfnglprovokingvertexproc (_: GLenum) {
	panic!("OpenGL Function pointer of `glProvokingVertex()` is NULL");
}
extern "system" fn dummy_pfnglfencesyncproc (_: GLenum, _: GLbitfield) -> GLsync {
	panic!("OpenGL Function pointer of `glFenceSync()` is NULL");
}
extern "system" fn dummy_pfnglissyncproc (_: GLsync) -> GLboolean {
	panic!("OpenGL Function pointer of `glIsSync()` is NULL");
}
extern "system" fn dummy_pfngldeletesyncproc (_: GLsync) {
	panic!("OpenGL Function pointer of `glDeleteSync()` is NULL");
}
extern "system" fn dummy_pfnglclientwaitsyncproc (_: GLsync, _: GLbitfield, _: GLuint64) -> GLenum {
	panic!("OpenGL Function pointer of `glClientWaitSync()` is NULL");
}
extern "system" fn dummy_pfnglwaitsyncproc (_: GLsync, _: GLbitfield, _: GLuint64) {
	panic!("OpenGL Function pointer of `glWaitSync()` is NULL");
}
extern "system" fn dummy_pfnglgetinteger64vproc (_: GLenum, _: *mut GLint64) {
	panic!("OpenGL Function pointer of `glGetInteger64v()` is NULL");
}
extern "system" fn dummy_pfnglgetsyncivproc (_: GLsync, _: GLenum, _: GLsizei, _: *mut GLsizei, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetSynciv()` is NULL");
}
extern "system" fn dummy_pfnglgetinteger64i_vproc (_: GLenum, _: GLuint, _: *mut GLint64) {
	panic!("OpenGL Function pointer of `glGetInteger64i_v()` is NULL");
}
extern "system" fn dummy_pfnglgetbufferparameteri64vproc (_: GLenum, _: GLenum, _: *mut GLint64) {
	panic!("OpenGL Function pointer of `glGetBufferParameteri64v()` is NULL");
}
extern "system" fn dummy_pfnglframebuffertextureproc (_: GLenum, _: GLenum, _: GLuint, _: GLint) {
	panic!("OpenGL Function pointer of `glFramebufferTexture()` is NULL");
}
extern "system" fn dummy_pfnglteximage2dmultisampleproc (_: GLenum, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei, _: GLboolean) {
	panic!("OpenGL Function pointer of `glTexImage2DMultisample()` is NULL");
}
extern "system" fn dummy_pfnglteximage3dmultisampleproc (_: GLenum, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei, _: GLsizei, _: GLboolean) {
	panic!("OpenGL Function pointer of `glTexImage3DMultisample()` is NULL");
}
extern "system" fn dummy_pfnglgetmultisamplefvproc (_: GLenum, _: GLuint, _: *mut GLfloat) {
	panic!("OpenGL Function pointer of `glGetMultisamplefv()` is NULL");
}
extern "system" fn dummy_pfnglsamplemaskiproc (_: GLuint, _: GLbitfield) {
	panic!("OpenGL Function pointer of `glSampleMaski()` is NULL");
}
pub const GL_CONTEXT_CORE_PROFILE_BIT: GLbitfield = 0x00000001;
pub const GL_CONTEXT_COMPATIBILITY_PROFILE_BIT: GLbitfield = 0x00000002;
pub const GL_LINES_ADJACENCY: GLenum = 0x000A;
pub const GL_LINE_STRIP_ADJACENCY: GLenum = 0x000B;
pub const GL_TRIANGLES_ADJACENCY: GLenum = 0x000C;
pub const GL_TRIANGLE_STRIP_ADJACENCY: GLenum = 0x000D;
pub const GL_PROGRAM_POINT_SIZE: GLenum = 0x8642;
pub const GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: GLenum = 0x8C29;
pub const GL_FRAMEBUFFER_ATTACHMENT_LAYERED: GLenum = 0x8DA7;
pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: GLenum = 0x8DA8;
pub const GL_GEOMETRY_SHADER: GLenum = 0x8DD9;
pub const GL_GEOMETRY_VERTICES_OUT: GLenum = 0x8916;
pub const GL_GEOMETRY_INPUT_TYPE: GLenum = 0x8917;
pub const GL_GEOMETRY_OUTPUT_TYPE: GLenum = 0x8918;
pub const GL_MAX_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 0x8DDF;
pub const GL_MAX_GEOMETRY_OUTPUT_VERTICES: GLenum = 0x8DE0;
pub const GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: GLenum = 0x8DE1;
pub const GL_MAX_VERTEX_OUTPUT_COMPONENTS: GLenum = 0x9122;
pub const GL_MAX_GEOMETRY_INPUT_COMPONENTS: GLenum = 0x9123;
pub const GL_MAX_GEOMETRY_OUTPUT_COMPONENTS: GLenum = 0x9124;
pub const GL_MAX_FRAGMENT_INPUT_COMPONENTS: GLenum = 0x9125;
pub const GL_CONTEXT_PROFILE_MASK: GLenum = 0x9126;
pub const GL_DEPTH_CLAMP: GLenum = 0x864F;
pub const GL_QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: GLenum = 0x8E4C;
pub const GL_FIRST_VERTEX_CONVENTION: GLenum = 0x8E4D;
pub const GL_LAST_VERTEX_CONVENTION: GLenum = 0x8E4E;
pub const GL_PROVOKING_VERTEX: GLenum = 0x8E4F;
pub const GL_TEXTURE_CUBE_MAP_SEAMLESS: GLenum = 0x884F;
pub const GL_MAX_SERVER_WAIT_TIMEOUT: GLenum = 0x9111;
pub const GL_OBJECT_TYPE: GLenum = 0x9112;
pub const GL_SYNC_CONDITION: GLenum = 0x9113;
pub const GL_SYNC_STATUS: GLenum = 0x9114;
pub const GL_SYNC_FLAGS: GLenum = 0x9115;
pub const GL_SYNC_FENCE: GLenum = 0x9116;
pub const GL_SYNC_GPU_COMMANDS_COMPLETE: GLenum = 0x9117;
pub const GL_UNSIGNALED: GLenum = 0x9118;
pub const GL_SIGNALED: GLenum = 0x9119;
pub const GL_ALREADY_SIGNALED: GLenum = 0x911A;
pub const GL_TIMEOUT_EXPIRED: GLenum = 0x911B;
pub const GL_CONDITION_SATISFIED: GLenum = 0x911C;
pub const GL_WAIT_FAILED: GLenum = 0x911D;
pub const GL_TIMEOUT_IGNORED: GLuint64 = 0xFFFFFFFFFFFFFFFFu64;
pub const GL_SYNC_FLUSH_COMMANDS_BIT: GLbitfield = 0x00000001;
pub const GL_SAMPLE_POSITION: GLenum = 0x8E50;
pub const GL_SAMPLE_MASK: GLenum = 0x8E51;
pub const GL_SAMPLE_MASK_VALUE: GLenum = 0x8E52;
pub const GL_MAX_SAMPLE_MASK_WORDS: GLenum = 0x8E59;
pub const GL_TEXTURE_2D_MULTISAMPLE: GLenum = 0x9100;
pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE: GLenum = 0x9101;
pub const GL_TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9102;
pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9103;
pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE: GLenum = 0x9104;
pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: GLenum = 0x9105;
pub const GL_TEXTURE_SAMPLES: GLenum = 0x9106;
pub const GL_TEXTURE_FIXED_SAMPLE_LOCATIONS: GLenum = 0x9107;
pub const GL_SAMPLER_2D_MULTISAMPLE: GLenum = 0x9108;
pub const GL_INT_SAMPLER_2D_MULTISAMPLE: GLenum = 0x9109;
pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: GLenum = 0x910A;
pub const GL_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910B;
pub const GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910C;
pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910D;
pub const GL_MAX_COLOR_TEXTURE_SAMPLES: GLenum = 0x910E;
pub const GL_MAX_DEPTH_TEXTURE_SAMPLES: GLenum = 0x910F;
pub const GL_MAX_INTEGER_SAMPLES: GLenum = 0x9110;

pub trait GL_3_2 {
	fn glDrawElementsBaseVertex(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, basevertex: GLint);
	fn glDrawRangeElementsBaseVertex(&self, mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const c_void, basevertex: GLint);
	fn glDrawElementsInstancedBaseVertex(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei, basevertex: GLint);
	fn glMultiDrawElementsBaseVertex(&self, mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const c_void, drawcount: GLsizei, basevertex: *const GLint);
	fn glProvokingVertex(&self, mode: GLenum);
	fn glFenceSync(&self, condition: GLenum, flags: GLbitfield) -> GLsync;
	fn glIsSync(&self, sync: GLsync) -> GLboolean;
	fn glDeleteSync(&self, sync: GLsync);
	fn glClientWaitSync(&self, sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum;
	fn glWaitSync(&self, sync: GLsync, flags: GLbitfield, timeout: GLuint64);
	fn glGetInteger64v(&self, pname: GLenum, data: *mut GLint64);
	fn glGetSynciv(&self, sync: GLsync, pname: GLenum, count: GLsizei, length: *mut GLsizei, values: *mut GLint);
	fn glGetInteger64i_v(&self, target: GLenum, index: GLuint, data: *mut GLint64);
	fn glGetBufferParameteri64v(&self, target: GLenum, pname: GLenum, params: *mut GLint64);
	fn glFramebufferTexture(&self, target: GLenum, attachment: GLenum, texture: GLuint, level: GLint);
	fn glTexImage2DMultisample(&self, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);
	fn glTexImage3DMultisample(&self, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);
	fn glGetMultisamplefv(&self, pname: GLenum, index: GLuint, val: *mut GLfloat);
	fn glSampleMaski(&self, maskNumber: GLuint, mask: GLbitfield);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
	pub fn new(base: impl GL_1_0, mut get_proc_address: impl FnMut(&'static str) -> *const c_void) -> Self {
		let (_spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 30200 {
			return Self::default();
		}
		Self {
			available: true,
			drawelementsbasevertex: {let proc = get_proc_address("glDrawElementsBaseVertex"); if proc == null() {dummy_pfngldrawelementsbasevertexproc} else {unsafe{transmute(proc)}}},
			drawrangeelementsbasevertex: {let proc = get_proc_address("glDrawRangeElementsBaseVertex"); if proc == null() {dummy_pfngldrawrangeelementsbasevertexproc} else {unsafe{transmute(proc)}}},
			drawelementsinstancedbasevertex: {let proc = get_proc_address("glDrawElementsInstancedBaseVertex"); if proc == null() {dummy_pfngldrawelementsinstancedbasevertexproc} else {unsafe{transmute(proc)}}},
			multidrawelementsbasevertex: {let proc = get_proc_address("glMultiDrawElementsBaseVertex"); if proc == null() {dummy_pfnglmultidrawelementsbasevertexproc} else {unsafe{transmute(proc)}}},
			provokingvertex: {let proc = get_proc_address("glProvokingVertex"); if proc == null() {dummy_pfnglprovokingvertexproc} else {unsafe{transmute(proc)}}},
			fencesync: {let proc = get_proc_address("glFenceSync"); if proc == null() {dummy_pfnglfencesyncproc} else {unsafe{transmute(proc)}}},
			issync: {let proc = get_proc_address("glIsSync"); if proc == null() {dummy_pfnglissyncproc} else {unsafe{transmute(proc)}}},
			deletesync: {let proc = get_proc_address("glDeleteSync"); if proc == null() {dummy_pfngldeletesyncproc} else {unsafe{transmute(proc)}}},
			clientwaitsync: {let proc = get_proc_address("glClientWaitSync"); if proc == null() {dummy_pfnglclientwaitsyncproc} else {unsafe{transmute(proc)}}},
			waitsync: {let proc = get_proc_address("glWaitSync"); if proc == null() {dummy_pfnglwaitsyncproc} else {unsafe{transmute(proc)}}},
			getinteger64v: {let proc = get_proc_address("glGetInteger64v"); if proc == null() {dummy_pfnglgetinteger64vproc} else {unsafe{transmute(proc)}}},
			getsynciv: {let proc = get_proc_address("glGetSynciv"); if proc == null() {dummy_pfnglgetsyncivproc} else {unsafe{transmute(proc)}}},
			getinteger64i_v: {let proc = get_proc_address("glGetInteger64i_v"); if proc == null() {dummy_pfnglgetinteger64i_vproc} else {unsafe{transmute(proc)}}},
			getbufferparameteri64v: {let proc = get_proc_address("glGetBufferParameteri64v"); if proc == null() {dummy_pfnglgetbufferparameteri64vproc} else {unsafe{transmute(proc)}}},
			framebuffertexture: {let proc = get_proc_address("glFramebufferTexture"); if proc == null() {dummy_pfnglframebuffertextureproc} else {unsafe{transmute(proc)}}},
			teximage2dmultisample: {let proc = get_proc_address("glTexImage2DMultisample"); if proc == null() {dummy_pfnglteximage2dmultisampleproc} else {unsafe{transmute(proc)}}},
			teximage3dmultisample: {let proc = get_proc_address("glTexImage3DMultisample"); if proc == null() {dummy_pfnglteximage3dmultisampleproc} else {unsafe{transmute(proc)}}},
			getmultisamplefv: {let proc = get_proc_address("glGetMultisamplefv"); if proc == null() {dummy_pfnglgetmultisamplefvproc} else {unsafe{transmute(proc)}}},
			samplemaski: {let proc = get_proc_address("glSampleMaski"); if proc == null() {dummy_pfnglsamplemaskiproc} else {unsafe{transmute(proc)}}},
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
			drawelementsbasevertex: dummy_pfngldrawelementsbasevertexproc,
			drawrangeelementsbasevertex: dummy_pfngldrawrangeelementsbasevertexproc,
			drawelementsinstancedbasevertex: dummy_pfngldrawelementsinstancedbasevertexproc,
			multidrawelementsbasevertex: dummy_pfnglmultidrawelementsbasevertexproc,
			provokingvertex: dummy_pfnglprovokingvertexproc,
			fencesync: dummy_pfnglfencesyncproc,
			issync: dummy_pfnglissyncproc,
			deletesync: dummy_pfngldeletesyncproc,
			clientwaitsync: dummy_pfnglclientwaitsyncproc,
			waitsync: dummy_pfnglwaitsyncproc,
			getinteger64v: dummy_pfnglgetinteger64vproc,
			getsynciv: dummy_pfnglgetsyncivproc,
			getinteger64i_v: dummy_pfnglgetinteger64i_vproc,
			getbufferparameteri64v: dummy_pfnglgetbufferparameteri64vproc,
			framebuffertexture: dummy_pfnglframebuffertextureproc,
			teximage2dmultisample: dummy_pfnglteximage2dmultisampleproc,
			teximage3dmultisample: dummy_pfnglteximage3dmultisampleproc,
			getmultisamplefv: dummy_pfnglgetmultisamplefvproc,
			samplemaski: dummy_pfnglsamplemaskiproc,
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
extern "system" fn dummy_pfnglbindfragdatalocationindexedproc (_: GLuint, _: GLuint, _: GLuint, _: *const GLchar) {
	panic!("OpenGL Function pointer of `glBindFragDataLocationIndexed()` is NULL");
}
extern "system" fn dummy_pfnglgetfragdataindexproc (_: GLuint, _: *const GLchar) -> GLint {
	panic!("OpenGL Function pointer of `glGetFragDataIndex()` is NULL");
}
extern "system" fn dummy_pfnglgensamplersproc (_: GLsizei, _: *mut GLuint) {
	panic!("OpenGL Function pointer of `glGenSamplers()` is NULL");
}
extern "system" fn dummy_pfngldeletesamplersproc (_: GLsizei, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glDeleteSamplers()` is NULL");
}
extern "system" fn dummy_pfnglissamplerproc (_: GLuint) -> GLboolean {
	panic!("OpenGL Function pointer of `glIsSampler()` is NULL");
}
extern "system" fn dummy_pfnglbindsamplerproc (_: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glBindSampler()` is NULL");
}
extern "system" fn dummy_pfnglsamplerparameteriproc (_: GLuint, _: GLenum, _: GLint) {
	panic!("OpenGL Function pointer of `glSamplerParameteri()` is NULL");
}
extern "system" fn dummy_pfnglsamplerparameterivproc (_: GLuint, _: GLenum, _: *const GLint) {
	panic!("OpenGL Function pointer of `glSamplerParameteriv()` is NULL");
}
extern "system" fn dummy_pfnglsamplerparameterfproc (_: GLuint, _: GLenum, _: GLfloat) {
	panic!("OpenGL Function pointer of `glSamplerParameterf()` is NULL");
}
extern "system" fn dummy_pfnglsamplerparameterfvproc (_: GLuint, _: GLenum, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glSamplerParameterfv()` is NULL");
}
extern "system" fn dummy_pfnglsamplerparameteriivproc (_: GLuint, _: GLenum, _: *const GLint) {
	panic!("OpenGL Function pointer of `glSamplerParameterIiv()` is NULL");
}
extern "system" fn dummy_pfnglsamplerparameteriuivproc (_: GLuint, _: GLenum, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glSamplerParameterIuiv()` is NULL");
}
extern "system" fn dummy_pfnglgetsamplerparameterivproc (_: GLuint, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetSamplerParameteriv()` is NULL");
}
extern "system" fn dummy_pfnglgetsamplerparameteriivproc (_: GLuint, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetSamplerParameterIiv()` is NULL");
}
extern "system" fn dummy_pfnglgetsamplerparameterfvproc (_: GLuint, _: GLenum, _: *mut GLfloat) {
	panic!("OpenGL Function pointer of `glGetSamplerParameterfv()` is NULL");
}
extern "system" fn dummy_pfnglgetsamplerparameteriuivproc (_: GLuint, _: GLenum, _: *mut GLuint) {
	panic!("OpenGL Function pointer of `glGetSamplerParameterIuiv()` is NULL");
}
extern "system" fn dummy_pfnglquerycounterproc (_: GLuint, _: GLenum) {
	panic!("OpenGL Function pointer of `glQueryCounter()` is NULL");
}
extern "system" fn dummy_pfnglgetqueryobjecti64vproc (_: GLuint, _: GLenum, _: *mut GLint64) {
	panic!("OpenGL Function pointer of `glGetQueryObjecti64v()` is NULL");
}
extern "system" fn dummy_pfnglgetqueryobjectui64vproc (_: GLuint, _: GLenum, _: *mut GLuint64) {
	panic!("OpenGL Function pointer of `glGetQueryObjectui64v()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribdivisorproc (_: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glVertexAttribDivisor()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribp1uiproc (_: GLuint, _: GLenum, _: GLboolean, _: GLuint) {
	panic!("OpenGL Function pointer of `glVertexAttribP1ui()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribp1uivproc (_: GLuint, _: GLenum, _: GLboolean, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glVertexAttribP1uiv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribp2uiproc (_: GLuint, _: GLenum, _: GLboolean, _: GLuint) {
	panic!("OpenGL Function pointer of `glVertexAttribP2ui()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribp2uivproc (_: GLuint, _: GLenum, _: GLboolean, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glVertexAttribP2uiv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribp3uiproc (_: GLuint, _: GLenum, _: GLboolean, _: GLuint) {
	panic!("OpenGL Function pointer of `glVertexAttribP3ui()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribp3uivproc (_: GLuint, _: GLenum, _: GLboolean, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glVertexAttribP3uiv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribp4uiproc (_: GLuint, _: GLenum, _: GLboolean, _: GLuint) {
	panic!("OpenGL Function pointer of `glVertexAttribP4ui()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribp4uivproc (_: GLuint, _: GLenum, _: GLboolean, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glVertexAttribP4uiv()` is NULL");
}
extern "system" fn dummy_pfnglvertexp2uiproc (_: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glVertexP2ui()` is NULL");
}
extern "system" fn dummy_pfnglvertexp2uivproc (_: GLenum, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glVertexP2uiv()` is NULL");
}
extern "system" fn dummy_pfnglvertexp3uiproc (_: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glVertexP3ui()` is NULL");
}
extern "system" fn dummy_pfnglvertexp3uivproc (_: GLenum, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glVertexP3uiv()` is NULL");
}
extern "system" fn dummy_pfnglvertexp4uiproc (_: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glVertexP4ui()` is NULL");
}
extern "system" fn dummy_pfnglvertexp4uivproc (_: GLenum, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glVertexP4uiv()` is NULL");
}
extern "system" fn dummy_pfngltexcoordp1uiproc (_: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glTexCoordP1ui()` is NULL");
}
extern "system" fn dummy_pfngltexcoordp1uivproc (_: GLenum, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glTexCoordP1uiv()` is NULL");
}
extern "system" fn dummy_pfngltexcoordp2uiproc (_: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glTexCoordP2ui()` is NULL");
}
extern "system" fn dummy_pfngltexcoordp2uivproc (_: GLenum, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glTexCoordP2uiv()` is NULL");
}
extern "system" fn dummy_pfngltexcoordp3uiproc (_: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glTexCoordP3ui()` is NULL");
}
extern "system" fn dummy_pfngltexcoordp3uivproc (_: GLenum, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glTexCoordP3uiv()` is NULL");
}
extern "system" fn dummy_pfngltexcoordp4uiproc (_: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glTexCoordP4ui()` is NULL");
}
extern "system" fn dummy_pfngltexcoordp4uivproc (_: GLenum, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glTexCoordP4uiv()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoordp1uiproc (_: GLenum, _: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glMultiTexCoordP1ui()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoordp1uivproc (_: GLenum, _: GLenum, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glMultiTexCoordP1uiv()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoordp2uiproc (_: GLenum, _: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glMultiTexCoordP2ui()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoordp2uivproc (_: GLenum, _: GLenum, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glMultiTexCoordP2uiv()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoordp3uiproc (_: GLenum, _: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glMultiTexCoordP3ui()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoordp3uivproc (_: GLenum, _: GLenum, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glMultiTexCoordP3uiv()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoordp4uiproc (_: GLenum, _: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glMultiTexCoordP4ui()` is NULL");
}
extern "system" fn dummy_pfnglmultitexcoordp4uivproc (_: GLenum, _: GLenum, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glMultiTexCoordP4uiv()` is NULL");
}
extern "system" fn dummy_pfnglnormalp3uiproc (_: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glNormalP3ui()` is NULL");
}
extern "system" fn dummy_pfnglnormalp3uivproc (_: GLenum, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glNormalP3uiv()` is NULL");
}
extern "system" fn dummy_pfnglcolorp3uiproc (_: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glColorP3ui()` is NULL");
}
extern "system" fn dummy_pfnglcolorp3uivproc (_: GLenum, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glColorP3uiv()` is NULL");
}
extern "system" fn dummy_pfnglcolorp4uiproc (_: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glColorP4ui()` is NULL");
}
extern "system" fn dummy_pfnglcolorp4uivproc (_: GLenum, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glColorP4uiv()` is NULL");
}
extern "system" fn dummy_pfnglsecondarycolorp3uiproc (_: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glSecondaryColorP3ui()` is NULL");
}
extern "system" fn dummy_pfnglsecondarycolorp3uivproc (_: GLenum, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glSecondaryColorP3uiv()` is NULL");
}
pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR: GLenum = 0x88FE;
pub const GL_SRC1_COLOR: GLenum = 0x88F9;
pub const GL_ONE_MINUS_SRC1_COLOR: GLenum = 0x88FA;
pub const GL_ONE_MINUS_SRC1_ALPHA: GLenum = 0x88FB;
pub const GL_MAX_DUAL_SOURCE_DRAW_BUFFERS: GLenum = 0x88FC;
pub const GL_ANY_SAMPLES_PASSED: GLenum = 0x8C2F;
pub const GL_SAMPLER_BINDING: GLenum = 0x8919;
pub const GL_RGB10_A2UI: GLenum = 0x906F;
pub const GL_TEXTURE_SWIZZLE_R: GLenum = 0x8E42;
pub const GL_TEXTURE_SWIZZLE_G: GLenum = 0x8E43;
pub const GL_TEXTURE_SWIZZLE_B: GLenum = 0x8E44;
pub const GL_TEXTURE_SWIZZLE_A: GLenum = 0x8E45;
pub const GL_TEXTURE_SWIZZLE_RGBA: GLenum = 0x8E46;
pub const GL_TIME_ELAPSED: GLenum = 0x88BF;
pub const GL_TIMESTAMP: GLenum = 0x8E28;
pub const GL_INT_2_10_10_10_REV: GLenum = 0x8D9F;

pub trait GL_3_3 {
	fn glBindFragDataLocationIndexed(&self, program: GLuint, colorNumber: GLuint, index: GLuint, name: *const GLchar);
	fn glGetFragDataIndex(&self, program: GLuint, name: *const GLchar) -> GLint;
	fn glGenSamplers(&self, count: GLsizei, samplers: *mut GLuint);
	fn glDeleteSamplers(&self, count: GLsizei, samplers: *const GLuint);
	fn glIsSampler(&self, sampler: GLuint) -> GLboolean;
	fn glBindSampler(&self, unit: GLuint, sampler: GLuint);
	fn glSamplerParameteri(&self, sampler: GLuint, pname: GLenum, param: GLint);
	fn glSamplerParameteriv(&self, sampler: GLuint, pname: GLenum, param: *const GLint);
	fn glSamplerParameterf(&self, sampler: GLuint, pname: GLenum, param: GLfloat);
	fn glSamplerParameterfv(&self, sampler: GLuint, pname: GLenum, param: *const GLfloat);
	fn glSamplerParameterIiv(&self, sampler: GLuint, pname: GLenum, param: *const GLint);
	fn glSamplerParameterIuiv(&self, sampler: GLuint, pname: GLenum, param: *const GLuint);
	fn glGetSamplerParameteriv(&self, sampler: GLuint, pname: GLenum, params: *mut GLint);
	fn glGetSamplerParameterIiv(&self, sampler: GLuint, pname: GLenum, params: *mut GLint);
	fn glGetSamplerParameterfv(&self, sampler: GLuint, pname: GLenum, params: *mut GLfloat);
	fn glGetSamplerParameterIuiv(&self, sampler: GLuint, pname: GLenum, params: *mut GLuint);
	fn glQueryCounter(&self, id: GLuint, target: GLenum);
	fn glGetQueryObjecti64v(&self, id: GLuint, pname: GLenum, params: *mut GLint64);
	fn glGetQueryObjectui64v(&self, id: GLuint, pname: GLenum, params: *mut GLuint64);
	fn glVertexAttribDivisor(&self, index: GLuint, divisor: GLuint);
	fn glVertexAttribP1ui(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint);
	fn glVertexAttribP1uiv(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint);
	fn glVertexAttribP2ui(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint);
	fn glVertexAttribP2uiv(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint);
	fn glVertexAttribP3ui(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint);
	fn glVertexAttribP3uiv(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint);
	fn glVertexAttribP4ui(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint);
	fn glVertexAttribP4uiv(&self, index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint);
	fn glVertexP2ui(&self, type_: GLenum, value: GLuint);
	fn glVertexP2uiv(&self, type_: GLenum, value: *const GLuint);
	fn glVertexP3ui(&self, type_: GLenum, value: GLuint);
	fn glVertexP3uiv(&self, type_: GLenum, value: *const GLuint);
	fn glVertexP4ui(&self, type_: GLenum, value: GLuint);
	fn glVertexP4uiv(&self, type_: GLenum, value: *const GLuint);
	fn glTexCoordP1ui(&self, type_: GLenum, coords: GLuint);
	fn glTexCoordP1uiv(&self, type_: GLenum, coords: *const GLuint);
	fn glTexCoordP2ui(&self, type_: GLenum, coords: GLuint);
	fn glTexCoordP2uiv(&self, type_: GLenum, coords: *const GLuint);
	fn glTexCoordP3ui(&self, type_: GLenum, coords: GLuint);
	fn glTexCoordP3uiv(&self, type_: GLenum, coords: *const GLuint);
	fn glTexCoordP4ui(&self, type_: GLenum, coords: GLuint);
	fn glTexCoordP4uiv(&self, type_: GLenum, coords: *const GLuint);
	fn glMultiTexCoordP1ui(&self, texture: GLenum, type_: GLenum, coords: GLuint);
	fn glMultiTexCoordP1uiv(&self, texture: GLenum, type_: GLenum, coords: *const GLuint);
	fn glMultiTexCoordP2ui(&self, texture: GLenum, type_: GLenum, coords: GLuint);
	fn glMultiTexCoordP2uiv(&self, texture: GLenum, type_: GLenum, coords: *const GLuint);
	fn glMultiTexCoordP3ui(&self, texture: GLenum, type_: GLenum, coords: GLuint);
	fn glMultiTexCoordP3uiv(&self, texture: GLenum, type_: GLenum, coords: *const GLuint);
	fn glMultiTexCoordP4ui(&self, texture: GLenum, type_: GLenum, coords: GLuint);
	fn glMultiTexCoordP4uiv(&self, texture: GLenum, type_: GLenum, coords: *const GLuint);
	fn glNormalP3ui(&self, type_: GLenum, coords: GLuint);
	fn glNormalP3uiv(&self, type_: GLenum, coords: *const GLuint);
	fn glColorP3ui(&self, type_: GLenum, color: GLuint);
	fn glColorP3uiv(&self, type_: GLenum, color: *const GLuint);
	fn glColorP4ui(&self, type_: GLenum, color: GLuint);
	fn glColorP4uiv(&self, type_: GLenum, color: *const GLuint);
	fn glSecondaryColorP3ui(&self, type_: GLenum, color: GLuint);
	fn glSecondaryColorP3uiv(&self, type_: GLenum, color: *const GLuint);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
	pub fn new(base: impl GL_1_0, mut get_proc_address: impl FnMut(&'static str) -> *const c_void) -> Self {
		let (_spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 30300 {
			return Self::default();
		}
		Self {
			available: true,
			bindfragdatalocationindexed: {let proc = get_proc_address("glBindFragDataLocationIndexed"); if proc == null() {dummy_pfnglbindfragdatalocationindexedproc} else {unsafe{transmute(proc)}}},
			getfragdataindex: {let proc = get_proc_address("glGetFragDataIndex"); if proc == null() {dummy_pfnglgetfragdataindexproc} else {unsafe{transmute(proc)}}},
			gensamplers: {let proc = get_proc_address("glGenSamplers"); if proc == null() {dummy_pfnglgensamplersproc} else {unsafe{transmute(proc)}}},
			deletesamplers: {let proc = get_proc_address("glDeleteSamplers"); if proc == null() {dummy_pfngldeletesamplersproc} else {unsafe{transmute(proc)}}},
			issampler: {let proc = get_proc_address("glIsSampler"); if proc == null() {dummy_pfnglissamplerproc} else {unsafe{transmute(proc)}}},
			bindsampler: {let proc = get_proc_address("glBindSampler"); if proc == null() {dummy_pfnglbindsamplerproc} else {unsafe{transmute(proc)}}},
			samplerparameteri: {let proc = get_proc_address("glSamplerParameteri"); if proc == null() {dummy_pfnglsamplerparameteriproc} else {unsafe{transmute(proc)}}},
			samplerparameteriv: {let proc = get_proc_address("glSamplerParameteriv"); if proc == null() {dummy_pfnglsamplerparameterivproc} else {unsafe{transmute(proc)}}},
			samplerparameterf: {let proc = get_proc_address("glSamplerParameterf"); if proc == null() {dummy_pfnglsamplerparameterfproc} else {unsafe{transmute(proc)}}},
			samplerparameterfv: {let proc = get_proc_address("glSamplerParameterfv"); if proc == null() {dummy_pfnglsamplerparameterfvproc} else {unsafe{transmute(proc)}}},
			samplerparameteriiv: {let proc = get_proc_address("glSamplerParameterIiv"); if proc == null() {dummy_pfnglsamplerparameteriivproc} else {unsafe{transmute(proc)}}},
			samplerparameteriuiv: {let proc = get_proc_address("glSamplerParameterIuiv"); if proc == null() {dummy_pfnglsamplerparameteriuivproc} else {unsafe{transmute(proc)}}},
			getsamplerparameteriv: {let proc = get_proc_address("glGetSamplerParameteriv"); if proc == null() {dummy_pfnglgetsamplerparameterivproc} else {unsafe{transmute(proc)}}},
			getsamplerparameteriiv: {let proc = get_proc_address("glGetSamplerParameterIiv"); if proc == null() {dummy_pfnglgetsamplerparameteriivproc} else {unsafe{transmute(proc)}}},
			getsamplerparameterfv: {let proc = get_proc_address("glGetSamplerParameterfv"); if proc == null() {dummy_pfnglgetsamplerparameterfvproc} else {unsafe{transmute(proc)}}},
			getsamplerparameteriuiv: {let proc = get_proc_address("glGetSamplerParameterIuiv"); if proc == null() {dummy_pfnglgetsamplerparameteriuivproc} else {unsafe{transmute(proc)}}},
			querycounter: {let proc = get_proc_address("glQueryCounter"); if proc == null() {dummy_pfnglquerycounterproc} else {unsafe{transmute(proc)}}},
			getqueryobjecti64v: {let proc = get_proc_address("glGetQueryObjecti64v"); if proc == null() {dummy_pfnglgetqueryobjecti64vproc} else {unsafe{transmute(proc)}}},
			getqueryobjectui64v: {let proc = get_proc_address("glGetQueryObjectui64v"); if proc == null() {dummy_pfnglgetqueryobjectui64vproc} else {unsafe{transmute(proc)}}},
			vertexattribdivisor: {let proc = get_proc_address("glVertexAttribDivisor"); if proc == null() {dummy_pfnglvertexattribdivisorproc} else {unsafe{transmute(proc)}}},
			vertexattribp1ui: {let proc = get_proc_address("glVertexAttribP1ui"); if proc == null() {dummy_pfnglvertexattribp1uiproc} else {unsafe{transmute(proc)}}},
			vertexattribp1uiv: {let proc = get_proc_address("glVertexAttribP1uiv"); if proc == null() {dummy_pfnglvertexattribp1uivproc} else {unsafe{transmute(proc)}}},
			vertexattribp2ui: {let proc = get_proc_address("glVertexAttribP2ui"); if proc == null() {dummy_pfnglvertexattribp2uiproc} else {unsafe{transmute(proc)}}},
			vertexattribp2uiv: {let proc = get_proc_address("glVertexAttribP2uiv"); if proc == null() {dummy_pfnglvertexattribp2uivproc} else {unsafe{transmute(proc)}}},
			vertexattribp3ui: {let proc = get_proc_address("glVertexAttribP3ui"); if proc == null() {dummy_pfnglvertexattribp3uiproc} else {unsafe{transmute(proc)}}},
			vertexattribp3uiv: {let proc = get_proc_address("glVertexAttribP3uiv"); if proc == null() {dummy_pfnglvertexattribp3uivproc} else {unsafe{transmute(proc)}}},
			vertexattribp4ui: {let proc = get_proc_address("glVertexAttribP4ui"); if proc == null() {dummy_pfnglvertexattribp4uiproc} else {unsafe{transmute(proc)}}},
			vertexattribp4uiv: {let proc = get_proc_address("glVertexAttribP4uiv"); if proc == null() {dummy_pfnglvertexattribp4uivproc} else {unsafe{transmute(proc)}}},
			vertexp2ui: {let proc = get_proc_address("glVertexP2ui"); if proc == null() {dummy_pfnglvertexp2uiproc} else {unsafe{transmute(proc)}}},
			vertexp2uiv: {let proc = get_proc_address("glVertexP2uiv"); if proc == null() {dummy_pfnglvertexp2uivproc} else {unsafe{transmute(proc)}}},
			vertexp3ui: {let proc = get_proc_address("glVertexP3ui"); if proc == null() {dummy_pfnglvertexp3uiproc} else {unsafe{transmute(proc)}}},
			vertexp3uiv: {let proc = get_proc_address("glVertexP3uiv"); if proc == null() {dummy_pfnglvertexp3uivproc} else {unsafe{transmute(proc)}}},
			vertexp4ui: {let proc = get_proc_address("glVertexP4ui"); if proc == null() {dummy_pfnglvertexp4uiproc} else {unsafe{transmute(proc)}}},
			vertexp4uiv: {let proc = get_proc_address("glVertexP4uiv"); if proc == null() {dummy_pfnglvertexp4uivproc} else {unsafe{transmute(proc)}}},
			texcoordp1ui: {let proc = get_proc_address("glTexCoordP1ui"); if proc == null() {dummy_pfngltexcoordp1uiproc} else {unsafe{transmute(proc)}}},
			texcoordp1uiv: {let proc = get_proc_address("glTexCoordP1uiv"); if proc == null() {dummy_pfngltexcoordp1uivproc} else {unsafe{transmute(proc)}}},
			texcoordp2ui: {let proc = get_proc_address("glTexCoordP2ui"); if proc == null() {dummy_pfngltexcoordp2uiproc} else {unsafe{transmute(proc)}}},
			texcoordp2uiv: {let proc = get_proc_address("glTexCoordP2uiv"); if proc == null() {dummy_pfngltexcoordp2uivproc} else {unsafe{transmute(proc)}}},
			texcoordp3ui: {let proc = get_proc_address("glTexCoordP3ui"); if proc == null() {dummy_pfngltexcoordp3uiproc} else {unsafe{transmute(proc)}}},
			texcoordp3uiv: {let proc = get_proc_address("glTexCoordP3uiv"); if proc == null() {dummy_pfngltexcoordp3uivproc} else {unsafe{transmute(proc)}}},
			texcoordp4ui: {let proc = get_proc_address("glTexCoordP4ui"); if proc == null() {dummy_pfngltexcoordp4uiproc} else {unsafe{transmute(proc)}}},
			texcoordp4uiv: {let proc = get_proc_address("glTexCoordP4uiv"); if proc == null() {dummy_pfngltexcoordp4uivproc} else {unsafe{transmute(proc)}}},
			multitexcoordp1ui: {let proc = get_proc_address("glMultiTexCoordP1ui"); if proc == null() {dummy_pfnglmultitexcoordp1uiproc} else {unsafe{transmute(proc)}}},
			multitexcoordp1uiv: {let proc = get_proc_address("glMultiTexCoordP1uiv"); if proc == null() {dummy_pfnglmultitexcoordp1uivproc} else {unsafe{transmute(proc)}}},
			multitexcoordp2ui: {let proc = get_proc_address("glMultiTexCoordP2ui"); if proc == null() {dummy_pfnglmultitexcoordp2uiproc} else {unsafe{transmute(proc)}}},
			multitexcoordp2uiv: {let proc = get_proc_address("glMultiTexCoordP2uiv"); if proc == null() {dummy_pfnglmultitexcoordp2uivproc} else {unsafe{transmute(proc)}}},
			multitexcoordp3ui: {let proc = get_proc_address("glMultiTexCoordP3ui"); if proc == null() {dummy_pfnglmultitexcoordp3uiproc} else {unsafe{transmute(proc)}}},
			multitexcoordp3uiv: {let proc = get_proc_address("glMultiTexCoordP3uiv"); if proc == null() {dummy_pfnglmultitexcoordp3uivproc} else {unsafe{transmute(proc)}}},
			multitexcoordp4ui: {let proc = get_proc_address("glMultiTexCoordP4ui"); if proc == null() {dummy_pfnglmultitexcoordp4uiproc} else {unsafe{transmute(proc)}}},
			multitexcoordp4uiv: {let proc = get_proc_address("glMultiTexCoordP4uiv"); if proc == null() {dummy_pfnglmultitexcoordp4uivproc} else {unsafe{transmute(proc)}}},
			normalp3ui: {let proc = get_proc_address("glNormalP3ui"); if proc == null() {dummy_pfnglnormalp3uiproc} else {unsafe{transmute(proc)}}},
			normalp3uiv: {let proc = get_proc_address("glNormalP3uiv"); if proc == null() {dummy_pfnglnormalp3uivproc} else {unsafe{transmute(proc)}}},
			colorp3ui: {let proc = get_proc_address("glColorP3ui"); if proc == null() {dummy_pfnglcolorp3uiproc} else {unsafe{transmute(proc)}}},
			colorp3uiv: {let proc = get_proc_address("glColorP3uiv"); if proc == null() {dummy_pfnglcolorp3uivproc} else {unsafe{transmute(proc)}}},
			colorp4ui: {let proc = get_proc_address("glColorP4ui"); if proc == null() {dummy_pfnglcolorp4uiproc} else {unsafe{transmute(proc)}}},
			colorp4uiv: {let proc = get_proc_address("glColorP4uiv"); if proc == null() {dummy_pfnglcolorp4uivproc} else {unsafe{transmute(proc)}}},
			secondarycolorp3ui: {let proc = get_proc_address("glSecondaryColorP3ui"); if proc == null() {dummy_pfnglsecondarycolorp3uiproc} else {unsafe{transmute(proc)}}},
			secondarycolorp3uiv: {let proc = get_proc_address("glSecondaryColorP3uiv"); if proc == null() {dummy_pfnglsecondarycolorp3uivproc} else {unsafe{transmute(proc)}}},
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
			bindfragdatalocationindexed: dummy_pfnglbindfragdatalocationindexedproc,
			getfragdataindex: dummy_pfnglgetfragdataindexproc,
			gensamplers: dummy_pfnglgensamplersproc,
			deletesamplers: dummy_pfngldeletesamplersproc,
			issampler: dummy_pfnglissamplerproc,
			bindsampler: dummy_pfnglbindsamplerproc,
			samplerparameteri: dummy_pfnglsamplerparameteriproc,
			samplerparameteriv: dummy_pfnglsamplerparameterivproc,
			samplerparameterf: dummy_pfnglsamplerparameterfproc,
			samplerparameterfv: dummy_pfnglsamplerparameterfvproc,
			samplerparameteriiv: dummy_pfnglsamplerparameteriivproc,
			samplerparameteriuiv: dummy_pfnglsamplerparameteriuivproc,
			getsamplerparameteriv: dummy_pfnglgetsamplerparameterivproc,
			getsamplerparameteriiv: dummy_pfnglgetsamplerparameteriivproc,
			getsamplerparameterfv: dummy_pfnglgetsamplerparameterfvproc,
			getsamplerparameteriuiv: dummy_pfnglgetsamplerparameteriuivproc,
			querycounter: dummy_pfnglquerycounterproc,
			getqueryobjecti64v: dummy_pfnglgetqueryobjecti64vproc,
			getqueryobjectui64v: dummy_pfnglgetqueryobjectui64vproc,
			vertexattribdivisor: dummy_pfnglvertexattribdivisorproc,
			vertexattribp1ui: dummy_pfnglvertexattribp1uiproc,
			vertexattribp1uiv: dummy_pfnglvertexattribp1uivproc,
			vertexattribp2ui: dummy_pfnglvertexattribp2uiproc,
			vertexattribp2uiv: dummy_pfnglvertexattribp2uivproc,
			vertexattribp3ui: dummy_pfnglvertexattribp3uiproc,
			vertexattribp3uiv: dummy_pfnglvertexattribp3uivproc,
			vertexattribp4ui: dummy_pfnglvertexattribp4uiproc,
			vertexattribp4uiv: dummy_pfnglvertexattribp4uivproc,
			vertexp2ui: dummy_pfnglvertexp2uiproc,
			vertexp2uiv: dummy_pfnglvertexp2uivproc,
			vertexp3ui: dummy_pfnglvertexp3uiproc,
			vertexp3uiv: dummy_pfnglvertexp3uivproc,
			vertexp4ui: dummy_pfnglvertexp4uiproc,
			vertexp4uiv: dummy_pfnglvertexp4uivproc,
			texcoordp1ui: dummy_pfngltexcoordp1uiproc,
			texcoordp1uiv: dummy_pfngltexcoordp1uivproc,
			texcoordp2ui: dummy_pfngltexcoordp2uiproc,
			texcoordp2uiv: dummy_pfngltexcoordp2uivproc,
			texcoordp3ui: dummy_pfngltexcoordp3uiproc,
			texcoordp3uiv: dummy_pfngltexcoordp3uivproc,
			texcoordp4ui: dummy_pfngltexcoordp4uiproc,
			texcoordp4uiv: dummy_pfngltexcoordp4uivproc,
			multitexcoordp1ui: dummy_pfnglmultitexcoordp1uiproc,
			multitexcoordp1uiv: dummy_pfnglmultitexcoordp1uivproc,
			multitexcoordp2ui: dummy_pfnglmultitexcoordp2uiproc,
			multitexcoordp2uiv: dummy_pfnglmultitexcoordp2uivproc,
			multitexcoordp3ui: dummy_pfnglmultitexcoordp3uiproc,
			multitexcoordp3uiv: dummy_pfnglmultitexcoordp3uivproc,
			multitexcoordp4ui: dummy_pfnglmultitexcoordp4uiproc,
			multitexcoordp4uiv: dummy_pfnglmultitexcoordp4uivproc,
			normalp3ui: dummy_pfnglnormalp3uiproc,
			normalp3uiv: dummy_pfnglnormalp3uivproc,
			colorp3ui: dummy_pfnglcolorp3uiproc,
			colorp3uiv: dummy_pfnglcolorp3uivproc,
			colorp4ui: dummy_pfnglcolorp4uiproc,
			colorp4uiv: dummy_pfnglcolorp4uivproc,
			secondarycolorp3ui: dummy_pfnglsecondarycolorp3uiproc,
			secondarycolorp3uiv: dummy_pfnglsecondarycolorp3uivproc,
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
extern "system" fn dummy_pfnglminsampleshadingproc (_: GLfloat) {
	panic!("OpenGL Function pointer of `glMinSampleShading()` is NULL");
}
extern "system" fn dummy_pfnglblendequationiproc (_: GLuint, _: GLenum) {
	panic!("OpenGL Function pointer of `glBlendEquationi()` is NULL");
}
extern "system" fn dummy_pfnglblendequationseparateiproc (_: GLuint, _: GLenum, _: GLenum) {
	panic!("OpenGL Function pointer of `glBlendEquationSeparatei()` is NULL");
}
extern "system" fn dummy_pfnglblendfunciproc (_: GLuint, _: GLenum, _: GLenum) {
	panic!("OpenGL Function pointer of `glBlendFunci()` is NULL");
}
extern "system" fn dummy_pfnglblendfuncseparateiproc (_: GLuint, _: GLenum, _: GLenum, _: GLenum, _: GLenum) {
	panic!("OpenGL Function pointer of `glBlendFuncSeparatei()` is NULL");
}
extern "system" fn dummy_pfngldrawarraysindirectproc (_: GLenum, _: *const c_void) {
	panic!("OpenGL Function pointer of `glDrawArraysIndirect()` is NULL");
}
extern "system" fn dummy_pfngldrawelementsindirectproc (_: GLenum, _: GLenum, _: *const c_void) {
	panic!("OpenGL Function pointer of `glDrawElementsIndirect()` is NULL");
}
extern "system" fn dummy_pfngluniform1dproc (_: GLint, _: GLdouble) {
	panic!("OpenGL Function pointer of `glUniform1d()` is NULL");
}
extern "system" fn dummy_pfngluniform2dproc (_: GLint, _: GLdouble, _: GLdouble) {
	panic!("OpenGL Function pointer of `glUniform2d()` is NULL");
}
extern "system" fn dummy_pfngluniform3dproc (_: GLint, _: GLdouble, _: GLdouble, _: GLdouble) {
	panic!("OpenGL Function pointer of `glUniform3d()` is NULL");
}
extern "system" fn dummy_pfngluniform4dproc (_: GLint, _: GLdouble, _: GLdouble, _: GLdouble, _: GLdouble) {
	panic!("OpenGL Function pointer of `glUniform4d()` is NULL");
}
extern "system" fn dummy_pfngluniform1dvproc (_: GLint, _: GLsizei, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glUniform1dv()` is NULL");
}
extern "system" fn dummy_pfngluniform2dvproc (_: GLint, _: GLsizei, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glUniform2dv()` is NULL");
}
extern "system" fn dummy_pfngluniform3dvproc (_: GLint, _: GLsizei, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glUniform3dv()` is NULL");
}
extern "system" fn dummy_pfngluniform4dvproc (_: GLint, _: GLsizei, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glUniform4dv()` is NULL");
}
extern "system" fn dummy_pfngluniformmatrix2dvproc (_: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glUniformMatrix2dv()` is NULL");
}
extern "system" fn dummy_pfngluniformmatrix3dvproc (_: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glUniformMatrix3dv()` is NULL");
}
extern "system" fn dummy_pfngluniformmatrix4dvproc (_: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glUniformMatrix4dv()` is NULL");
}
extern "system" fn dummy_pfngluniformmatrix2x3dvproc (_: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glUniformMatrix2x3dv()` is NULL");
}
extern "system" fn dummy_pfngluniformmatrix2x4dvproc (_: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glUniformMatrix2x4dv()` is NULL");
}
extern "system" fn dummy_pfngluniformmatrix3x2dvproc (_: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glUniformMatrix3x2dv()` is NULL");
}
extern "system" fn dummy_pfngluniformmatrix3x4dvproc (_: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glUniformMatrix3x4dv()` is NULL");
}
extern "system" fn dummy_pfngluniformmatrix4x2dvproc (_: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glUniformMatrix4x2dv()` is NULL");
}
extern "system" fn dummy_pfngluniformmatrix4x3dvproc (_: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glUniformMatrix4x3dv()` is NULL");
}
extern "system" fn dummy_pfnglgetuniformdvproc (_: GLuint, _: GLint, _: *mut GLdouble) {
	panic!("OpenGL Function pointer of `glGetUniformdv()` is NULL");
}
extern "system" fn dummy_pfnglgetsubroutineuniformlocationproc (_: GLuint, _: GLenum, _: *const GLchar) -> GLint {
	panic!("OpenGL Function pointer of `glGetSubroutineUniformLocation()` is NULL");
}
extern "system" fn dummy_pfnglgetsubroutineindexproc (_: GLuint, _: GLenum, _: *const GLchar) -> GLuint {
	panic!("OpenGL Function pointer of `glGetSubroutineIndex()` is NULL");
}
extern "system" fn dummy_pfnglgetactivesubroutineuniformivproc (_: GLuint, _: GLenum, _: GLuint, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetActiveSubroutineUniformiv()` is NULL");
}
extern "system" fn dummy_pfnglgetactivesubroutineuniformnameproc (_: GLuint, _: GLenum, _: GLuint, _: GLsizei, _: *mut GLsizei, _: *mut GLchar) {
	panic!("OpenGL Function pointer of `glGetActiveSubroutineUniformName()` is NULL");
}
extern "system" fn dummy_pfnglgetactivesubroutinenameproc (_: GLuint, _: GLenum, _: GLuint, _: GLsizei, _: *mut GLsizei, _: *mut GLchar) {
	panic!("OpenGL Function pointer of `glGetActiveSubroutineName()` is NULL");
}
extern "system" fn dummy_pfngluniformsubroutinesuivproc (_: GLenum, _: GLsizei, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glUniformSubroutinesuiv()` is NULL");
}
extern "system" fn dummy_pfnglgetuniformsubroutineuivproc (_: GLenum, _: GLint, _: *mut GLuint) {
	panic!("OpenGL Function pointer of `glGetUniformSubroutineuiv()` is NULL");
}
extern "system" fn dummy_pfnglgetprogramstageivproc (_: GLuint, _: GLenum, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetProgramStageiv()` is NULL");
}
extern "system" fn dummy_pfnglpatchparameteriproc (_: GLenum, _: GLint) {
	panic!("OpenGL Function pointer of `glPatchParameteri()` is NULL");
}
extern "system" fn dummy_pfnglpatchparameterfvproc (_: GLenum, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glPatchParameterfv()` is NULL");
}
extern "system" fn dummy_pfnglbindtransformfeedbackproc (_: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glBindTransformFeedback()` is NULL");
}
extern "system" fn dummy_pfngldeletetransformfeedbacksproc (_: GLsizei, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glDeleteTransformFeedbacks()` is NULL");
}
extern "system" fn dummy_pfnglgentransformfeedbacksproc (_: GLsizei, _: *mut GLuint) {
	panic!("OpenGL Function pointer of `glGenTransformFeedbacks()` is NULL");
}
extern "system" fn dummy_pfnglistransformfeedbackproc (_: GLuint) -> GLboolean {
	panic!("OpenGL Function pointer of `glIsTransformFeedback()` is NULL");
}
extern "system" fn dummy_pfnglpausetransformfeedbackproc () {
	panic!("OpenGL Function pointer of `glPauseTransformFeedback()` is NULL");
}
extern "system" fn dummy_pfnglresumetransformfeedbackproc () {
	panic!("OpenGL Function pointer of `glResumeTransformFeedback()` is NULL");
}
extern "system" fn dummy_pfngldrawtransformfeedbackproc (_: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glDrawTransformFeedback()` is NULL");
}
extern "system" fn dummy_pfngldrawtransformfeedbackstreamproc (_: GLenum, _: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glDrawTransformFeedbackStream()` is NULL");
}
extern "system" fn dummy_pfnglbeginqueryindexedproc (_: GLenum, _: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glBeginQueryIndexed()` is NULL");
}
extern "system" fn dummy_pfnglendqueryindexedproc (_: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glEndQueryIndexed()` is NULL");
}
extern "system" fn dummy_pfnglgetqueryindexedivproc (_: GLenum, _: GLuint, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetQueryIndexediv()` is NULL");
}
pub const GL_SAMPLE_SHADING: GLenum = 0x8C36;
pub const GL_MIN_SAMPLE_SHADING_VALUE: GLenum = 0x8C37;
pub const GL_MIN_PROGRAM_TEXTURE_GATHER_OFFSET: GLenum = 0x8E5E;
pub const GL_MAX_PROGRAM_TEXTURE_GATHER_OFFSET: GLenum = 0x8E5F;
pub const GL_TEXTURE_CUBE_MAP_ARRAY: GLenum = 0x9009;
pub const GL_TEXTURE_BINDING_CUBE_MAP_ARRAY: GLenum = 0x900A;
pub const GL_PROXY_TEXTURE_CUBE_MAP_ARRAY: GLenum = 0x900B;
pub const GL_SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900C;
pub const GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW: GLenum = 0x900D;
pub const GL_INT_SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900E;
pub const GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900F;
pub const GL_DRAW_INDIRECT_BUFFER: GLenum = 0x8F3F;
pub const GL_DRAW_INDIRECT_BUFFER_BINDING: GLenum = 0x8F43;
pub const GL_GEOMETRY_SHADER_INVOCATIONS: GLenum = 0x887F;
pub const GL_MAX_GEOMETRY_SHADER_INVOCATIONS: GLenum = 0x8E5A;
pub const GL_MIN_FRAGMENT_INTERPOLATION_OFFSET: GLenum = 0x8E5B;
pub const GL_MAX_FRAGMENT_INTERPOLATION_OFFSET: GLenum = 0x8E5C;
pub const GL_FRAGMENT_INTERPOLATION_OFFSET_BITS: GLenum = 0x8E5D;
pub const GL_MAX_VERTEX_STREAMS: GLenum = 0x8E71;
pub const GL_DOUBLE_VEC2: GLenum = 0x8FFC;
pub const GL_DOUBLE_VEC3: GLenum = 0x8FFD;
pub const GL_DOUBLE_VEC4: GLenum = 0x8FFE;
pub const GL_DOUBLE_MAT2: GLenum = 0x8F46;
pub const GL_DOUBLE_MAT3: GLenum = 0x8F47;
pub const GL_DOUBLE_MAT4: GLenum = 0x8F48;
pub const GL_DOUBLE_MAT2x3: GLenum = 0x8F49;
pub const GL_DOUBLE_MAT2x4: GLenum = 0x8F4A;
pub const GL_DOUBLE_MAT3x2: GLenum = 0x8F4B;
pub const GL_DOUBLE_MAT3x4: GLenum = 0x8F4C;
pub const GL_DOUBLE_MAT4x2: GLenum = 0x8F4D;
pub const GL_DOUBLE_MAT4x3: GLenum = 0x8F4E;
pub const GL_ACTIVE_SUBROUTINES: GLenum = 0x8DE5;
pub const GL_ACTIVE_SUBROUTINE_UNIFORMS: GLenum = 0x8DE6;
pub const GL_ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS: GLenum = 0x8E47;
pub const GL_ACTIVE_SUBROUTINE_MAX_LENGTH: GLenum = 0x8E48;
pub const GL_ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH: GLenum = 0x8E49;
pub const GL_MAX_SUBROUTINES: GLenum = 0x8DE7;
pub const GL_MAX_SUBROUTINE_UNIFORM_LOCATIONS: GLenum = 0x8DE8;
pub const GL_NUM_COMPATIBLE_SUBROUTINES: GLenum = 0x8E4A;
pub const GL_COMPATIBLE_SUBROUTINES: GLenum = 0x8E4B;
pub const GL_PATCHES: GLenum = 0x000E;
pub const GL_PATCH_VERTICES: GLenum = 0x8E72;
pub const GL_PATCH_DEFAULT_INNER_LEVEL: GLenum = 0x8E73;
pub const GL_PATCH_DEFAULT_OUTER_LEVEL: GLenum = 0x8E74;
pub const GL_TESS_CONTROL_OUTPUT_VERTICES: GLenum = 0x8E75;
pub const GL_TESS_GEN_MODE: GLenum = 0x8E76;
pub const GL_TESS_GEN_SPACING: GLenum = 0x8E77;
pub const GL_TESS_GEN_VERTEX_ORDER: GLenum = 0x8E78;
pub const GL_TESS_GEN_POINT_MODE: GLenum = 0x8E79;
pub const GL_ISOLINES: GLenum = 0x8E7A;
pub const GL_FRACTIONAL_ODD: GLenum = 0x8E7B;
pub const GL_FRACTIONAL_EVEN: GLenum = 0x8E7C;
pub const GL_MAX_PATCH_VERTICES: GLenum = 0x8E7D;
pub const GL_MAX_TESS_GEN_LEVEL: GLenum = 0x8E7E;
pub const GL_MAX_TESS_CONTROL_UNIFORM_COMPONENTS: GLenum = 0x8E7F;
pub const GL_MAX_TESS_EVALUATION_UNIFORM_COMPONENTS: GLenum = 0x8E80;
pub const GL_MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS: GLenum = 0x8E81;
pub const GL_MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS: GLenum = 0x8E82;
pub const GL_MAX_TESS_CONTROL_OUTPUT_COMPONENTS: GLenum = 0x8E83;
pub const GL_MAX_TESS_PATCH_COMPONENTS: GLenum = 0x8E84;
pub const GL_MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS: GLenum = 0x8E85;
pub const GL_MAX_TESS_EVALUATION_OUTPUT_COMPONENTS: GLenum = 0x8E86;
pub const GL_MAX_TESS_CONTROL_UNIFORM_BLOCKS: GLenum = 0x8E89;
pub const GL_MAX_TESS_EVALUATION_UNIFORM_BLOCKS: GLenum = 0x8E8A;
pub const GL_MAX_TESS_CONTROL_INPUT_COMPONENTS: GLenum = 0x886C;
pub const GL_MAX_TESS_EVALUATION_INPUT_COMPONENTS: GLenum = 0x886D;
pub const GL_MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS: GLenum = 0x8E1E;
pub const GL_MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS: GLenum = 0x8E1F;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x84F0;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x84F1;
pub const GL_TESS_EVALUATION_SHADER: GLenum = 0x8E87;
pub const GL_TESS_CONTROL_SHADER: GLenum = 0x8E88;
pub const GL_TRANSFORM_FEEDBACK: GLenum = 0x8E22;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_PAUSED: GLenum = 0x8E23;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_ACTIVE: GLenum = 0x8E24;
pub const GL_TRANSFORM_FEEDBACK_BINDING: GLenum = 0x8E25;
pub const GL_MAX_TRANSFORM_FEEDBACK_BUFFERS: GLenum = 0x8E70;

pub trait GL_4_0 {
	fn glMinSampleShading(&self, value: GLfloat);
	fn glBlendEquationi(&self, buf: GLuint, mode: GLenum);
	fn glBlendEquationSeparatei(&self, buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum);
	fn glBlendFunci(&self, buf: GLuint, src: GLenum, dst: GLenum);
	fn glBlendFuncSeparatei(&self, buf: GLuint, srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum);
	fn glDrawArraysIndirect(&self, mode: GLenum, indirect: *const c_void);
	fn glDrawElementsIndirect(&self, mode: GLenum, type_: GLenum, indirect: *const c_void);
	fn glUniform1d(&self, location: GLint, x: GLdouble);
	fn glUniform2d(&self, location: GLint, x: GLdouble, y: GLdouble);
	fn glUniform3d(&self, location: GLint, x: GLdouble, y: GLdouble, z: GLdouble);
	fn glUniform4d(&self, location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
	fn glUniform1dv(&self, location: GLint, count: GLsizei, value: *const GLdouble);
	fn glUniform2dv(&self, location: GLint, count: GLsizei, value: *const GLdouble);
	fn glUniform3dv(&self, location: GLint, count: GLsizei, value: *const GLdouble);
	fn glUniform4dv(&self, location: GLint, count: GLsizei, value: *const GLdouble);
	fn glUniformMatrix2dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
	fn glUniformMatrix3dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
	fn glUniformMatrix4dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
	fn glUniformMatrix2x3dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
	fn glUniformMatrix2x4dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
	fn glUniformMatrix3x2dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
	fn glUniformMatrix3x4dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
	fn glUniformMatrix4x2dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
	fn glUniformMatrix4x3dv(&self, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
	fn glGetUniformdv(&self, program: GLuint, location: GLint, params: *mut GLdouble);
	fn glGetSubroutineUniformLocation(&self, program: GLuint, shadertype: GLenum, name: *const GLchar) -> GLint;
	fn glGetSubroutineIndex(&self, program: GLuint, shadertype: GLenum, name: *const GLchar) -> GLuint;
	fn glGetActiveSubroutineUniformiv(&self, program: GLuint, shadertype: GLenum, index: GLuint, pname: GLenum, values: *mut GLint);
	fn glGetActiveSubroutineUniformName(&self, program: GLuint, shadertype: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar);
	fn glGetActiveSubroutineName(&self, program: GLuint, shadertype: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar);
	fn glUniformSubroutinesuiv(&self, shadertype: GLenum, count: GLsizei, indices: *const GLuint);
	fn glGetUniformSubroutineuiv(&self, shadertype: GLenum, location: GLint, params: *mut GLuint);
	fn glGetProgramStageiv(&self, program: GLuint, shadertype: GLenum, pname: GLenum, values: *mut GLint);
	fn glPatchParameteri(&self, pname: GLenum, value: GLint);
	fn glPatchParameterfv(&self, pname: GLenum, values: *const GLfloat);
	fn glBindTransformFeedback(&self, target: GLenum, id: GLuint);
	fn glDeleteTransformFeedbacks(&self, n: GLsizei, ids: *const GLuint);
	fn glGenTransformFeedbacks(&self, n: GLsizei, ids: *mut GLuint);
	fn glIsTransformFeedback(&self, id: GLuint) -> GLboolean;
	fn glPauseTransformFeedback(&self);
	fn glResumeTransformFeedback(&self);
	fn glDrawTransformFeedback(&self, mode: GLenum, id: GLuint);
	fn glDrawTransformFeedbackStream(&self, mode: GLenum, id: GLuint, stream: GLuint);
	fn glBeginQueryIndexed(&self, target: GLenum, index: GLuint, id: GLuint);
	fn glEndQueryIndexed(&self, target: GLenum, index: GLuint);
	fn glGetQueryIndexediv(&self, target: GLenum, index: GLuint, pname: GLenum, params: *mut GLint);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
	pub fn new(base: impl GL_1_0, mut get_proc_address: impl FnMut(&'static str) -> *const c_void) -> Self {
		let (_spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 40000 {
			return Self::default();
		}
		Self {
			available: true,
			minsampleshading: {let proc = get_proc_address("glMinSampleShading"); if proc == null() {dummy_pfnglminsampleshadingproc} else {unsafe{transmute(proc)}}},
			blendequationi: {let proc = get_proc_address("glBlendEquationi"); if proc == null() {dummy_pfnglblendequationiproc} else {unsafe{transmute(proc)}}},
			blendequationseparatei: {let proc = get_proc_address("glBlendEquationSeparatei"); if proc == null() {dummy_pfnglblendequationseparateiproc} else {unsafe{transmute(proc)}}},
			blendfunci: {let proc = get_proc_address("glBlendFunci"); if proc == null() {dummy_pfnglblendfunciproc} else {unsafe{transmute(proc)}}},
			blendfuncseparatei: {let proc = get_proc_address("glBlendFuncSeparatei"); if proc == null() {dummy_pfnglblendfuncseparateiproc} else {unsafe{transmute(proc)}}},
			drawarraysindirect: {let proc = get_proc_address("glDrawArraysIndirect"); if proc == null() {dummy_pfngldrawarraysindirectproc} else {unsafe{transmute(proc)}}},
			drawelementsindirect: {let proc = get_proc_address("glDrawElementsIndirect"); if proc == null() {dummy_pfngldrawelementsindirectproc} else {unsafe{transmute(proc)}}},
			uniform1d: {let proc = get_proc_address("glUniform1d"); if proc == null() {dummy_pfngluniform1dproc} else {unsafe{transmute(proc)}}},
			uniform2d: {let proc = get_proc_address("glUniform2d"); if proc == null() {dummy_pfngluniform2dproc} else {unsafe{transmute(proc)}}},
			uniform3d: {let proc = get_proc_address("glUniform3d"); if proc == null() {dummy_pfngluniform3dproc} else {unsafe{transmute(proc)}}},
			uniform4d: {let proc = get_proc_address("glUniform4d"); if proc == null() {dummy_pfngluniform4dproc} else {unsafe{transmute(proc)}}},
			uniform1dv: {let proc = get_proc_address("glUniform1dv"); if proc == null() {dummy_pfngluniform1dvproc} else {unsafe{transmute(proc)}}},
			uniform2dv: {let proc = get_proc_address("glUniform2dv"); if proc == null() {dummy_pfngluniform2dvproc} else {unsafe{transmute(proc)}}},
			uniform3dv: {let proc = get_proc_address("glUniform3dv"); if proc == null() {dummy_pfngluniform3dvproc} else {unsafe{transmute(proc)}}},
			uniform4dv: {let proc = get_proc_address("glUniform4dv"); if proc == null() {dummy_pfngluniform4dvproc} else {unsafe{transmute(proc)}}},
			uniformmatrix2dv: {let proc = get_proc_address("glUniformMatrix2dv"); if proc == null() {dummy_pfngluniformmatrix2dvproc} else {unsafe{transmute(proc)}}},
			uniformmatrix3dv: {let proc = get_proc_address("glUniformMatrix3dv"); if proc == null() {dummy_pfngluniformmatrix3dvproc} else {unsafe{transmute(proc)}}},
			uniformmatrix4dv: {let proc = get_proc_address("glUniformMatrix4dv"); if proc == null() {dummy_pfngluniformmatrix4dvproc} else {unsafe{transmute(proc)}}},
			uniformmatrix2x3dv: {let proc = get_proc_address("glUniformMatrix2x3dv"); if proc == null() {dummy_pfngluniformmatrix2x3dvproc} else {unsafe{transmute(proc)}}},
			uniformmatrix2x4dv: {let proc = get_proc_address("glUniformMatrix2x4dv"); if proc == null() {dummy_pfngluniformmatrix2x4dvproc} else {unsafe{transmute(proc)}}},
			uniformmatrix3x2dv: {let proc = get_proc_address("glUniformMatrix3x2dv"); if proc == null() {dummy_pfngluniformmatrix3x2dvproc} else {unsafe{transmute(proc)}}},
			uniformmatrix3x4dv: {let proc = get_proc_address("glUniformMatrix3x4dv"); if proc == null() {dummy_pfngluniformmatrix3x4dvproc} else {unsafe{transmute(proc)}}},
			uniformmatrix4x2dv: {let proc = get_proc_address("glUniformMatrix4x2dv"); if proc == null() {dummy_pfngluniformmatrix4x2dvproc} else {unsafe{transmute(proc)}}},
			uniformmatrix4x3dv: {let proc = get_proc_address("glUniformMatrix4x3dv"); if proc == null() {dummy_pfngluniformmatrix4x3dvproc} else {unsafe{transmute(proc)}}},
			getuniformdv: {let proc = get_proc_address("glGetUniformdv"); if proc == null() {dummy_pfnglgetuniformdvproc} else {unsafe{transmute(proc)}}},
			getsubroutineuniformlocation: {let proc = get_proc_address("glGetSubroutineUniformLocation"); if proc == null() {dummy_pfnglgetsubroutineuniformlocationproc} else {unsafe{transmute(proc)}}},
			getsubroutineindex: {let proc = get_proc_address("glGetSubroutineIndex"); if proc == null() {dummy_pfnglgetsubroutineindexproc} else {unsafe{transmute(proc)}}},
			getactivesubroutineuniformiv: {let proc = get_proc_address("glGetActiveSubroutineUniformiv"); if proc == null() {dummy_pfnglgetactivesubroutineuniformivproc} else {unsafe{transmute(proc)}}},
			getactivesubroutineuniformname: {let proc = get_proc_address("glGetActiveSubroutineUniformName"); if proc == null() {dummy_pfnglgetactivesubroutineuniformnameproc} else {unsafe{transmute(proc)}}},
			getactivesubroutinename: {let proc = get_proc_address("glGetActiveSubroutineName"); if proc == null() {dummy_pfnglgetactivesubroutinenameproc} else {unsafe{transmute(proc)}}},
			uniformsubroutinesuiv: {let proc = get_proc_address("glUniformSubroutinesuiv"); if proc == null() {dummy_pfngluniformsubroutinesuivproc} else {unsafe{transmute(proc)}}},
			getuniformsubroutineuiv: {let proc = get_proc_address("glGetUniformSubroutineuiv"); if proc == null() {dummy_pfnglgetuniformsubroutineuivproc} else {unsafe{transmute(proc)}}},
			getprogramstageiv: {let proc = get_proc_address("glGetProgramStageiv"); if proc == null() {dummy_pfnglgetprogramstageivproc} else {unsafe{transmute(proc)}}},
			patchparameteri: {let proc = get_proc_address("glPatchParameteri"); if proc == null() {dummy_pfnglpatchparameteriproc} else {unsafe{transmute(proc)}}},
			patchparameterfv: {let proc = get_proc_address("glPatchParameterfv"); if proc == null() {dummy_pfnglpatchparameterfvproc} else {unsafe{transmute(proc)}}},
			bindtransformfeedback: {let proc = get_proc_address("glBindTransformFeedback"); if proc == null() {dummy_pfnglbindtransformfeedbackproc} else {unsafe{transmute(proc)}}},
			deletetransformfeedbacks: {let proc = get_proc_address("glDeleteTransformFeedbacks"); if proc == null() {dummy_pfngldeletetransformfeedbacksproc} else {unsafe{transmute(proc)}}},
			gentransformfeedbacks: {let proc = get_proc_address("glGenTransformFeedbacks"); if proc == null() {dummy_pfnglgentransformfeedbacksproc} else {unsafe{transmute(proc)}}},
			istransformfeedback: {let proc = get_proc_address("glIsTransformFeedback"); if proc == null() {dummy_pfnglistransformfeedbackproc} else {unsafe{transmute(proc)}}},
			pausetransformfeedback: {let proc = get_proc_address("glPauseTransformFeedback"); if proc == null() {dummy_pfnglpausetransformfeedbackproc} else {unsafe{transmute(proc)}}},
			resumetransformfeedback: {let proc = get_proc_address("glResumeTransformFeedback"); if proc == null() {dummy_pfnglresumetransformfeedbackproc} else {unsafe{transmute(proc)}}},
			drawtransformfeedback: {let proc = get_proc_address("glDrawTransformFeedback"); if proc == null() {dummy_pfngldrawtransformfeedbackproc} else {unsafe{transmute(proc)}}},
			drawtransformfeedbackstream: {let proc = get_proc_address("glDrawTransformFeedbackStream"); if proc == null() {dummy_pfngldrawtransformfeedbackstreamproc} else {unsafe{transmute(proc)}}},
			beginqueryindexed: {let proc = get_proc_address("glBeginQueryIndexed"); if proc == null() {dummy_pfnglbeginqueryindexedproc} else {unsafe{transmute(proc)}}},
			endqueryindexed: {let proc = get_proc_address("glEndQueryIndexed"); if proc == null() {dummy_pfnglendqueryindexedproc} else {unsafe{transmute(proc)}}},
			getqueryindexediv: {let proc = get_proc_address("glGetQueryIndexediv"); if proc == null() {dummy_pfnglgetqueryindexedivproc} else {unsafe{transmute(proc)}}},
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
			minsampleshading: dummy_pfnglminsampleshadingproc,
			blendequationi: dummy_pfnglblendequationiproc,
			blendequationseparatei: dummy_pfnglblendequationseparateiproc,
			blendfunci: dummy_pfnglblendfunciproc,
			blendfuncseparatei: dummy_pfnglblendfuncseparateiproc,
			drawarraysindirect: dummy_pfngldrawarraysindirectproc,
			drawelementsindirect: dummy_pfngldrawelementsindirectproc,
			uniform1d: dummy_pfngluniform1dproc,
			uniform2d: dummy_pfngluniform2dproc,
			uniform3d: dummy_pfngluniform3dproc,
			uniform4d: dummy_pfngluniform4dproc,
			uniform1dv: dummy_pfngluniform1dvproc,
			uniform2dv: dummy_pfngluniform2dvproc,
			uniform3dv: dummy_pfngluniform3dvproc,
			uniform4dv: dummy_pfngluniform4dvproc,
			uniformmatrix2dv: dummy_pfngluniformmatrix2dvproc,
			uniformmatrix3dv: dummy_pfngluniformmatrix3dvproc,
			uniformmatrix4dv: dummy_pfngluniformmatrix4dvproc,
			uniformmatrix2x3dv: dummy_pfngluniformmatrix2x3dvproc,
			uniformmatrix2x4dv: dummy_pfngluniformmatrix2x4dvproc,
			uniformmatrix3x2dv: dummy_pfngluniformmatrix3x2dvproc,
			uniformmatrix3x4dv: dummy_pfngluniformmatrix3x4dvproc,
			uniformmatrix4x2dv: dummy_pfngluniformmatrix4x2dvproc,
			uniformmatrix4x3dv: dummy_pfngluniformmatrix4x3dvproc,
			getuniformdv: dummy_pfnglgetuniformdvproc,
			getsubroutineuniformlocation: dummy_pfnglgetsubroutineuniformlocationproc,
			getsubroutineindex: dummy_pfnglgetsubroutineindexproc,
			getactivesubroutineuniformiv: dummy_pfnglgetactivesubroutineuniformivproc,
			getactivesubroutineuniformname: dummy_pfnglgetactivesubroutineuniformnameproc,
			getactivesubroutinename: dummy_pfnglgetactivesubroutinenameproc,
			uniformsubroutinesuiv: dummy_pfngluniformsubroutinesuivproc,
			getuniformsubroutineuiv: dummy_pfnglgetuniformsubroutineuivproc,
			getprogramstageiv: dummy_pfnglgetprogramstageivproc,
			patchparameteri: dummy_pfnglpatchparameteriproc,
			patchparameterfv: dummy_pfnglpatchparameterfvproc,
			bindtransformfeedback: dummy_pfnglbindtransformfeedbackproc,
			deletetransformfeedbacks: dummy_pfngldeletetransformfeedbacksproc,
			gentransformfeedbacks: dummy_pfnglgentransformfeedbacksproc,
			istransformfeedback: dummy_pfnglistransformfeedbackproc,
			pausetransformfeedback: dummy_pfnglpausetransformfeedbackproc,
			resumetransformfeedback: dummy_pfnglresumetransformfeedbackproc,
			drawtransformfeedback: dummy_pfngldrawtransformfeedbackproc,
			drawtransformfeedbackstream: dummy_pfngldrawtransformfeedbackstreamproc,
			beginqueryindexed: dummy_pfnglbeginqueryindexedproc,
			endqueryindexed: dummy_pfnglendqueryindexedproc,
			getqueryindexediv: dummy_pfnglgetqueryindexedivproc,
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
extern "system" fn dummy_pfnglreleaseshadercompilerproc () {
	panic!("OpenGL Function pointer of `glReleaseShaderCompiler()` is NULL");
}
extern "system" fn dummy_pfnglshaderbinaryproc (_: GLsizei, _: *const GLuint, _: GLenum, _: *const c_void, _: GLsizei) {
	panic!("OpenGL Function pointer of `glShaderBinary()` is NULL");
}
extern "system" fn dummy_pfnglgetshaderprecisionformatproc (_: GLenum, _: GLenum, _: *mut GLint, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetShaderPrecisionFormat()` is NULL");
}
extern "system" fn dummy_pfngldepthrangefproc (_: GLfloat, _: GLfloat) {
	panic!("OpenGL Function pointer of `glDepthRangef()` is NULL");
}
extern "system" fn dummy_pfnglcleardepthfproc (_: GLfloat) {
	panic!("OpenGL Function pointer of `glClearDepthf()` is NULL");
}
extern "system" fn dummy_pfnglgetprogrambinaryproc (_: GLuint, _: GLsizei, _: *mut GLsizei, _: *mut GLenum, _: *mut c_void) {
	panic!("OpenGL Function pointer of `glGetProgramBinary()` is NULL");
}
extern "system" fn dummy_pfnglprogrambinaryproc (_: GLuint, _: GLenum, _: *const c_void, _: GLsizei) {
	panic!("OpenGL Function pointer of `glProgramBinary()` is NULL");
}
extern "system" fn dummy_pfnglprogramparameteriproc (_: GLuint, _: GLenum, _: GLint) {
	panic!("OpenGL Function pointer of `glProgramParameteri()` is NULL");
}
extern "system" fn dummy_pfngluseprogramstagesproc (_: GLuint, _: GLbitfield, _: GLuint) {
	panic!("OpenGL Function pointer of `glUseProgramStages()` is NULL");
}
extern "system" fn dummy_pfnglactiveshaderprogramproc (_: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glActiveShaderProgram()` is NULL");
}
extern "system" fn dummy_pfnglcreateshaderprogramvproc (_: GLenum, _: GLsizei, _: *const *const GLchar) -> GLuint {
	panic!("OpenGL Function pointer of `glCreateShaderProgramv()` is NULL");
}
extern "system" fn dummy_pfnglbindprogrampipelineproc (_: GLuint) {
	panic!("OpenGL Function pointer of `glBindProgramPipeline()` is NULL");
}
extern "system" fn dummy_pfngldeleteprogrampipelinesproc (_: GLsizei, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glDeleteProgramPipelines()` is NULL");
}
extern "system" fn dummy_pfnglgenprogrampipelinesproc (_: GLsizei, _: *mut GLuint) {
	panic!("OpenGL Function pointer of `glGenProgramPipelines()` is NULL");
}
extern "system" fn dummy_pfnglisprogrampipelineproc (_: GLuint) -> GLboolean {
	panic!("OpenGL Function pointer of `glIsProgramPipeline()` is NULL");
}
extern "system" fn dummy_pfnglgetprogrampipelineivproc (_: GLuint, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetProgramPipelineiv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform1iproc (_: GLuint, _: GLint, _: GLint) {
	panic!("OpenGL Function pointer of `glProgramUniform1i()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform1ivproc (_: GLuint, _: GLint, _: GLsizei, _: *const GLint) {
	panic!("OpenGL Function pointer of `glProgramUniform1iv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform1fproc (_: GLuint, _: GLint, _: GLfloat) {
	panic!("OpenGL Function pointer of `glProgramUniform1f()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform1fvproc (_: GLuint, _: GLint, _: GLsizei, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glProgramUniform1fv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform1dproc (_: GLuint, _: GLint, _: GLdouble) {
	panic!("OpenGL Function pointer of `glProgramUniform1d()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform1dvproc (_: GLuint, _: GLint, _: GLsizei, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glProgramUniform1dv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform1uiproc (_: GLuint, _: GLint, _: GLuint) {
	panic!("OpenGL Function pointer of `glProgramUniform1ui()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform1uivproc (_: GLuint, _: GLint, _: GLsizei, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glProgramUniform1uiv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform2iproc (_: GLuint, _: GLint, _: GLint, _: GLint) {
	panic!("OpenGL Function pointer of `glProgramUniform2i()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform2ivproc (_: GLuint, _: GLint, _: GLsizei, _: *const GLint) {
	panic!("OpenGL Function pointer of `glProgramUniform2iv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform2fproc (_: GLuint, _: GLint, _: GLfloat, _: GLfloat) {
	panic!("OpenGL Function pointer of `glProgramUniform2f()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform2fvproc (_: GLuint, _: GLint, _: GLsizei, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glProgramUniform2fv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform2dproc (_: GLuint, _: GLint, _: GLdouble, _: GLdouble) {
	panic!("OpenGL Function pointer of `glProgramUniform2d()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform2dvproc (_: GLuint, _: GLint, _: GLsizei, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glProgramUniform2dv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform2uiproc (_: GLuint, _: GLint, _: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glProgramUniform2ui()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform2uivproc (_: GLuint, _: GLint, _: GLsizei, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glProgramUniform2uiv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform3iproc (_: GLuint, _: GLint, _: GLint, _: GLint, _: GLint) {
	panic!("OpenGL Function pointer of `glProgramUniform3i()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform3ivproc (_: GLuint, _: GLint, _: GLsizei, _: *const GLint) {
	panic!("OpenGL Function pointer of `glProgramUniform3iv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform3fproc (_: GLuint, _: GLint, _: GLfloat, _: GLfloat, _: GLfloat) {
	panic!("OpenGL Function pointer of `glProgramUniform3f()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform3fvproc (_: GLuint, _: GLint, _: GLsizei, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glProgramUniform3fv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform3dproc (_: GLuint, _: GLint, _: GLdouble, _: GLdouble, _: GLdouble) {
	panic!("OpenGL Function pointer of `glProgramUniform3d()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform3dvproc (_: GLuint, _: GLint, _: GLsizei, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glProgramUniform3dv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform3uiproc (_: GLuint, _: GLint, _: GLuint, _: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glProgramUniform3ui()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform3uivproc (_: GLuint, _: GLint, _: GLsizei, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glProgramUniform3uiv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform4iproc (_: GLuint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint) {
	panic!("OpenGL Function pointer of `glProgramUniform4i()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform4ivproc (_: GLuint, _: GLint, _: GLsizei, _: *const GLint) {
	panic!("OpenGL Function pointer of `glProgramUniform4iv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform4fproc (_: GLuint, _: GLint, _: GLfloat, _: GLfloat, _: GLfloat, _: GLfloat) {
	panic!("OpenGL Function pointer of `glProgramUniform4f()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform4fvproc (_: GLuint, _: GLint, _: GLsizei, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glProgramUniform4fv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform4dproc (_: GLuint, _: GLint, _: GLdouble, _: GLdouble, _: GLdouble, _: GLdouble) {
	panic!("OpenGL Function pointer of `glProgramUniform4d()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform4dvproc (_: GLuint, _: GLint, _: GLsizei, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glProgramUniform4dv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform4uiproc (_: GLuint, _: GLint, _: GLuint, _: GLuint, _: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glProgramUniform4ui()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniform4uivproc (_: GLuint, _: GLint, _: GLsizei, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glProgramUniform4uiv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniformmatrix2fvproc (_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glProgramUniformMatrix2fv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniformmatrix3fvproc (_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glProgramUniformMatrix3fv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniformmatrix4fvproc (_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glProgramUniformMatrix4fv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniformmatrix2dvproc (_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glProgramUniformMatrix2dv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniformmatrix3dvproc (_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glProgramUniformMatrix3dv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniformmatrix4dvproc (_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glProgramUniformMatrix4dv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniformmatrix2x3fvproc (_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glProgramUniformMatrix2x3fv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniformmatrix3x2fvproc (_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glProgramUniformMatrix3x2fv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniformmatrix2x4fvproc (_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glProgramUniformMatrix2x4fv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniformmatrix4x2fvproc (_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glProgramUniformMatrix4x2fv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniformmatrix3x4fvproc (_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glProgramUniformMatrix3x4fv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniformmatrix4x3fvproc (_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glProgramUniformMatrix4x3fv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniformmatrix2x3dvproc (_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glProgramUniformMatrix2x3dv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniformmatrix3x2dvproc (_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glProgramUniformMatrix3x2dv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniformmatrix2x4dvproc (_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glProgramUniformMatrix2x4dv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniformmatrix4x2dvproc (_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glProgramUniformMatrix4x2dv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniformmatrix3x4dvproc (_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glProgramUniformMatrix3x4dv()` is NULL");
}
extern "system" fn dummy_pfnglprogramuniformmatrix4x3dvproc (_: GLuint, _: GLint, _: GLsizei, _: GLboolean, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glProgramUniformMatrix4x3dv()` is NULL");
}
extern "system" fn dummy_pfnglvalidateprogrampipelineproc (_: GLuint) {
	panic!("OpenGL Function pointer of `glValidateProgramPipeline()` is NULL");
}
extern "system" fn dummy_pfnglgetprogrampipelineinfologproc (_: GLuint, _: GLsizei, _: *mut GLsizei, _: *mut GLchar) {
	panic!("OpenGL Function pointer of `glGetProgramPipelineInfoLog()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribl1dproc (_: GLuint, _: GLdouble) {
	panic!("OpenGL Function pointer of `glVertexAttribL1d()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribl2dproc (_: GLuint, _: GLdouble, _: GLdouble) {
	panic!("OpenGL Function pointer of `glVertexAttribL2d()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribl3dproc (_: GLuint, _: GLdouble, _: GLdouble, _: GLdouble) {
	panic!("OpenGL Function pointer of `glVertexAttribL3d()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribl4dproc (_: GLuint, _: GLdouble, _: GLdouble, _: GLdouble, _: GLdouble) {
	panic!("OpenGL Function pointer of `glVertexAttribL4d()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribl1dvproc (_: GLuint, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glVertexAttribL1dv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribl2dvproc (_: GLuint, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glVertexAttribL2dv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribl3dvproc (_: GLuint, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glVertexAttribL3dv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribl4dvproc (_: GLuint, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glVertexAttribL4dv()` is NULL");
}
extern "system" fn dummy_pfnglvertexattriblpointerproc (_: GLuint, _: GLint, _: GLenum, _: GLsizei, _: *const c_void) {
	panic!("OpenGL Function pointer of `glVertexAttribLPointer()` is NULL");
}
extern "system" fn dummy_pfnglgetvertexattribldvproc (_: GLuint, _: GLenum, _: *mut GLdouble) {
	panic!("OpenGL Function pointer of `glGetVertexAttribLdv()` is NULL");
}
extern "system" fn dummy_pfnglviewportarrayvproc (_: GLuint, _: GLsizei, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glViewportArrayv()` is NULL");
}
extern "system" fn dummy_pfnglviewportindexedfproc (_: GLuint, _: GLfloat, _: GLfloat, _: GLfloat, _: GLfloat) {
	panic!("OpenGL Function pointer of `glViewportIndexedf()` is NULL");
}
extern "system" fn dummy_pfnglviewportindexedfvproc (_: GLuint, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glViewportIndexedfv()` is NULL");
}
extern "system" fn dummy_pfnglscissorarrayvproc (_: GLuint, _: GLsizei, _: *const GLint) {
	panic!("OpenGL Function pointer of `glScissorArrayv()` is NULL");
}
extern "system" fn dummy_pfnglscissorindexedproc (_: GLuint, _: GLint, _: GLint, _: GLsizei, _: GLsizei) {
	panic!("OpenGL Function pointer of `glScissorIndexed()` is NULL");
}
extern "system" fn dummy_pfnglscissorindexedvproc (_: GLuint, _: *const GLint) {
	panic!("OpenGL Function pointer of `glScissorIndexedv()` is NULL");
}
extern "system" fn dummy_pfngldepthrangearrayvproc (_: GLuint, _: GLsizei, _: *const GLdouble) {
	panic!("OpenGL Function pointer of `glDepthRangeArrayv()` is NULL");
}
extern "system" fn dummy_pfngldepthrangeindexedproc (_: GLuint, _: GLdouble, _: GLdouble) {
	panic!("OpenGL Function pointer of `glDepthRangeIndexed()` is NULL");
}
extern "system" fn dummy_pfnglgetfloati_vproc (_: GLenum, _: GLuint, _: *mut GLfloat) {
	panic!("OpenGL Function pointer of `glGetFloati_v()` is NULL");
}
extern "system" fn dummy_pfnglgetdoublei_vproc (_: GLenum, _: GLuint, _: *mut GLdouble) {
	panic!("OpenGL Function pointer of `glGetDoublei_v()` is NULL");
}
pub const GL_FIXED: GLenum = 0x140C;
pub const GL_IMPLEMENTATION_COLOR_READ_TYPE: GLenum = 0x8B9A;
pub const GL_IMPLEMENTATION_COLOR_READ_FORMAT: GLenum = 0x8B9B;
pub const GL_LOW_FLOAT: GLenum = 0x8DF0;
pub const GL_MEDIUM_FLOAT: GLenum = 0x8DF1;
pub const GL_HIGH_FLOAT: GLenum = 0x8DF2;
pub const GL_LOW_INT: GLenum = 0x8DF3;
pub const GL_MEDIUM_INT: GLenum = 0x8DF4;
pub const GL_HIGH_INT: GLenum = 0x8DF5;
pub const GL_SHADER_COMPILER: GLenum = 0x8DFA;
pub const GL_SHADER_BINARY_FORMATS: GLenum = 0x8DF8;
pub const GL_NUM_SHADER_BINARY_FORMATS: GLenum = 0x8DF9;
pub const GL_MAX_VERTEX_UNIFORM_VECTORS: GLenum = 0x8DFB;
pub const GL_MAX_VARYING_VECTORS: GLenum = 0x8DFC;
pub const GL_MAX_FRAGMENT_UNIFORM_VECTORS: GLenum = 0x8DFD;
pub const GL_RGB565: GLenum = 0x8D62;
pub const GL_PROGRAM_BINARY_RETRIEVABLE_HINT: GLenum = 0x8257;
pub const GL_PROGRAM_BINARY_LENGTH: GLenum = 0x8741;
pub const GL_NUM_PROGRAM_BINARY_FORMATS: GLenum = 0x87FE;
pub const GL_PROGRAM_BINARY_FORMATS: GLenum = 0x87FF;
pub const GL_VERTEX_SHADER_BIT: GLbitfield = 0x00000001;
pub const GL_FRAGMENT_SHADER_BIT: GLbitfield = 0x00000002;
pub const GL_GEOMETRY_SHADER_BIT: GLbitfield = 0x00000004;
pub const GL_TESS_CONTROL_SHADER_BIT: GLbitfield = 0x00000008;
pub const GL_TESS_EVALUATION_SHADER_BIT: GLbitfield = 0x00000010;
pub const GL_ALL_SHADER_BITS: GLbitfield = 0xFFFFFFFF;
pub const GL_PROGRAM_SEPARABLE: GLenum = 0x8258;
pub const GL_ACTIVE_PROGRAM: GLenum = 0x8259;
pub const GL_PROGRAM_PIPELINE_BINDING: GLenum = 0x825A;
pub const GL_MAX_VIEWPORTS: GLenum = 0x825B;
pub const GL_VIEWPORT_SUBPIXEL_BITS: GLenum = 0x825C;
pub const GL_VIEWPORT_BOUNDS_RANGE: GLenum = 0x825D;
pub const GL_LAYER_PROVOKING_VERTEX: GLenum = 0x825E;
pub const GL_VIEWPORT_INDEX_PROVOKING_VERTEX: GLenum = 0x825F;
pub const GL_UNDEFINED_VERTEX: GLenum = 0x8260;

pub trait GL_4_1 {
	fn glReleaseShaderCompiler(&self);
	fn glShaderBinary(&self, count: GLsizei, shaders: *const GLuint, binaryFormat: GLenum, binary: *const c_void, length: GLsizei);
	fn glGetShaderPrecisionFormat(&self, shadertype: GLenum, precisiontype: GLenum, range: *mut GLint, precision: *mut GLint);
	fn glDepthRangef(&self, n: GLfloat, f: GLfloat);
	fn glClearDepthf(&self, d: GLfloat);
	fn glGetProgramBinary(&self, program: GLuint, bufSize: GLsizei, length: *mut GLsizei, binaryFormat: *mut GLenum, binary: *mut c_void);
	fn glProgramBinary(&self, program: GLuint, binaryFormat: GLenum, binary: *const c_void, length: GLsizei);
	fn glProgramParameteri(&self, program: GLuint, pname: GLenum, value: GLint);
	fn glUseProgramStages(&self, pipeline: GLuint, stages: GLbitfield, program: GLuint);
	fn glActiveShaderProgram(&self, pipeline: GLuint, program: GLuint);
	fn glCreateShaderProgramv(&self, type_: GLenum, count: GLsizei, strings: *const *const GLchar) -> GLuint;
	fn glBindProgramPipeline(&self, pipeline: GLuint);
	fn glDeleteProgramPipelines(&self, n: GLsizei, pipelines: *const GLuint);
	fn glGenProgramPipelines(&self, n: GLsizei, pipelines: *mut GLuint);
	fn glIsProgramPipeline(&self, pipeline: GLuint) -> GLboolean;
	fn glGetProgramPipelineiv(&self, pipeline: GLuint, pname: GLenum, params: *mut GLint);
	fn glProgramUniform1i(&self, program: GLuint, location: GLint, v0: GLint);
	fn glProgramUniform1iv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
	fn glProgramUniform1f(&self, program: GLuint, location: GLint, v0: GLfloat);
	fn glProgramUniform1fv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);
	fn glProgramUniform1d(&self, program: GLuint, location: GLint, v0: GLdouble);
	fn glProgramUniform1dv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);
	fn glProgramUniform1ui(&self, program: GLuint, location: GLint, v0: GLuint);
	fn glProgramUniform1uiv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
	fn glProgramUniform2i(&self, program: GLuint, location: GLint, v0: GLint, v1: GLint);
	fn glProgramUniform2iv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
	fn glProgramUniform2f(&self, program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat);
	fn glProgramUniform2fv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);
	fn glProgramUniform2d(&self, program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble);
	fn glProgramUniform2dv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);
	fn glProgramUniform2ui(&self, program: GLuint, location: GLint, v0: GLuint, v1: GLuint);
	fn glProgramUniform2uiv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
	fn glProgramUniform3i(&self, program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint);
	fn glProgramUniform3iv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
	fn glProgramUniform3f(&self, program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);
	fn glProgramUniform3fv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);
	fn glProgramUniform3d(&self, program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble);
	fn glProgramUniform3dv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);
	fn glProgramUniform3ui(&self, program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);
	fn glProgramUniform3uiv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
	fn glProgramUniform4i(&self, program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);
	fn glProgramUniform4iv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
	fn glProgramUniform4f(&self, program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);
	fn glProgramUniform4fv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);
	fn glProgramUniform4d(&self, program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble, v3: GLdouble);
	fn glProgramUniform4dv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);
	fn glProgramUniform4ui(&self, program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);
	fn glProgramUniform4uiv(&self, program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
	fn glProgramUniformMatrix2fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
	fn glProgramUniformMatrix3fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
	fn glProgramUniformMatrix4fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
	fn glProgramUniformMatrix2dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
	fn glProgramUniformMatrix3dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
	fn glProgramUniformMatrix4dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
	fn glProgramUniformMatrix2x3fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
	fn glProgramUniformMatrix3x2fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
	fn glProgramUniformMatrix2x4fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
	fn glProgramUniformMatrix4x2fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
	fn glProgramUniformMatrix3x4fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
	fn glProgramUniformMatrix4x3fv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
	fn glProgramUniformMatrix2x3dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
	fn glProgramUniformMatrix3x2dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
	fn glProgramUniformMatrix2x4dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
	fn glProgramUniformMatrix4x2dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
	fn glProgramUniformMatrix3x4dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
	fn glProgramUniformMatrix4x3dv(&self, program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
	fn glValidateProgramPipeline(&self, pipeline: GLuint);
	fn glGetProgramPipelineInfoLog(&self, pipeline: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);
	fn glVertexAttribL1d(&self, index: GLuint, x: GLdouble);
	fn glVertexAttribL2d(&self, index: GLuint, x: GLdouble, y: GLdouble);
	fn glVertexAttribL3d(&self, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);
	fn glVertexAttribL4d(&self, index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
	fn glVertexAttribL1dv(&self, index: GLuint, v: *const GLdouble);
	fn glVertexAttribL2dv(&self, index: GLuint, v: *const GLdouble);
	fn glVertexAttribL3dv(&self, index: GLuint, v: *const GLdouble);
	fn glVertexAttribL4dv(&self, index: GLuint, v: *const GLdouble);
	fn glVertexAttribLPointer(&self, index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void);
	fn glGetVertexAttribLdv(&self, index: GLuint, pname: GLenum, params: *mut GLdouble);
	fn glViewportArrayv(&self, first: GLuint, count: GLsizei, v: *const GLfloat);
	fn glViewportIndexedf(&self, index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat);
	fn glViewportIndexedfv(&self, index: GLuint, v: *const GLfloat);
	fn glScissorArrayv(&self, first: GLuint, count: GLsizei, v: *const GLint);
	fn glScissorIndexed(&self, index: GLuint, left: GLint, bottom: GLint, width: GLsizei, height: GLsizei);
	fn glScissorIndexedv(&self, index: GLuint, v: *const GLint);
	fn glDepthRangeArrayv(&self, first: GLuint, count: GLsizei, v: *const GLdouble);
	fn glDepthRangeIndexed(&self, index: GLuint, n: GLdouble, f: GLdouble);
	fn glGetFloati_v(&self, target: GLenum, index: GLuint, data: *mut GLfloat);
	fn glGetDoublei_v(&self, target: GLenum, index: GLuint, data: *mut GLdouble);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
	pub fn new(base: impl GL_1_0, mut get_proc_address: impl FnMut(&'static str) -> *const c_void) -> Self {
		let (_spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 40100 {
			return Self::default();
		}
		Self {
			available: true,
			releaseshadercompiler: {let proc = get_proc_address("glReleaseShaderCompiler"); if proc == null() {dummy_pfnglreleaseshadercompilerproc} else {unsafe{transmute(proc)}}},
			shaderbinary: {let proc = get_proc_address("glShaderBinary"); if proc == null() {dummy_pfnglshaderbinaryproc} else {unsafe{transmute(proc)}}},
			getshaderprecisionformat: {let proc = get_proc_address("glGetShaderPrecisionFormat"); if proc == null() {dummy_pfnglgetshaderprecisionformatproc} else {unsafe{transmute(proc)}}},
			depthrangef: {let proc = get_proc_address("glDepthRangef"); if proc == null() {dummy_pfngldepthrangefproc} else {unsafe{transmute(proc)}}},
			cleardepthf: {let proc = get_proc_address("glClearDepthf"); if proc == null() {dummy_pfnglcleardepthfproc} else {unsafe{transmute(proc)}}},
			getprogrambinary: {let proc = get_proc_address("glGetProgramBinary"); if proc == null() {dummy_pfnglgetprogrambinaryproc} else {unsafe{transmute(proc)}}},
			programbinary: {let proc = get_proc_address("glProgramBinary"); if proc == null() {dummy_pfnglprogrambinaryproc} else {unsafe{transmute(proc)}}},
			programparameteri: {let proc = get_proc_address("glProgramParameteri"); if proc == null() {dummy_pfnglprogramparameteriproc} else {unsafe{transmute(proc)}}},
			useprogramstages: {let proc = get_proc_address("glUseProgramStages"); if proc == null() {dummy_pfngluseprogramstagesproc} else {unsafe{transmute(proc)}}},
			activeshaderprogram: {let proc = get_proc_address("glActiveShaderProgram"); if proc == null() {dummy_pfnglactiveshaderprogramproc} else {unsafe{transmute(proc)}}},
			createshaderprogramv: {let proc = get_proc_address("glCreateShaderProgramv"); if proc == null() {dummy_pfnglcreateshaderprogramvproc} else {unsafe{transmute(proc)}}},
			bindprogrampipeline: {let proc = get_proc_address("glBindProgramPipeline"); if proc == null() {dummy_pfnglbindprogrampipelineproc} else {unsafe{transmute(proc)}}},
			deleteprogrampipelines: {let proc = get_proc_address("glDeleteProgramPipelines"); if proc == null() {dummy_pfngldeleteprogrampipelinesproc} else {unsafe{transmute(proc)}}},
			genprogrampipelines: {let proc = get_proc_address("glGenProgramPipelines"); if proc == null() {dummy_pfnglgenprogrampipelinesproc} else {unsafe{transmute(proc)}}},
			isprogrampipeline: {let proc = get_proc_address("glIsProgramPipeline"); if proc == null() {dummy_pfnglisprogrampipelineproc} else {unsafe{transmute(proc)}}},
			getprogrampipelineiv: {let proc = get_proc_address("glGetProgramPipelineiv"); if proc == null() {dummy_pfnglgetprogrampipelineivproc} else {unsafe{transmute(proc)}}},
			programuniform1i: {let proc = get_proc_address("glProgramUniform1i"); if proc == null() {dummy_pfnglprogramuniform1iproc} else {unsafe{transmute(proc)}}},
			programuniform1iv: {let proc = get_proc_address("glProgramUniform1iv"); if proc == null() {dummy_pfnglprogramuniform1ivproc} else {unsafe{transmute(proc)}}},
			programuniform1f: {let proc = get_proc_address("glProgramUniform1f"); if proc == null() {dummy_pfnglprogramuniform1fproc} else {unsafe{transmute(proc)}}},
			programuniform1fv: {let proc = get_proc_address("glProgramUniform1fv"); if proc == null() {dummy_pfnglprogramuniform1fvproc} else {unsafe{transmute(proc)}}},
			programuniform1d: {let proc = get_proc_address("glProgramUniform1d"); if proc == null() {dummy_pfnglprogramuniform1dproc} else {unsafe{transmute(proc)}}},
			programuniform1dv: {let proc = get_proc_address("glProgramUniform1dv"); if proc == null() {dummy_pfnglprogramuniform1dvproc} else {unsafe{transmute(proc)}}},
			programuniform1ui: {let proc = get_proc_address("glProgramUniform1ui"); if proc == null() {dummy_pfnglprogramuniform1uiproc} else {unsafe{transmute(proc)}}},
			programuniform1uiv: {let proc = get_proc_address("glProgramUniform1uiv"); if proc == null() {dummy_pfnglprogramuniform1uivproc} else {unsafe{transmute(proc)}}},
			programuniform2i: {let proc = get_proc_address("glProgramUniform2i"); if proc == null() {dummy_pfnglprogramuniform2iproc} else {unsafe{transmute(proc)}}},
			programuniform2iv: {let proc = get_proc_address("glProgramUniform2iv"); if proc == null() {dummy_pfnglprogramuniform2ivproc} else {unsafe{transmute(proc)}}},
			programuniform2f: {let proc = get_proc_address("glProgramUniform2f"); if proc == null() {dummy_pfnglprogramuniform2fproc} else {unsafe{transmute(proc)}}},
			programuniform2fv: {let proc = get_proc_address("glProgramUniform2fv"); if proc == null() {dummy_pfnglprogramuniform2fvproc} else {unsafe{transmute(proc)}}},
			programuniform2d: {let proc = get_proc_address("glProgramUniform2d"); if proc == null() {dummy_pfnglprogramuniform2dproc} else {unsafe{transmute(proc)}}},
			programuniform2dv: {let proc = get_proc_address("glProgramUniform2dv"); if proc == null() {dummy_pfnglprogramuniform2dvproc} else {unsafe{transmute(proc)}}},
			programuniform2ui: {let proc = get_proc_address("glProgramUniform2ui"); if proc == null() {dummy_pfnglprogramuniform2uiproc} else {unsafe{transmute(proc)}}},
			programuniform2uiv: {let proc = get_proc_address("glProgramUniform2uiv"); if proc == null() {dummy_pfnglprogramuniform2uivproc} else {unsafe{transmute(proc)}}},
			programuniform3i: {let proc = get_proc_address("glProgramUniform3i"); if proc == null() {dummy_pfnglprogramuniform3iproc} else {unsafe{transmute(proc)}}},
			programuniform3iv: {let proc = get_proc_address("glProgramUniform3iv"); if proc == null() {dummy_pfnglprogramuniform3ivproc} else {unsafe{transmute(proc)}}},
			programuniform3f: {let proc = get_proc_address("glProgramUniform3f"); if proc == null() {dummy_pfnglprogramuniform3fproc} else {unsafe{transmute(proc)}}},
			programuniform3fv: {let proc = get_proc_address("glProgramUniform3fv"); if proc == null() {dummy_pfnglprogramuniform3fvproc} else {unsafe{transmute(proc)}}},
			programuniform3d: {let proc = get_proc_address("glProgramUniform3d"); if proc == null() {dummy_pfnglprogramuniform3dproc} else {unsafe{transmute(proc)}}},
			programuniform3dv: {let proc = get_proc_address("glProgramUniform3dv"); if proc == null() {dummy_pfnglprogramuniform3dvproc} else {unsafe{transmute(proc)}}},
			programuniform3ui: {let proc = get_proc_address("glProgramUniform3ui"); if proc == null() {dummy_pfnglprogramuniform3uiproc} else {unsafe{transmute(proc)}}},
			programuniform3uiv: {let proc = get_proc_address("glProgramUniform3uiv"); if proc == null() {dummy_pfnglprogramuniform3uivproc} else {unsafe{transmute(proc)}}},
			programuniform4i: {let proc = get_proc_address("glProgramUniform4i"); if proc == null() {dummy_pfnglprogramuniform4iproc} else {unsafe{transmute(proc)}}},
			programuniform4iv: {let proc = get_proc_address("glProgramUniform4iv"); if proc == null() {dummy_pfnglprogramuniform4ivproc} else {unsafe{transmute(proc)}}},
			programuniform4f: {let proc = get_proc_address("glProgramUniform4f"); if proc == null() {dummy_pfnglprogramuniform4fproc} else {unsafe{transmute(proc)}}},
			programuniform4fv: {let proc = get_proc_address("glProgramUniform4fv"); if proc == null() {dummy_pfnglprogramuniform4fvproc} else {unsafe{transmute(proc)}}},
			programuniform4d: {let proc = get_proc_address("glProgramUniform4d"); if proc == null() {dummy_pfnglprogramuniform4dproc} else {unsafe{transmute(proc)}}},
			programuniform4dv: {let proc = get_proc_address("glProgramUniform4dv"); if proc == null() {dummy_pfnglprogramuniform4dvproc} else {unsafe{transmute(proc)}}},
			programuniform4ui: {let proc = get_proc_address("glProgramUniform4ui"); if proc == null() {dummy_pfnglprogramuniform4uiproc} else {unsafe{transmute(proc)}}},
			programuniform4uiv: {let proc = get_proc_address("glProgramUniform4uiv"); if proc == null() {dummy_pfnglprogramuniform4uivproc} else {unsafe{transmute(proc)}}},
			programuniformmatrix2fv: {let proc = get_proc_address("glProgramUniformMatrix2fv"); if proc == null() {dummy_pfnglprogramuniformmatrix2fvproc} else {unsafe{transmute(proc)}}},
			programuniformmatrix3fv: {let proc = get_proc_address("glProgramUniformMatrix3fv"); if proc == null() {dummy_pfnglprogramuniformmatrix3fvproc} else {unsafe{transmute(proc)}}},
			programuniformmatrix4fv: {let proc = get_proc_address("glProgramUniformMatrix4fv"); if proc == null() {dummy_pfnglprogramuniformmatrix4fvproc} else {unsafe{transmute(proc)}}},
			programuniformmatrix2dv: {let proc = get_proc_address("glProgramUniformMatrix2dv"); if proc == null() {dummy_pfnglprogramuniformmatrix2dvproc} else {unsafe{transmute(proc)}}},
			programuniformmatrix3dv: {let proc = get_proc_address("glProgramUniformMatrix3dv"); if proc == null() {dummy_pfnglprogramuniformmatrix3dvproc} else {unsafe{transmute(proc)}}},
			programuniformmatrix4dv: {let proc = get_proc_address("glProgramUniformMatrix4dv"); if proc == null() {dummy_pfnglprogramuniformmatrix4dvproc} else {unsafe{transmute(proc)}}},
			programuniformmatrix2x3fv: {let proc = get_proc_address("glProgramUniformMatrix2x3fv"); if proc == null() {dummy_pfnglprogramuniformmatrix2x3fvproc} else {unsafe{transmute(proc)}}},
			programuniformmatrix3x2fv: {let proc = get_proc_address("glProgramUniformMatrix3x2fv"); if proc == null() {dummy_pfnglprogramuniformmatrix3x2fvproc} else {unsafe{transmute(proc)}}},
			programuniformmatrix2x4fv: {let proc = get_proc_address("glProgramUniformMatrix2x4fv"); if proc == null() {dummy_pfnglprogramuniformmatrix2x4fvproc} else {unsafe{transmute(proc)}}},
			programuniformmatrix4x2fv: {let proc = get_proc_address("glProgramUniformMatrix4x2fv"); if proc == null() {dummy_pfnglprogramuniformmatrix4x2fvproc} else {unsafe{transmute(proc)}}},
			programuniformmatrix3x4fv: {let proc = get_proc_address("glProgramUniformMatrix3x4fv"); if proc == null() {dummy_pfnglprogramuniformmatrix3x4fvproc} else {unsafe{transmute(proc)}}},
			programuniformmatrix4x3fv: {let proc = get_proc_address("glProgramUniformMatrix4x3fv"); if proc == null() {dummy_pfnglprogramuniformmatrix4x3fvproc} else {unsafe{transmute(proc)}}},
			programuniformmatrix2x3dv: {let proc = get_proc_address("glProgramUniformMatrix2x3dv"); if proc == null() {dummy_pfnglprogramuniformmatrix2x3dvproc} else {unsafe{transmute(proc)}}},
			programuniformmatrix3x2dv: {let proc = get_proc_address("glProgramUniformMatrix3x2dv"); if proc == null() {dummy_pfnglprogramuniformmatrix3x2dvproc} else {unsafe{transmute(proc)}}},
			programuniformmatrix2x4dv: {let proc = get_proc_address("glProgramUniformMatrix2x4dv"); if proc == null() {dummy_pfnglprogramuniformmatrix2x4dvproc} else {unsafe{transmute(proc)}}},
			programuniformmatrix4x2dv: {let proc = get_proc_address("glProgramUniformMatrix4x2dv"); if proc == null() {dummy_pfnglprogramuniformmatrix4x2dvproc} else {unsafe{transmute(proc)}}},
			programuniformmatrix3x4dv: {let proc = get_proc_address("glProgramUniformMatrix3x4dv"); if proc == null() {dummy_pfnglprogramuniformmatrix3x4dvproc} else {unsafe{transmute(proc)}}},
			programuniformmatrix4x3dv: {let proc = get_proc_address("glProgramUniformMatrix4x3dv"); if proc == null() {dummy_pfnglprogramuniformmatrix4x3dvproc} else {unsafe{transmute(proc)}}},
			validateprogrampipeline: {let proc = get_proc_address("glValidateProgramPipeline"); if proc == null() {dummy_pfnglvalidateprogrampipelineproc} else {unsafe{transmute(proc)}}},
			getprogrampipelineinfolog: {let proc = get_proc_address("glGetProgramPipelineInfoLog"); if proc == null() {dummy_pfnglgetprogrampipelineinfologproc} else {unsafe{transmute(proc)}}},
			vertexattribl1d: {let proc = get_proc_address("glVertexAttribL1d"); if proc == null() {dummy_pfnglvertexattribl1dproc} else {unsafe{transmute(proc)}}},
			vertexattribl2d: {let proc = get_proc_address("glVertexAttribL2d"); if proc == null() {dummy_pfnglvertexattribl2dproc} else {unsafe{transmute(proc)}}},
			vertexattribl3d: {let proc = get_proc_address("glVertexAttribL3d"); if proc == null() {dummy_pfnglvertexattribl3dproc} else {unsafe{transmute(proc)}}},
			vertexattribl4d: {let proc = get_proc_address("glVertexAttribL4d"); if proc == null() {dummy_pfnglvertexattribl4dproc} else {unsafe{transmute(proc)}}},
			vertexattribl1dv: {let proc = get_proc_address("glVertexAttribL1dv"); if proc == null() {dummy_pfnglvertexattribl1dvproc} else {unsafe{transmute(proc)}}},
			vertexattribl2dv: {let proc = get_proc_address("glVertexAttribL2dv"); if proc == null() {dummy_pfnglvertexattribl2dvproc} else {unsafe{transmute(proc)}}},
			vertexattribl3dv: {let proc = get_proc_address("glVertexAttribL3dv"); if proc == null() {dummy_pfnglvertexattribl3dvproc} else {unsafe{transmute(proc)}}},
			vertexattribl4dv: {let proc = get_proc_address("glVertexAttribL4dv"); if proc == null() {dummy_pfnglvertexattribl4dvproc} else {unsafe{transmute(proc)}}},
			vertexattriblpointer: {let proc = get_proc_address("glVertexAttribLPointer"); if proc == null() {dummy_pfnglvertexattriblpointerproc} else {unsafe{transmute(proc)}}},
			getvertexattribldv: {let proc = get_proc_address("glGetVertexAttribLdv"); if proc == null() {dummy_pfnglgetvertexattribldvproc} else {unsafe{transmute(proc)}}},
			viewportarrayv: {let proc = get_proc_address("glViewportArrayv"); if proc == null() {dummy_pfnglviewportarrayvproc} else {unsafe{transmute(proc)}}},
			viewportindexedf: {let proc = get_proc_address("glViewportIndexedf"); if proc == null() {dummy_pfnglviewportindexedfproc} else {unsafe{transmute(proc)}}},
			viewportindexedfv: {let proc = get_proc_address("glViewportIndexedfv"); if proc == null() {dummy_pfnglviewportindexedfvproc} else {unsafe{transmute(proc)}}},
			scissorarrayv: {let proc = get_proc_address("glScissorArrayv"); if proc == null() {dummy_pfnglscissorarrayvproc} else {unsafe{transmute(proc)}}},
			scissorindexed: {let proc = get_proc_address("glScissorIndexed"); if proc == null() {dummy_pfnglscissorindexedproc} else {unsafe{transmute(proc)}}},
			scissorindexedv: {let proc = get_proc_address("glScissorIndexedv"); if proc == null() {dummy_pfnglscissorindexedvproc} else {unsafe{transmute(proc)}}},
			depthrangearrayv: {let proc = get_proc_address("glDepthRangeArrayv"); if proc == null() {dummy_pfngldepthrangearrayvproc} else {unsafe{transmute(proc)}}},
			depthrangeindexed: {let proc = get_proc_address("glDepthRangeIndexed"); if proc == null() {dummy_pfngldepthrangeindexedproc} else {unsafe{transmute(proc)}}},
			getfloati_v: {let proc = get_proc_address("glGetFloati_v"); if proc == null() {dummy_pfnglgetfloati_vproc} else {unsafe{transmute(proc)}}},
			getdoublei_v: {let proc = get_proc_address("glGetDoublei_v"); if proc == null() {dummy_pfnglgetdoublei_vproc} else {unsafe{transmute(proc)}}},
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
			releaseshadercompiler: dummy_pfnglreleaseshadercompilerproc,
			shaderbinary: dummy_pfnglshaderbinaryproc,
			getshaderprecisionformat: dummy_pfnglgetshaderprecisionformatproc,
			depthrangef: dummy_pfngldepthrangefproc,
			cleardepthf: dummy_pfnglcleardepthfproc,
			getprogrambinary: dummy_pfnglgetprogrambinaryproc,
			programbinary: dummy_pfnglprogrambinaryproc,
			programparameteri: dummy_pfnglprogramparameteriproc,
			useprogramstages: dummy_pfngluseprogramstagesproc,
			activeshaderprogram: dummy_pfnglactiveshaderprogramproc,
			createshaderprogramv: dummy_pfnglcreateshaderprogramvproc,
			bindprogrampipeline: dummy_pfnglbindprogrampipelineproc,
			deleteprogrampipelines: dummy_pfngldeleteprogrampipelinesproc,
			genprogrampipelines: dummy_pfnglgenprogrampipelinesproc,
			isprogrampipeline: dummy_pfnglisprogrampipelineproc,
			getprogrampipelineiv: dummy_pfnglgetprogrampipelineivproc,
			programuniform1i: dummy_pfnglprogramuniform1iproc,
			programuniform1iv: dummy_pfnglprogramuniform1ivproc,
			programuniform1f: dummy_pfnglprogramuniform1fproc,
			programuniform1fv: dummy_pfnglprogramuniform1fvproc,
			programuniform1d: dummy_pfnglprogramuniform1dproc,
			programuniform1dv: dummy_pfnglprogramuniform1dvproc,
			programuniform1ui: dummy_pfnglprogramuniform1uiproc,
			programuniform1uiv: dummy_pfnglprogramuniform1uivproc,
			programuniform2i: dummy_pfnglprogramuniform2iproc,
			programuniform2iv: dummy_pfnglprogramuniform2ivproc,
			programuniform2f: dummy_pfnglprogramuniform2fproc,
			programuniform2fv: dummy_pfnglprogramuniform2fvproc,
			programuniform2d: dummy_pfnglprogramuniform2dproc,
			programuniform2dv: dummy_pfnglprogramuniform2dvproc,
			programuniform2ui: dummy_pfnglprogramuniform2uiproc,
			programuniform2uiv: dummy_pfnglprogramuniform2uivproc,
			programuniform3i: dummy_pfnglprogramuniform3iproc,
			programuniform3iv: dummy_pfnglprogramuniform3ivproc,
			programuniform3f: dummy_pfnglprogramuniform3fproc,
			programuniform3fv: dummy_pfnglprogramuniform3fvproc,
			programuniform3d: dummy_pfnglprogramuniform3dproc,
			programuniform3dv: dummy_pfnglprogramuniform3dvproc,
			programuniform3ui: dummy_pfnglprogramuniform3uiproc,
			programuniform3uiv: dummy_pfnglprogramuniform3uivproc,
			programuniform4i: dummy_pfnglprogramuniform4iproc,
			programuniform4iv: dummy_pfnglprogramuniform4ivproc,
			programuniform4f: dummy_pfnglprogramuniform4fproc,
			programuniform4fv: dummy_pfnglprogramuniform4fvproc,
			programuniform4d: dummy_pfnglprogramuniform4dproc,
			programuniform4dv: dummy_pfnglprogramuniform4dvproc,
			programuniform4ui: dummy_pfnglprogramuniform4uiproc,
			programuniform4uiv: dummy_pfnglprogramuniform4uivproc,
			programuniformmatrix2fv: dummy_pfnglprogramuniformmatrix2fvproc,
			programuniformmatrix3fv: dummy_pfnglprogramuniformmatrix3fvproc,
			programuniformmatrix4fv: dummy_pfnglprogramuniformmatrix4fvproc,
			programuniformmatrix2dv: dummy_pfnglprogramuniformmatrix2dvproc,
			programuniformmatrix3dv: dummy_pfnglprogramuniformmatrix3dvproc,
			programuniformmatrix4dv: dummy_pfnglprogramuniformmatrix4dvproc,
			programuniformmatrix2x3fv: dummy_pfnglprogramuniformmatrix2x3fvproc,
			programuniformmatrix3x2fv: dummy_pfnglprogramuniformmatrix3x2fvproc,
			programuniformmatrix2x4fv: dummy_pfnglprogramuniformmatrix2x4fvproc,
			programuniformmatrix4x2fv: dummy_pfnglprogramuniformmatrix4x2fvproc,
			programuniformmatrix3x4fv: dummy_pfnglprogramuniformmatrix3x4fvproc,
			programuniformmatrix4x3fv: dummy_pfnglprogramuniformmatrix4x3fvproc,
			programuniformmatrix2x3dv: dummy_pfnglprogramuniformmatrix2x3dvproc,
			programuniformmatrix3x2dv: dummy_pfnglprogramuniformmatrix3x2dvproc,
			programuniformmatrix2x4dv: dummy_pfnglprogramuniformmatrix2x4dvproc,
			programuniformmatrix4x2dv: dummy_pfnglprogramuniformmatrix4x2dvproc,
			programuniformmatrix3x4dv: dummy_pfnglprogramuniformmatrix3x4dvproc,
			programuniformmatrix4x3dv: dummy_pfnglprogramuniformmatrix4x3dvproc,
			validateprogrampipeline: dummy_pfnglvalidateprogrampipelineproc,
			getprogrampipelineinfolog: dummy_pfnglgetprogrampipelineinfologproc,
			vertexattribl1d: dummy_pfnglvertexattribl1dproc,
			vertexattribl2d: dummy_pfnglvertexattribl2dproc,
			vertexattribl3d: dummy_pfnglvertexattribl3dproc,
			vertexattribl4d: dummy_pfnglvertexattribl4dproc,
			vertexattribl1dv: dummy_pfnglvertexattribl1dvproc,
			vertexattribl2dv: dummy_pfnglvertexattribl2dvproc,
			vertexattribl3dv: dummy_pfnglvertexattribl3dvproc,
			vertexattribl4dv: dummy_pfnglvertexattribl4dvproc,
			vertexattriblpointer: dummy_pfnglvertexattriblpointerproc,
			getvertexattribldv: dummy_pfnglgetvertexattribldvproc,
			viewportarrayv: dummy_pfnglviewportarrayvproc,
			viewportindexedf: dummy_pfnglviewportindexedfproc,
			viewportindexedfv: dummy_pfnglviewportindexedfvproc,
			scissorarrayv: dummy_pfnglscissorarrayvproc,
			scissorindexed: dummy_pfnglscissorindexedproc,
			scissorindexedv: dummy_pfnglscissorindexedvproc,
			depthrangearrayv: dummy_pfngldepthrangearrayvproc,
			depthrangeindexed: dummy_pfngldepthrangeindexedproc,
			getfloati_v: dummy_pfnglgetfloati_vproc,
			getdoublei_v: dummy_pfnglgetdoublei_vproc,
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
extern "system" fn dummy_pfngldrawarraysinstancedbaseinstanceproc (_: GLenum, _: GLint, _: GLsizei, _: GLsizei, _: GLuint) {
	panic!("OpenGL Function pointer of `glDrawArraysInstancedBaseInstance()` is NULL");
}
extern "system" fn dummy_pfngldrawelementsinstancedbaseinstanceproc (_: GLenum, _: GLsizei, _: GLenum, _: *const c_void, _: GLsizei, _: GLuint) {
	panic!("OpenGL Function pointer of `glDrawElementsInstancedBaseInstance()` is NULL");
}
extern "system" fn dummy_pfngldrawelementsinstancedbasevertexbaseinstanceproc (_: GLenum, _: GLsizei, _: GLenum, _: *const c_void, _: GLsizei, _: GLint, _: GLuint) {
	panic!("OpenGL Function pointer of `glDrawElementsInstancedBaseVertexBaseInstance()` is NULL");
}
extern "system" fn dummy_pfnglgetinternalformativproc (_: GLenum, _: GLenum, _: GLenum, _: GLsizei, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetInternalformativ()` is NULL");
}
extern "system" fn dummy_pfnglgetactiveatomiccounterbufferivproc (_: GLuint, _: GLuint, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetActiveAtomicCounterBufferiv()` is NULL");
}
extern "system" fn dummy_pfnglbindimagetextureproc (_: GLuint, _: GLuint, _: GLint, _: GLboolean, _: GLint, _: GLenum, _: GLenum) {
	panic!("OpenGL Function pointer of `glBindImageTexture()` is NULL");
}
extern "system" fn dummy_pfnglmemorybarrierproc (_: GLbitfield) {
	panic!("OpenGL Function pointer of `glMemoryBarrier()` is NULL");
}
extern "system" fn dummy_pfngltexstorage1dproc (_: GLenum, _: GLsizei, _: GLenum, _: GLsizei) {
	panic!("OpenGL Function pointer of `glTexStorage1D()` is NULL");
}
extern "system" fn dummy_pfngltexstorage2dproc (_: GLenum, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei) {
	panic!("OpenGL Function pointer of `glTexStorage2D()` is NULL");
}
extern "system" fn dummy_pfngltexstorage3dproc (_: GLenum, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei, _: GLsizei) {
	panic!("OpenGL Function pointer of `glTexStorage3D()` is NULL");
}
extern "system" fn dummy_pfngldrawtransformfeedbackinstancedproc (_: GLenum, _: GLuint, _: GLsizei) {
	panic!("OpenGL Function pointer of `glDrawTransformFeedbackInstanced()` is NULL");
}
extern "system" fn dummy_pfngldrawtransformfeedbackstreaminstancedproc (_: GLenum, _: GLuint, _: GLuint, _: GLsizei) {
	panic!("OpenGL Function pointer of `glDrawTransformFeedbackStreamInstanced()` is NULL");
}
pub const GL_COPY_READ_BUFFER_BINDING: GLenum = 0x8F36;
pub const GL_COPY_WRITE_BUFFER_BINDING: GLenum = 0x8F37;
pub const GL_TRANSFORM_FEEDBACK_ACTIVE: GLenum = 0x8E24;
pub const GL_TRANSFORM_FEEDBACK_PAUSED: GLenum = 0x8E23;
pub const GL_UNPACK_COMPRESSED_BLOCK_WIDTH: GLenum = 0x9127;
pub const GL_UNPACK_COMPRESSED_BLOCK_HEIGHT: GLenum = 0x9128;
pub const GL_UNPACK_COMPRESSED_BLOCK_DEPTH: GLenum = 0x9129;
pub const GL_UNPACK_COMPRESSED_BLOCK_SIZE: GLenum = 0x912A;
pub const GL_PACK_COMPRESSED_BLOCK_WIDTH: GLenum = 0x912B;
pub const GL_PACK_COMPRESSED_BLOCK_HEIGHT: GLenum = 0x912C;
pub const GL_PACK_COMPRESSED_BLOCK_DEPTH: GLenum = 0x912D;
pub const GL_PACK_COMPRESSED_BLOCK_SIZE: GLenum = 0x912E;
pub const GL_NUM_SAMPLE_COUNTS: GLenum = 0x9380;
pub const GL_MIN_MAP_BUFFER_ALIGNMENT: GLenum = 0x90BC;
pub const GL_ATOMIC_COUNTER_BUFFER: GLenum = 0x92C0;
pub const GL_ATOMIC_COUNTER_BUFFER_BINDING: GLenum = 0x92C1;
pub const GL_ATOMIC_COUNTER_BUFFER_START: GLenum = 0x92C2;
pub const GL_ATOMIC_COUNTER_BUFFER_SIZE: GLenum = 0x92C3;
pub const GL_ATOMIC_COUNTER_BUFFER_DATA_SIZE: GLenum = 0x92C4;
pub const GL_ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS: GLenum = 0x92C5;
pub const GL_ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES: GLenum = 0x92C6;
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x92C7;
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x92C8;
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x92C9;
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x92CA;
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x92CB;
pub const GL_MAX_VERTEX_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CC;
pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CD;
pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CE;
pub const GL_MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CF;
pub const GL_MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D0;
pub const GL_MAX_COMBINED_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D1;
pub const GL_MAX_VERTEX_ATOMIC_COUNTERS: GLenum = 0x92D2;
pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS: GLenum = 0x92D3;
pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS: GLenum = 0x92D4;
pub const GL_MAX_GEOMETRY_ATOMIC_COUNTERS: GLenum = 0x92D5;
pub const GL_MAX_FRAGMENT_ATOMIC_COUNTERS: GLenum = 0x92D6;
pub const GL_MAX_COMBINED_ATOMIC_COUNTERS: GLenum = 0x92D7;
pub const GL_MAX_ATOMIC_COUNTER_BUFFER_SIZE: GLenum = 0x92D8;
pub const GL_MAX_ATOMIC_COUNTER_BUFFER_BINDINGS: GLenum = 0x92DC;
pub const GL_ACTIVE_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D9;
pub const GL_UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX: GLenum = 0x92DA;
pub const GL_UNSIGNED_INT_ATOMIC_COUNTER: GLenum = 0x92DB;
pub const GL_VERTEX_ATTRIB_ARRAY_BARRIER_BIT: GLbitfield = 0x00000001;
pub const GL_ELEMENT_ARRAY_BARRIER_BIT: GLbitfield = 0x00000002;
pub const GL_UNIFORM_BARRIER_BIT: GLbitfield = 0x00000004;
pub const GL_TEXTURE_FETCH_BARRIER_BIT: GLbitfield = 0x00000008;
pub const GL_SHADER_IMAGE_ACCESS_BARRIER_BIT: GLbitfield = 0x00000020;
pub const GL_COMMAND_BARRIER_BIT: GLbitfield = 0x00000040;
pub const GL_PIXEL_BUFFER_BARRIER_BIT: GLbitfield = 0x00000080;
pub const GL_TEXTURE_UPDATE_BARRIER_BIT: GLbitfield = 0x00000100;
pub const GL_BUFFER_UPDATE_BARRIER_BIT: GLbitfield = 0x00000200;
pub const GL_FRAMEBUFFER_BARRIER_BIT: GLbitfield = 0x00000400;
pub const GL_TRANSFORM_FEEDBACK_BARRIER_BIT: GLbitfield = 0x00000800;
pub const GL_ATOMIC_COUNTER_BARRIER_BIT: GLbitfield = 0x00001000;
pub const GL_ALL_BARRIER_BITS: GLbitfield = 0xFFFFFFFF;
pub const GL_MAX_IMAGE_UNITS: GLenum = 0x8F38;
pub const GL_MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS: GLenum = 0x8F39;
pub const GL_IMAGE_BINDING_NAME: GLenum = 0x8F3A;
pub const GL_IMAGE_BINDING_LEVEL: GLenum = 0x8F3B;
pub const GL_IMAGE_BINDING_LAYERED: GLenum = 0x8F3C;
pub const GL_IMAGE_BINDING_LAYER: GLenum = 0x8F3D;
pub const GL_IMAGE_BINDING_ACCESS: GLenum = 0x8F3E;
pub const GL_IMAGE_1D: GLenum = 0x904C;
pub const GL_IMAGE_2D: GLenum = 0x904D;
pub const GL_IMAGE_3D: GLenum = 0x904E;
pub const GL_IMAGE_2D_RECT: GLenum = 0x904F;
pub const GL_IMAGE_CUBE: GLenum = 0x9050;
pub const GL_IMAGE_BUFFER: GLenum = 0x9051;
pub const GL_IMAGE_1D_ARRAY: GLenum = 0x9052;
pub const GL_IMAGE_2D_ARRAY: GLenum = 0x9053;
pub const GL_IMAGE_CUBE_MAP_ARRAY: GLenum = 0x9054;
pub const GL_IMAGE_2D_MULTISAMPLE: GLenum = 0x9055;
pub const GL_IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9056;
pub const GL_INT_IMAGE_1D: GLenum = 0x9057;
pub const GL_INT_IMAGE_2D: GLenum = 0x9058;
pub const GL_INT_IMAGE_3D: GLenum = 0x9059;
pub const GL_INT_IMAGE_2D_RECT: GLenum = 0x905A;
pub const GL_INT_IMAGE_CUBE: GLenum = 0x905B;
pub const GL_INT_IMAGE_BUFFER: GLenum = 0x905C;
pub const GL_INT_IMAGE_1D_ARRAY: GLenum = 0x905D;
pub const GL_INT_IMAGE_2D_ARRAY: GLenum = 0x905E;
pub const GL_INT_IMAGE_CUBE_MAP_ARRAY: GLenum = 0x905F;
pub const GL_INT_IMAGE_2D_MULTISAMPLE: GLenum = 0x9060;
pub const GL_INT_IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9061;
pub const GL_UNSIGNED_INT_IMAGE_1D: GLenum = 0x9062;
pub const GL_UNSIGNED_INT_IMAGE_2D: GLenum = 0x9063;
pub const GL_UNSIGNED_INT_IMAGE_3D: GLenum = 0x9064;
pub const GL_UNSIGNED_INT_IMAGE_2D_RECT: GLenum = 0x9065;
pub const GL_UNSIGNED_INT_IMAGE_CUBE: GLenum = 0x9066;
pub const GL_UNSIGNED_INT_IMAGE_BUFFER: GLenum = 0x9067;
pub const GL_UNSIGNED_INT_IMAGE_1D_ARRAY: GLenum = 0x9068;
pub const GL_UNSIGNED_INT_IMAGE_2D_ARRAY: GLenum = 0x9069;
pub const GL_UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY: GLenum = 0x906A;
pub const GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE: GLenum = 0x906B;
pub const GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 0x906C;
pub const GL_MAX_IMAGE_SAMPLES: GLenum = 0x906D;
pub const GL_IMAGE_BINDING_FORMAT: GLenum = 0x906E;
pub const GL_IMAGE_FORMAT_COMPATIBILITY_TYPE: GLenum = 0x90C7;
pub const GL_IMAGE_FORMAT_COMPATIBILITY_BY_SIZE: GLenum = 0x90C8;
pub const GL_IMAGE_FORMAT_COMPATIBILITY_BY_CLASS: GLenum = 0x90C9;
pub const GL_MAX_VERTEX_IMAGE_UNIFORMS: GLenum = 0x90CA;
pub const GL_MAX_TESS_CONTROL_IMAGE_UNIFORMS: GLenum = 0x90CB;
pub const GL_MAX_TESS_EVALUATION_IMAGE_UNIFORMS: GLenum = 0x90CC;
pub const GL_MAX_GEOMETRY_IMAGE_UNIFORMS: GLenum = 0x90CD;
pub const GL_MAX_FRAGMENT_IMAGE_UNIFORMS: GLenum = 0x90CE;
pub const GL_MAX_COMBINED_IMAGE_UNIFORMS: GLenum = 0x90CF;
pub const GL_COMPRESSED_RGBA_BPTC_UNORM: GLenum = 0x8E8C;
pub const GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM: GLenum = 0x8E8D;
pub const GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT: GLenum = 0x8E8E;
pub const GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT: GLenum = 0x8E8F;
pub const GL_TEXTURE_IMMUTABLE_FORMAT: GLenum = 0x912F;

pub trait GL_4_2 {
	fn glDrawArraysInstancedBaseInstance(&self, mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei, baseinstance: GLuint);
	fn glDrawElementsInstancedBaseInstance(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei, baseinstance: GLuint);
	fn glDrawElementsInstancedBaseVertexBaseInstance(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei, basevertex: GLint, baseinstance: GLuint);
	fn glGetInternalformativ(&self, target: GLenum, internalformat: GLenum, pname: GLenum, count: GLsizei, params: *mut GLint);
	fn glGetActiveAtomicCounterBufferiv(&self, program: GLuint, bufferIndex: GLuint, pname: GLenum, params: *mut GLint);
	fn glBindImageTexture(&self, unit: GLuint, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, access: GLenum, format: GLenum);
	fn glMemoryBarrier(&self, barriers: GLbitfield);
	fn glTexStorage1D(&self, target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei);
	fn glTexStorage2D(&self, target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
	fn glTexStorage3D(&self, target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei);
	fn glDrawTransformFeedbackInstanced(&self, mode: GLenum, id: GLuint, instancecount: GLsizei);
	fn glDrawTransformFeedbackStreamInstanced(&self, mode: GLenum, id: GLuint, stream: GLuint, instancecount: GLsizei);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
	pub fn new(base: impl GL_1_0, mut get_proc_address: impl FnMut(&'static str) -> *const c_void) -> Self {
		let (_spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 40200 {
			return Self::default();
		}
		Self {
			available: true,
			drawarraysinstancedbaseinstance: {let proc = get_proc_address("glDrawArraysInstancedBaseInstance"); if proc == null() {dummy_pfngldrawarraysinstancedbaseinstanceproc} else {unsafe{transmute(proc)}}},
			drawelementsinstancedbaseinstance: {let proc = get_proc_address("glDrawElementsInstancedBaseInstance"); if proc == null() {dummy_pfngldrawelementsinstancedbaseinstanceproc} else {unsafe{transmute(proc)}}},
			drawelementsinstancedbasevertexbaseinstance: {let proc = get_proc_address("glDrawElementsInstancedBaseVertexBaseInstance"); if proc == null() {dummy_pfngldrawelementsinstancedbasevertexbaseinstanceproc} else {unsafe{transmute(proc)}}},
			getinternalformativ: {let proc = get_proc_address("glGetInternalformativ"); if proc == null() {dummy_pfnglgetinternalformativproc} else {unsafe{transmute(proc)}}},
			getactiveatomiccounterbufferiv: {let proc = get_proc_address("glGetActiveAtomicCounterBufferiv"); if proc == null() {dummy_pfnglgetactiveatomiccounterbufferivproc} else {unsafe{transmute(proc)}}},
			bindimagetexture: {let proc = get_proc_address("glBindImageTexture"); if proc == null() {dummy_pfnglbindimagetextureproc} else {unsafe{transmute(proc)}}},
			memorybarrier: {let proc = get_proc_address("glMemoryBarrier"); if proc == null() {dummy_pfnglmemorybarrierproc} else {unsafe{transmute(proc)}}},
			texstorage1d: {let proc = get_proc_address("glTexStorage1D"); if proc == null() {dummy_pfngltexstorage1dproc} else {unsafe{transmute(proc)}}},
			texstorage2d: {let proc = get_proc_address("glTexStorage2D"); if proc == null() {dummy_pfngltexstorage2dproc} else {unsafe{transmute(proc)}}},
			texstorage3d: {let proc = get_proc_address("glTexStorage3D"); if proc == null() {dummy_pfngltexstorage3dproc} else {unsafe{transmute(proc)}}},
			drawtransformfeedbackinstanced: {let proc = get_proc_address("glDrawTransformFeedbackInstanced"); if proc == null() {dummy_pfngldrawtransformfeedbackinstancedproc} else {unsafe{transmute(proc)}}},
			drawtransformfeedbackstreaminstanced: {let proc = get_proc_address("glDrawTransformFeedbackStreamInstanced"); if proc == null() {dummy_pfngldrawtransformfeedbackstreaminstancedproc} else {unsafe{transmute(proc)}}},
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
			drawarraysinstancedbaseinstance: dummy_pfngldrawarraysinstancedbaseinstanceproc,
			drawelementsinstancedbaseinstance: dummy_pfngldrawelementsinstancedbaseinstanceproc,
			drawelementsinstancedbasevertexbaseinstance: dummy_pfngldrawelementsinstancedbasevertexbaseinstanceproc,
			getinternalformativ: dummy_pfnglgetinternalformativproc,
			getactiveatomiccounterbufferiv: dummy_pfnglgetactiveatomiccounterbufferivproc,
			bindimagetexture: dummy_pfnglbindimagetextureproc,
			memorybarrier: dummy_pfnglmemorybarrierproc,
			texstorage1d: dummy_pfngltexstorage1dproc,
			texstorage2d: dummy_pfngltexstorage2dproc,
			texstorage3d: dummy_pfngltexstorage3dproc,
			drawtransformfeedbackinstanced: dummy_pfngldrawtransformfeedbackinstancedproc,
			drawtransformfeedbackstreaminstanced: dummy_pfngldrawtransformfeedbackstreaminstancedproc,
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
extern "system" fn dummy_pfnglclearbufferdataproc (_: GLenum, _: GLenum, _: GLenum, _: GLenum, _: *const c_void) {
	panic!("OpenGL Function pointer of `glClearBufferData()` is NULL");
}
extern "system" fn dummy_pfnglclearbuffersubdataproc (_: GLenum, _: GLenum, _: GLintptr, _: GLsizeiptr, _: GLenum, _: GLenum, _: *const c_void) {
	panic!("OpenGL Function pointer of `glClearBufferSubData()` is NULL");
}
extern "system" fn dummy_pfngldispatchcomputeproc (_: GLuint, _: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glDispatchCompute()` is NULL");
}
extern "system" fn dummy_pfngldispatchcomputeindirectproc (_: GLintptr) {
	panic!("OpenGL Function pointer of `glDispatchComputeIndirect()` is NULL");
}
extern "system" fn dummy_pfnglcopyimagesubdataproc (_: GLuint, _: GLenum, _: GLint, _: GLint, _: GLint, _: GLint, _: GLuint, _: GLenum, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLsizei) {
	panic!("OpenGL Function pointer of `glCopyImageSubData()` is NULL");
}
extern "system" fn dummy_pfnglframebufferparameteriproc (_: GLenum, _: GLenum, _: GLint) {
	panic!("OpenGL Function pointer of `glFramebufferParameteri()` is NULL");
}
extern "system" fn dummy_pfnglgetframebufferparameterivproc (_: GLenum, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetFramebufferParameteriv()` is NULL");
}
extern "system" fn dummy_pfnglgetinternalformati64vproc (_: GLenum, _: GLenum, _: GLenum, _: GLsizei, _: *mut GLint64) {
	panic!("OpenGL Function pointer of `glGetInternalformati64v()` is NULL");
}
extern "system" fn dummy_pfnglinvalidatetexsubimageproc (_: GLuint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLsizei) {
	panic!("OpenGL Function pointer of `glInvalidateTexSubImage()` is NULL");
}
extern "system" fn dummy_pfnglinvalidateteximageproc (_: GLuint, _: GLint) {
	panic!("OpenGL Function pointer of `glInvalidateTexImage()` is NULL");
}
extern "system" fn dummy_pfnglinvalidatebuffersubdataproc (_: GLuint, _: GLintptr, _: GLsizeiptr) {
	panic!("OpenGL Function pointer of `glInvalidateBufferSubData()` is NULL");
}
extern "system" fn dummy_pfnglinvalidatebufferdataproc (_: GLuint) {
	panic!("OpenGL Function pointer of `glInvalidateBufferData()` is NULL");
}
extern "system" fn dummy_pfnglinvalidateframebufferproc (_: GLenum, _: GLsizei, _: *const GLenum) {
	panic!("OpenGL Function pointer of `glInvalidateFramebuffer()` is NULL");
}
extern "system" fn dummy_pfnglinvalidatesubframebufferproc (_: GLenum, _: GLsizei, _: *const GLenum, _: GLint, _: GLint, _: GLsizei, _: GLsizei) {
	panic!("OpenGL Function pointer of `glInvalidateSubFramebuffer()` is NULL");
}
extern "system" fn dummy_pfnglmultidrawarraysindirectproc (_: GLenum, _: *const c_void, _: GLsizei, _: GLsizei) {
	panic!("OpenGL Function pointer of `glMultiDrawArraysIndirect()` is NULL");
}
extern "system" fn dummy_pfnglmultidrawelementsindirectproc (_: GLenum, _: GLenum, _: *const c_void, _: GLsizei, _: GLsizei) {
	panic!("OpenGL Function pointer of `glMultiDrawElementsIndirect()` is NULL");
}
extern "system" fn dummy_pfnglgetprograminterfaceivproc (_: GLuint, _: GLenum, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetProgramInterfaceiv()` is NULL");
}
extern "system" fn dummy_pfnglgetprogramresourceindexproc (_: GLuint, _: GLenum, _: *const GLchar) -> GLuint {
	panic!("OpenGL Function pointer of `glGetProgramResourceIndex()` is NULL");
}
extern "system" fn dummy_pfnglgetprogramresourcenameproc (_: GLuint, _: GLenum, _: GLuint, _: GLsizei, _: *mut GLsizei, _: *mut GLchar) {
	panic!("OpenGL Function pointer of `glGetProgramResourceName()` is NULL");
}
extern "system" fn dummy_pfnglgetprogramresourceivproc (_: GLuint, _: GLenum, _: GLuint, _: GLsizei, _: *const GLenum, _: GLsizei, _: *mut GLsizei, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetProgramResourceiv()` is NULL");
}
extern "system" fn dummy_pfnglgetprogramresourcelocationproc (_: GLuint, _: GLenum, _: *const GLchar) -> GLint {
	panic!("OpenGL Function pointer of `glGetProgramResourceLocation()` is NULL");
}
extern "system" fn dummy_pfnglgetprogramresourcelocationindexproc (_: GLuint, _: GLenum, _: *const GLchar) -> GLint {
	panic!("OpenGL Function pointer of `glGetProgramResourceLocationIndex()` is NULL");
}
extern "system" fn dummy_pfnglshaderstorageblockbindingproc (_: GLuint, _: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glShaderStorageBlockBinding()` is NULL");
}
extern "system" fn dummy_pfngltexbufferrangeproc (_: GLenum, _: GLenum, _: GLuint, _: GLintptr, _: GLsizeiptr) {
	panic!("OpenGL Function pointer of `glTexBufferRange()` is NULL");
}
extern "system" fn dummy_pfngltexstorage2dmultisampleproc (_: GLenum, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei, _: GLboolean) {
	panic!("OpenGL Function pointer of `glTexStorage2DMultisample()` is NULL");
}
extern "system" fn dummy_pfngltexstorage3dmultisampleproc (_: GLenum, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei, _: GLsizei, _: GLboolean) {
	panic!("OpenGL Function pointer of `glTexStorage3DMultisample()` is NULL");
}
extern "system" fn dummy_pfngltextureviewproc (_: GLuint, _: GLenum, _: GLuint, _: GLenum, _: GLuint, _: GLuint, _: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glTextureView()` is NULL");
}
extern "system" fn dummy_pfnglbindvertexbufferproc (_: GLuint, _: GLuint, _: GLintptr, _: GLsizei) {
	panic!("OpenGL Function pointer of `glBindVertexBuffer()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribformatproc (_: GLuint, _: GLint, _: GLenum, _: GLboolean, _: GLuint) {
	panic!("OpenGL Function pointer of `glVertexAttribFormat()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribiformatproc (_: GLuint, _: GLint, _: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glVertexAttribIFormat()` is NULL");
}
extern "system" fn dummy_pfnglvertexattriblformatproc (_: GLuint, _: GLint, _: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glVertexAttribLFormat()` is NULL");
}
extern "system" fn dummy_pfnglvertexattribbindingproc (_: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glVertexAttribBinding()` is NULL");
}
extern "system" fn dummy_pfnglvertexbindingdivisorproc (_: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glVertexBindingDivisor()` is NULL");
}
extern "system" fn dummy_pfngldebugmessagecontrolproc (_: GLenum, _: GLenum, _: GLenum, _: GLsizei, _: *const GLuint, _: GLboolean) {
	panic!("OpenGL Function pointer of `glDebugMessageControl()` is NULL");
}
extern "system" fn dummy_pfngldebugmessageinsertproc (_: GLenum, _: GLenum, _: GLuint, _: GLenum, _: GLsizei, _: *const GLchar) {
	panic!("OpenGL Function pointer of `glDebugMessageInsert()` is NULL");
}
extern "system" fn dummy_pfngldebugmessagecallbackproc (_: GLDEBUGPROC, _: *const c_void) {
	panic!("OpenGL Function pointer of `glDebugMessageCallback()` is NULL");
}
extern "system" fn dummy_pfnglgetdebugmessagelogproc (_: GLuint, _: GLsizei, _: *mut GLenum, _: *mut GLenum, _: *mut GLuint, _: *mut GLenum, _: *mut GLsizei, _: *mut GLchar) -> GLuint {
	panic!("OpenGL Function pointer of `glGetDebugMessageLog()` is NULL");
}
extern "system" fn dummy_pfnglpushdebuggroupproc (_: GLenum, _: GLuint, _: GLsizei, _: *const GLchar) {
	panic!("OpenGL Function pointer of `glPushDebugGroup()` is NULL");
}
extern "system" fn dummy_pfnglpopdebuggroupproc () {
	panic!("OpenGL Function pointer of `glPopDebugGroup()` is NULL");
}
extern "system" fn dummy_pfnglobjectlabelproc (_: GLenum, _: GLuint, _: GLsizei, _: *const GLchar) {
	panic!("OpenGL Function pointer of `glObjectLabel()` is NULL");
}
extern "system" fn dummy_pfnglgetobjectlabelproc (_: GLenum, _: GLuint, _: GLsizei, _: *mut GLsizei, _: *mut GLchar) {
	panic!("OpenGL Function pointer of `glGetObjectLabel()` is NULL");
}
extern "system" fn dummy_pfnglobjectptrlabelproc (_: *const c_void, _: GLsizei, _: *const GLchar) {
	panic!("OpenGL Function pointer of `glObjectPtrLabel()` is NULL");
}
extern "system" fn dummy_pfnglgetobjectptrlabelproc (_: *const c_void, _: GLsizei, _: *mut GLsizei, _: *mut GLchar) {
	panic!("OpenGL Function pointer of `glGetObjectPtrLabel()` is NULL");
}
pub const GL_NUM_SHADING_LANGUAGE_VERSIONS: GLenum = 0x82E9;
pub const GL_VERTEX_ATTRIB_ARRAY_LONG: GLenum = 0x874E;
pub const GL_COMPRESSED_RGB8_ETC2: GLenum = 0x9274;
pub const GL_COMPRESSED_SRGB8_ETC2: GLenum = 0x9275;
pub const GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = 0x9276;
pub const GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = 0x9277;
pub const GL_COMPRESSED_RGBA8_ETC2_EAC: GLenum = 0x9278;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: GLenum = 0x9279;
pub const GL_COMPRESSED_R11_EAC: GLenum = 0x9270;
pub const GL_COMPRESSED_SIGNED_R11_EAC: GLenum = 0x9271;
pub const GL_COMPRESSED_RG11_EAC: GLenum = 0x9272;
pub const GL_COMPRESSED_SIGNED_RG11_EAC: GLenum = 0x9273;
pub const GL_PRIMITIVE_RESTART_FIXED_INDEX: GLenum = 0x8D69;
pub const GL_ANY_SAMPLES_PASSED_CONSERVATIVE: GLenum = 0x8D6A;
pub const GL_MAX_ELEMENT_INDEX: GLenum = 0x8D6B;
pub const GL_COMPUTE_SHADER: GLenum = 0x91B9;
pub const GL_MAX_COMPUTE_UNIFORM_BLOCKS: GLenum = 0x91BB;
pub const GL_MAX_COMPUTE_TEXTURE_IMAGE_UNITS: GLenum = 0x91BC;
pub const GL_MAX_COMPUTE_IMAGE_UNIFORMS: GLenum = 0x91BD;
pub const GL_MAX_COMPUTE_SHARED_MEMORY_SIZE: GLenum = 0x8262;
pub const GL_MAX_COMPUTE_UNIFORM_COMPONENTS: GLenum = 0x8263;
pub const GL_MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS: GLenum = 0x8264;
pub const GL_MAX_COMPUTE_ATOMIC_COUNTERS: GLenum = 0x8265;
pub const GL_MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS: GLenum = 0x8266;
pub const GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS: GLenum = 0x90EB;
pub const GL_MAX_COMPUTE_WORK_GROUP_COUNT: GLenum = 0x91BE;
pub const GL_MAX_COMPUTE_WORK_GROUP_SIZE: GLenum = 0x91BF;
pub const GL_COMPUTE_WORK_GROUP_SIZE: GLenum = 0x8267;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x90EC;
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x90ED;
pub const GL_DISPATCH_INDIRECT_BUFFER: GLenum = 0x90EE;
pub const GL_DISPATCH_INDIRECT_BUFFER_BINDING: GLenum = 0x90EF;
pub const GL_COMPUTE_SHADER_BIT: GLbitfield = 0x00000020;
pub const GL_DEBUG_OUTPUT_SYNCHRONOUS: GLenum = 0x8242;
pub const GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: GLenum = 0x8243;
pub const GL_DEBUG_CALLBACK_FUNCTION: GLenum = 0x8244;
pub const GL_DEBUG_CALLBACK_USER_PARAM: GLenum = 0x8245;
pub const GL_DEBUG_SOURCE_API: GLenum = 0x8246;
pub const GL_DEBUG_SOURCE_WINDOW_SYSTEM: GLenum = 0x8247;
pub const GL_DEBUG_SOURCE_SHADER_COMPILER: GLenum = 0x8248;
pub const GL_DEBUG_SOURCE_THIRD_PARTY: GLenum = 0x8249;
pub const GL_DEBUG_SOURCE_APPLICATION: GLenum = 0x824A;
pub const GL_DEBUG_SOURCE_OTHER: GLenum = 0x824B;
pub const GL_DEBUG_TYPE_ERROR: GLenum = 0x824C;
pub const GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR: GLenum = 0x824D;
pub const GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR: GLenum = 0x824E;
pub const GL_DEBUG_TYPE_PORTABILITY: GLenum = 0x824F;
pub const GL_DEBUG_TYPE_PERFORMANCE: GLenum = 0x8250;
pub const GL_DEBUG_TYPE_OTHER: GLenum = 0x8251;
pub const GL_MAX_DEBUG_MESSAGE_LENGTH: GLenum = 0x9143;
pub const GL_MAX_DEBUG_LOGGED_MESSAGES: GLenum = 0x9144;
pub const GL_DEBUG_LOGGED_MESSAGES: GLenum = 0x9145;
pub const GL_DEBUG_SEVERITY_HIGH: GLenum = 0x9146;
pub const GL_DEBUG_SEVERITY_MEDIUM: GLenum = 0x9147;
pub const GL_DEBUG_SEVERITY_LOW: GLenum = 0x9148;
pub const GL_DEBUG_TYPE_MARKER: GLenum = 0x8268;
pub const GL_DEBUG_TYPE_PUSH_GROUP: GLenum = 0x8269;
pub const GL_DEBUG_TYPE_POP_GROUP: GLenum = 0x826A;
pub const GL_DEBUG_SEVERITY_NOTIFICATION: GLenum = 0x826B;
pub const GL_MAX_DEBUG_GROUP_STACK_DEPTH: GLenum = 0x826C;
pub const GL_DEBUG_GROUP_STACK_DEPTH: GLenum = 0x826D;
pub const GL_BUFFER: GLenum = 0x82E0;
pub const GL_SHADER: GLenum = 0x82E1;
pub const GL_PROGRAM: GLenum = 0x82E2;
pub const GL_QUERY: GLenum = 0x82E3;
pub const GL_PROGRAM_PIPELINE: GLenum = 0x82E4;
pub const GL_SAMPLER: GLenum = 0x82E6;
pub const GL_MAX_LABEL_LENGTH: GLenum = 0x82E8;
pub const GL_DEBUG_OUTPUT: GLenum = 0x92E0;
pub const GL_CONTEXT_FLAG_DEBUG_BIT: GLbitfield = 0x00000002;
pub const GL_MAX_UNIFORM_LOCATIONS: GLenum = 0x826E;
pub const GL_FRAMEBUFFER_DEFAULT_WIDTH: GLenum = 0x9310;
pub const GL_FRAMEBUFFER_DEFAULT_HEIGHT: GLenum = 0x9311;
pub const GL_FRAMEBUFFER_DEFAULT_LAYERS: GLenum = 0x9312;
pub const GL_FRAMEBUFFER_DEFAULT_SAMPLES: GLenum = 0x9313;
pub const GL_FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: GLenum = 0x9314;
pub const GL_MAX_FRAMEBUFFER_WIDTH: GLenum = 0x9315;
pub const GL_MAX_FRAMEBUFFER_HEIGHT: GLenum = 0x9316;
pub const GL_MAX_FRAMEBUFFER_LAYERS: GLenum = 0x9317;
pub const GL_MAX_FRAMEBUFFER_SAMPLES: GLenum = 0x9318;
pub const GL_INTERNALFORMAT_SUPPORTED: GLenum = 0x826F;
pub const GL_INTERNALFORMAT_PREFERRED: GLenum = 0x8270;
pub const GL_INTERNALFORMAT_RED_SIZE: GLenum = 0x8271;
pub const GL_INTERNALFORMAT_GREEN_SIZE: GLenum = 0x8272;
pub const GL_INTERNALFORMAT_BLUE_SIZE: GLenum = 0x8273;
pub const GL_INTERNALFORMAT_ALPHA_SIZE: GLenum = 0x8274;
pub const GL_INTERNALFORMAT_DEPTH_SIZE: GLenum = 0x8275;
pub const GL_INTERNALFORMAT_STENCIL_SIZE: GLenum = 0x8276;
pub const GL_INTERNALFORMAT_SHARED_SIZE: GLenum = 0x8277;
pub const GL_INTERNALFORMAT_RED_TYPE: GLenum = 0x8278;
pub const GL_INTERNALFORMAT_GREEN_TYPE: GLenum = 0x8279;
pub const GL_INTERNALFORMAT_BLUE_TYPE: GLenum = 0x827A;
pub const GL_INTERNALFORMAT_ALPHA_TYPE: GLenum = 0x827B;
pub const GL_INTERNALFORMAT_DEPTH_TYPE: GLenum = 0x827C;
pub const GL_INTERNALFORMAT_STENCIL_TYPE: GLenum = 0x827D;
pub const GL_MAX_WIDTH: GLenum = 0x827E;
pub const GL_MAX_HEIGHT: GLenum = 0x827F;
pub const GL_MAX_DEPTH: GLenum = 0x8280;
pub const GL_MAX_LAYERS: GLenum = 0x8281;
pub const GL_MAX_COMBINED_DIMENSIONS: GLenum = 0x8282;
pub const GL_COLOR_COMPONENTS: GLenum = 0x8283;
pub const GL_DEPTH_COMPONENTS: GLenum = 0x8284;
pub const GL_STENCIL_COMPONENTS: GLenum = 0x8285;
pub const GL_COLOR_RENDERABLE: GLenum = 0x8286;
pub const GL_DEPTH_RENDERABLE: GLenum = 0x8287;
pub const GL_STENCIL_RENDERABLE: GLenum = 0x8288;
pub const GL_FRAMEBUFFER_RENDERABLE: GLenum = 0x8289;
pub const GL_FRAMEBUFFER_RENDERABLE_LAYERED: GLenum = 0x828A;
pub const GL_FRAMEBUFFER_BLEND: GLenum = 0x828B;
pub const GL_READ_PIXELS: GLenum = 0x828C;
pub const GL_READ_PIXELS_FORMAT: GLenum = 0x828D;
pub const GL_READ_PIXELS_TYPE: GLenum = 0x828E;
pub const GL_TEXTURE_IMAGE_FORMAT: GLenum = 0x828F;
pub const GL_TEXTURE_IMAGE_TYPE: GLenum = 0x8290;
pub const GL_GET_TEXTURE_IMAGE_FORMAT: GLenum = 0x8291;
pub const GL_GET_TEXTURE_IMAGE_TYPE: GLenum = 0x8292;
pub const GL_MIPMAP: GLenum = 0x8293;
pub const GL_MANUAL_GENERATE_MIPMAP: GLenum = 0x8294;
pub const GL_AUTO_GENERATE_MIPMAP: GLenum = 0x8295;
pub const GL_COLOR_ENCODING: GLenum = 0x8296;
pub const GL_SRGB_READ: GLenum = 0x8297;
pub const GL_SRGB_WRITE: GLenum = 0x8298;
pub const GL_FILTER: GLenum = 0x829A;
pub const GL_VERTEX_TEXTURE: GLenum = 0x829B;
pub const GL_TESS_CONTROL_TEXTURE: GLenum = 0x829C;
pub const GL_TESS_EVALUATION_TEXTURE: GLenum = 0x829D;
pub const GL_GEOMETRY_TEXTURE: GLenum = 0x829E;
pub const GL_FRAGMENT_TEXTURE: GLenum = 0x829F;
pub const GL_COMPUTE_TEXTURE: GLenum = 0x82A0;
pub const GL_TEXTURE_SHADOW: GLenum = 0x82A1;
pub const GL_TEXTURE_GATHER: GLenum = 0x82A2;
pub const GL_TEXTURE_GATHER_SHADOW: GLenum = 0x82A3;
pub const GL_SHADER_IMAGE_LOAD: GLenum = 0x82A4;
pub const GL_SHADER_IMAGE_STORE: GLenum = 0x82A5;
pub const GL_SHADER_IMAGE_ATOMIC: GLenum = 0x82A6;
pub const GL_IMAGE_TEXEL_SIZE: GLenum = 0x82A7;
pub const GL_IMAGE_COMPATIBILITY_CLASS: GLenum = 0x82A8;
pub const GL_IMAGE_PIXEL_FORMAT: GLenum = 0x82A9;
pub const GL_IMAGE_PIXEL_TYPE: GLenum = 0x82AA;
pub const GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST: GLenum = 0x82AC;
pub const GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST: GLenum = 0x82AD;
pub const GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE: GLenum = 0x82AE;
pub const GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE: GLenum = 0x82AF;
pub const GL_TEXTURE_COMPRESSED_BLOCK_WIDTH: GLenum = 0x82B1;
pub const GL_TEXTURE_COMPRESSED_BLOCK_HEIGHT: GLenum = 0x82B2;
pub const GL_TEXTURE_COMPRESSED_BLOCK_SIZE: GLenum = 0x82B3;
pub const GL_CLEAR_BUFFER: GLenum = 0x82B4;
pub const GL_TEXTURE_VIEW: GLenum = 0x82B5;
pub const GL_VIEW_COMPATIBILITY_CLASS: GLenum = 0x82B6;
pub const GL_FULL_SUPPORT: GLenum = 0x82B7;
pub const GL_CAVEAT_SUPPORT: GLenum = 0x82B8;
pub const GL_IMAGE_CLASS_4_X_32: GLenum = 0x82B9;
pub const GL_IMAGE_CLASS_2_X_32: GLenum = 0x82BA;
pub const GL_IMAGE_CLASS_1_X_32: GLenum = 0x82BB;
pub const GL_IMAGE_CLASS_4_X_16: GLenum = 0x82BC;
pub const GL_IMAGE_CLASS_2_X_16: GLenum = 0x82BD;
pub const GL_IMAGE_CLASS_1_X_16: GLenum = 0x82BE;
pub const GL_IMAGE_CLASS_4_X_8: GLenum = 0x82BF;
pub const GL_IMAGE_CLASS_2_X_8: GLenum = 0x82C0;
pub const GL_IMAGE_CLASS_1_X_8: GLenum = 0x82C1;
pub const GL_IMAGE_CLASS_11_11_10: GLenum = 0x82C2;
pub const GL_IMAGE_CLASS_10_10_10_2: GLenum = 0x82C3;
pub const GL_VIEW_CLASS_128_BITS: GLenum = 0x82C4;
pub const GL_VIEW_CLASS_96_BITS: GLenum = 0x82C5;
pub const GL_VIEW_CLASS_64_BITS: GLenum = 0x82C6;
pub const GL_VIEW_CLASS_48_BITS: GLenum = 0x82C7;
pub const GL_VIEW_CLASS_32_BITS: GLenum = 0x82C8;
pub const GL_VIEW_CLASS_24_BITS: GLenum = 0x82C9;
pub const GL_VIEW_CLASS_16_BITS: GLenum = 0x82CA;
pub const GL_VIEW_CLASS_8_BITS: GLenum = 0x82CB;
pub const GL_VIEW_CLASS_S3TC_DXT1_RGB: GLenum = 0x82CC;
pub const GL_VIEW_CLASS_S3TC_DXT1_RGBA: GLenum = 0x82CD;
pub const GL_VIEW_CLASS_S3TC_DXT3_RGBA: GLenum = 0x82CE;
pub const GL_VIEW_CLASS_S3TC_DXT5_RGBA: GLenum = 0x82CF;
pub const GL_VIEW_CLASS_RGTC1_RED: GLenum = 0x82D0;
pub const GL_VIEW_CLASS_RGTC2_RG: GLenum = 0x82D1;
pub const GL_VIEW_CLASS_BPTC_UNORM: GLenum = 0x82D2;
pub const GL_VIEW_CLASS_BPTC_FLOAT: GLenum = 0x82D3;
pub const GL_UNIFORM: GLenum = 0x92E1;
pub const GL_UNIFORM_BLOCK: GLenum = 0x92E2;
pub const GL_PROGRAM_INPUT: GLenum = 0x92E3;
pub const GL_PROGRAM_OUTPUT: GLenum = 0x92E4;
pub const GL_BUFFER_VARIABLE: GLenum = 0x92E5;
pub const GL_SHADER_STORAGE_BLOCK: GLenum = 0x92E6;
pub const GL_VERTEX_SUBROUTINE: GLenum = 0x92E8;
pub const GL_TESS_CONTROL_SUBROUTINE: GLenum = 0x92E9;
pub const GL_TESS_EVALUATION_SUBROUTINE: GLenum = 0x92EA;
pub const GL_GEOMETRY_SUBROUTINE: GLenum = 0x92EB;
pub const GL_FRAGMENT_SUBROUTINE: GLenum = 0x92EC;
pub const GL_COMPUTE_SUBROUTINE: GLenum = 0x92ED;
pub const GL_VERTEX_SUBROUTINE_UNIFORM: GLenum = 0x92EE;
pub const GL_TESS_CONTROL_SUBROUTINE_UNIFORM: GLenum = 0x92EF;
pub const GL_TESS_EVALUATION_SUBROUTINE_UNIFORM: GLenum = 0x92F0;
pub const GL_GEOMETRY_SUBROUTINE_UNIFORM: GLenum = 0x92F1;
pub const GL_FRAGMENT_SUBROUTINE_UNIFORM: GLenum = 0x92F2;
pub const GL_COMPUTE_SUBROUTINE_UNIFORM: GLenum = 0x92F3;
pub const GL_TRANSFORM_FEEDBACK_VARYING: GLenum = 0x92F4;
pub const GL_ACTIVE_RESOURCES: GLenum = 0x92F5;
pub const GL_MAX_NAME_LENGTH: GLenum = 0x92F6;
pub const GL_MAX_NUM_ACTIVE_VARIABLES: GLenum = 0x92F7;
pub const GL_MAX_NUM_COMPATIBLE_SUBROUTINES: GLenum = 0x92F8;
pub const GL_NAME_LENGTH: GLenum = 0x92F9;
pub const GL_TYPE: GLenum = 0x92FA;
pub const GL_ARRAY_SIZE: GLenum = 0x92FB;
pub const GL_OFFSET: GLenum = 0x92FC;
pub const GL_BLOCK_INDEX: GLenum = 0x92FD;
pub const GL_ARRAY_STRIDE: GLenum = 0x92FE;
pub const GL_MATRIX_STRIDE: GLenum = 0x92FF;
pub const GL_IS_ROW_MAJOR: GLenum = 0x9300;
pub const GL_ATOMIC_COUNTER_BUFFER_INDEX: GLenum = 0x9301;
pub const GL_BUFFER_BINDING: GLenum = 0x9302;
pub const GL_BUFFER_DATA_SIZE: GLenum = 0x9303;
pub const GL_NUM_ACTIVE_VARIABLES: GLenum = 0x9304;
pub const GL_ACTIVE_VARIABLES: GLenum = 0x9305;
pub const GL_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x9306;
pub const GL_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x9307;
pub const GL_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x9308;
pub const GL_REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x9309;
pub const GL_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x930A;
pub const GL_REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x930B;
pub const GL_TOP_LEVEL_ARRAY_SIZE: GLenum = 0x930C;
pub const GL_TOP_LEVEL_ARRAY_STRIDE: GLenum = 0x930D;
pub const GL_LOCATION: GLenum = 0x930E;
pub const GL_LOCATION_INDEX: GLenum = 0x930F;
pub const GL_IS_PER_PATCH: GLenum = 0x92E7;
pub const GL_SHADER_STORAGE_BUFFER: GLenum = 0x90D2;
pub const GL_SHADER_STORAGE_BUFFER_BINDING: GLenum = 0x90D3;
pub const GL_SHADER_STORAGE_BUFFER_START: GLenum = 0x90D4;
pub const GL_SHADER_STORAGE_BUFFER_SIZE: GLenum = 0x90D5;
pub const GL_MAX_VERTEX_SHADER_STORAGE_BLOCKS: GLenum = 0x90D6;
pub const GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS: GLenum = 0x90D7;
pub const GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS: GLenum = 0x90D8;
pub const GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS: GLenum = 0x90D9;
pub const GL_MAX_FRAGMENT_SHADER_STORAGE_BLOCKS: GLenum = 0x90DA;
pub const GL_MAX_COMPUTE_SHADER_STORAGE_BLOCKS: GLenum = 0x90DB;
pub const GL_MAX_COMBINED_SHADER_STORAGE_BLOCKS: GLenum = 0x90DC;
pub const GL_MAX_SHADER_STORAGE_BUFFER_BINDINGS: GLenum = 0x90DD;
pub const GL_MAX_SHADER_STORAGE_BLOCK_SIZE: GLenum = 0x90DE;
pub const GL_SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x90DF;
pub const GL_SHADER_STORAGE_BARRIER_BIT: GLbitfield = 0x00002000;
pub const GL_MAX_COMBINED_SHADER_OUTPUT_RESOURCES: GLenum = 0x8F39;
pub const GL_DEPTH_STENCIL_TEXTURE_MODE: GLenum = 0x90EA;
pub const GL_TEXTURE_BUFFER_OFFSET: GLenum = 0x919D;
pub const GL_TEXTURE_BUFFER_SIZE: GLenum = 0x919E;
pub const GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x919F;
pub const GL_TEXTURE_VIEW_MIN_LEVEL: GLenum = 0x82DB;
pub const GL_TEXTURE_VIEW_NUM_LEVELS: GLenum = 0x82DC;
pub const GL_TEXTURE_VIEW_MIN_LAYER: GLenum = 0x82DD;
pub const GL_TEXTURE_VIEW_NUM_LAYERS: GLenum = 0x82DE;
pub const GL_TEXTURE_IMMUTABLE_LEVELS: GLenum = 0x82DF;
pub const GL_VERTEX_ATTRIB_BINDING: GLenum = 0x82D4;
pub const GL_VERTEX_ATTRIB_RELATIVE_OFFSET: GLenum = 0x82D5;
pub const GL_VERTEX_BINDING_DIVISOR: GLenum = 0x82D6;
pub const GL_VERTEX_BINDING_OFFSET: GLenum = 0x82D7;
pub const GL_VERTEX_BINDING_STRIDE: GLenum = 0x82D8;
pub const GL_MAX_VERTEX_ATTRIB_RELATIVE_OFFSET: GLenum = 0x82D9;
pub const GL_MAX_VERTEX_ATTRIB_BINDINGS: GLenum = 0x82DA;
pub const GL_VERTEX_BINDING_BUFFER: GLenum = 0x8F4F;
pub const GL_DISPLAY_LIST: GLenum = 0x82E7;

pub trait GL_4_3 {
	fn glClearBufferData(&self, target: GLenum, internalformat: GLenum, format: GLenum, type_: GLenum, data: *const c_void);
	fn glClearBufferSubData(&self, target: GLenum, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *const c_void);
	fn glDispatchCompute(&self, num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint);
	fn glDispatchComputeIndirect(&self, indirect: GLintptr);
	fn glCopyImageSubData(&self, srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei);
	fn glFramebufferParameteri(&self, target: GLenum, pname: GLenum, param: GLint);
	fn glGetFramebufferParameteriv(&self, target: GLenum, pname: GLenum, params: *mut GLint);
	fn glGetInternalformati64v(&self, target: GLenum, internalformat: GLenum, pname: GLenum, count: GLsizei, params: *mut GLint64);
	fn glInvalidateTexSubImage(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei);
	fn glInvalidateTexImage(&self, texture: GLuint, level: GLint);
	fn glInvalidateBufferSubData(&self, buffer: GLuint, offset: GLintptr, length: GLsizeiptr);
	fn glInvalidateBufferData(&self, buffer: GLuint);
	fn glInvalidateFramebuffer(&self, target: GLenum, numAttachments: GLsizei, attachments: *const GLenum);
	fn glInvalidateSubFramebuffer(&self, target: GLenum, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
	fn glMultiDrawArraysIndirect(&self, mode: GLenum, indirect: *const c_void, drawcount: GLsizei, stride: GLsizei);
	fn glMultiDrawElementsIndirect(&self, mode: GLenum, type_: GLenum, indirect: *const c_void, drawcount: GLsizei, stride: GLsizei);
	fn glGetProgramInterfaceiv(&self, program: GLuint, programInterface: GLenum, pname: GLenum, params: *mut GLint);
	fn glGetProgramResourceIndex(&self, program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLuint;
	fn glGetProgramResourceName(&self, program: GLuint, programInterface: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar);
	fn glGetProgramResourceiv(&self, program: GLuint, programInterface: GLenum, index: GLuint, propCount: GLsizei, props: *const GLenum, count: GLsizei, length: *mut GLsizei, params: *mut GLint);
	fn glGetProgramResourceLocation(&self, program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLint;
	fn glGetProgramResourceLocationIndex(&self, program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLint;
	fn glShaderStorageBlockBinding(&self, program: GLuint, storageBlockIndex: GLuint, storageBlockBinding: GLuint);
	fn glTexBufferRange(&self, target: GLenum, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);
	fn glTexStorage2DMultisample(&self, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);
	fn glTexStorage3DMultisample(&self, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);
	fn glTextureView(&self, texture: GLuint, target: GLenum, origtexture: GLuint, internalformat: GLenum, minlevel: GLuint, numlevels: GLuint, minlayer: GLuint, numlayers: GLuint);
	fn glBindVertexBuffer(&self, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei);
	fn glVertexAttribFormat(&self, attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint);
	fn glVertexAttribIFormat(&self, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint);
	fn glVertexAttribLFormat(&self, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint);
	fn glVertexAttribBinding(&self, attribindex: GLuint, bindingindex: GLuint);
	fn glVertexBindingDivisor(&self, bindingindex: GLuint, divisor: GLuint);
	fn glDebugMessageControl(&self, source: GLenum, type_: GLenum, severity: GLenum, count: GLsizei, ids: *const GLuint, enabled: GLboolean);
	fn glDebugMessageInsert(&self, source: GLenum, type_: GLenum, id: GLuint, severity: GLenum, length: GLsizei, buf: *const GLchar);
	fn glDebugMessageCallback(&self, callback: GLDEBUGPROC, userParam: *const c_void);
	fn glGetDebugMessageLog(&self, count: GLuint, bufSize: GLsizei, sources: *mut GLenum, types: *mut GLenum, ids: *mut GLuint, severities: *mut GLenum, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint;
	fn glPushDebugGroup(&self, source: GLenum, id: GLuint, length: GLsizei, message: *const GLchar);
	fn glPopDebugGroup(&self);
	fn glObjectLabel(&self, identifier: GLenum, name: GLuint, length: GLsizei, label: *const GLchar);
	fn glGetObjectLabel(&self, identifier: GLenum, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar);
	fn glObjectPtrLabel(&self, ptr: *const c_void, length: GLsizei, label: *const GLchar);
	fn glGetObjectPtrLabel(&self, ptr: *const c_void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
	pub fn new(base: impl GL_1_0, mut get_proc_address: impl FnMut(&'static str) -> *const c_void) -> Self {
		let (_spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 40300 {
			return Self::default();
		}
		Self {
			available: true,
			clearbufferdata: {let proc = get_proc_address("glClearBufferData"); if proc == null() {dummy_pfnglclearbufferdataproc} else {unsafe{transmute(proc)}}},
			clearbuffersubdata: {let proc = get_proc_address("glClearBufferSubData"); if proc == null() {dummy_pfnglclearbuffersubdataproc} else {unsafe{transmute(proc)}}},
			dispatchcompute: {let proc = get_proc_address("glDispatchCompute"); if proc == null() {dummy_pfngldispatchcomputeproc} else {unsafe{transmute(proc)}}},
			dispatchcomputeindirect: {let proc = get_proc_address("glDispatchComputeIndirect"); if proc == null() {dummy_pfngldispatchcomputeindirectproc} else {unsafe{transmute(proc)}}},
			copyimagesubdata: {let proc = get_proc_address("glCopyImageSubData"); if proc == null() {dummy_pfnglcopyimagesubdataproc} else {unsafe{transmute(proc)}}},
			framebufferparameteri: {let proc = get_proc_address("glFramebufferParameteri"); if proc == null() {dummy_pfnglframebufferparameteriproc} else {unsafe{transmute(proc)}}},
			getframebufferparameteriv: {let proc = get_proc_address("glGetFramebufferParameteriv"); if proc == null() {dummy_pfnglgetframebufferparameterivproc} else {unsafe{transmute(proc)}}},
			getinternalformati64v: {let proc = get_proc_address("glGetInternalformati64v"); if proc == null() {dummy_pfnglgetinternalformati64vproc} else {unsafe{transmute(proc)}}},
			invalidatetexsubimage: {let proc = get_proc_address("glInvalidateTexSubImage"); if proc == null() {dummy_pfnglinvalidatetexsubimageproc} else {unsafe{transmute(proc)}}},
			invalidateteximage: {let proc = get_proc_address("glInvalidateTexImage"); if proc == null() {dummy_pfnglinvalidateteximageproc} else {unsafe{transmute(proc)}}},
			invalidatebuffersubdata: {let proc = get_proc_address("glInvalidateBufferSubData"); if proc == null() {dummy_pfnglinvalidatebuffersubdataproc} else {unsafe{transmute(proc)}}},
			invalidatebufferdata: {let proc = get_proc_address("glInvalidateBufferData"); if proc == null() {dummy_pfnglinvalidatebufferdataproc} else {unsafe{transmute(proc)}}},
			invalidateframebuffer: {let proc = get_proc_address("glInvalidateFramebuffer"); if proc == null() {dummy_pfnglinvalidateframebufferproc} else {unsafe{transmute(proc)}}},
			invalidatesubframebuffer: {let proc = get_proc_address("glInvalidateSubFramebuffer"); if proc == null() {dummy_pfnglinvalidatesubframebufferproc} else {unsafe{transmute(proc)}}},
			multidrawarraysindirect: {let proc = get_proc_address("glMultiDrawArraysIndirect"); if proc == null() {dummy_pfnglmultidrawarraysindirectproc} else {unsafe{transmute(proc)}}},
			multidrawelementsindirect: {let proc = get_proc_address("glMultiDrawElementsIndirect"); if proc == null() {dummy_pfnglmultidrawelementsindirectproc} else {unsafe{transmute(proc)}}},
			getprograminterfaceiv: {let proc = get_proc_address("glGetProgramInterfaceiv"); if proc == null() {dummy_pfnglgetprograminterfaceivproc} else {unsafe{transmute(proc)}}},
			getprogramresourceindex: {let proc = get_proc_address("glGetProgramResourceIndex"); if proc == null() {dummy_pfnglgetprogramresourceindexproc} else {unsafe{transmute(proc)}}},
			getprogramresourcename: {let proc = get_proc_address("glGetProgramResourceName"); if proc == null() {dummy_pfnglgetprogramresourcenameproc} else {unsafe{transmute(proc)}}},
			getprogramresourceiv: {let proc = get_proc_address("glGetProgramResourceiv"); if proc == null() {dummy_pfnglgetprogramresourceivproc} else {unsafe{transmute(proc)}}},
			getprogramresourcelocation: {let proc = get_proc_address("glGetProgramResourceLocation"); if proc == null() {dummy_pfnglgetprogramresourcelocationproc} else {unsafe{transmute(proc)}}},
			getprogramresourcelocationindex: {let proc = get_proc_address("glGetProgramResourceLocationIndex"); if proc == null() {dummy_pfnglgetprogramresourcelocationindexproc} else {unsafe{transmute(proc)}}},
			shaderstorageblockbinding: {let proc = get_proc_address("glShaderStorageBlockBinding"); if proc == null() {dummy_pfnglshaderstorageblockbindingproc} else {unsafe{transmute(proc)}}},
			texbufferrange: {let proc = get_proc_address("glTexBufferRange"); if proc == null() {dummy_pfngltexbufferrangeproc} else {unsafe{transmute(proc)}}},
			texstorage2dmultisample: {let proc = get_proc_address("glTexStorage2DMultisample"); if proc == null() {dummy_pfngltexstorage2dmultisampleproc} else {unsafe{transmute(proc)}}},
			texstorage3dmultisample: {let proc = get_proc_address("glTexStorage3DMultisample"); if proc == null() {dummy_pfngltexstorage3dmultisampleproc} else {unsafe{transmute(proc)}}},
			textureview: {let proc = get_proc_address("glTextureView"); if proc == null() {dummy_pfngltextureviewproc} else {unsafe{transmute(proc)}}},
			bindvertexbuffer: {let proc = get_proc_address("glBindVertexBuffer"); if proc == null() {dummy_pfnglbindvertexbufferproc} else {unsafe{transmute(proc)}}},
			vertexattribformat: {let proc = get_proc_address("glVertexAttribFormat"); if proc == null() {dummy_pfnglvertexattribformatproc} else {unsafe{transmute(proc)}}},
			vertexattribiformat: {let proc = get_proc_address("glVertexAttribIFormat"); if proc == null() {dummy_pfnglvertexattribiformatproc} else {unsafe{transmute(proc)}}},
			vertexattriblformat: {let proc = get_proc_address("glVertexAttribLFormat"); if proc == null() {dummy_pfnglvertexattriblformatproc} else {unsafe{transmute(proc)}}},
			vertexattribbinding: {let proc = get_proc_address("glVertexAttribBinding"); if proc == null() {dummy_pfnglvertexattribbindingproc} else {unsafe{transmute(proc)}}},
			vertexbindingdivisor: {let proc = get_proc_address("glVertexBindingDivisor"); if proc == null() {dummy_pfnglvertexbindingdivisorproc} else {unsafe{transmute(proc)}}},
			debugmessagecontrol: {let proc = get_proc_address("glDebugMessageControl"); if proc == null() {dummy_pfngldebugmessagecontrolproc} else {unsafe{transmute(proc)}}},
			debugmessageinsert: {let proc = get_proc_address("glDebugMessageInsert"); if proc == null() {dummy_pfngldebugmessageinsertproc} else {unsafe{transmute(proc)}}},
			debugmessagecallback: {let proc = get_proc_address("glDebugMessageCallback"); if proc == null() {dummy_pfngldebugmessagecallbackproc} else {unsafe{transmute(proc)}}},
			getdebugmessagelog: {let proc = get_proc_address("glGetDebugMessageLog"); if proc == null() {dummy_pfnglgetdebugmessagelogproc} else {unsafe{transmute(proc)}}},
			pushdebuggroup: {let proc = get_proc_address("glPushDebugGroup"); if proc == null() {dummy_pfnglpushdebuggroupproc} else {unsafe{transmute(proc)}}},
			popdebuggroup: {let proc = get_proc_address("glPopDebugGroup"); if proc == null() {dummy_pfnglpopdebuggroupproc} else {unsafe{transmute(proc)}}},
			objectlabel: {let proc = get_proc_address("glObjectLabel"); if proc == null() {dummy_pfnglobjectlabelproc} else {unsafe{transmute(proc)}}},
			getobjectlabel: {let proc = get_proc_address("glGetObjectLabel"); if proc == null() {dummy_pfnglgetobjectlabelproc} else {unsafe{transmute(proc)}}},
			objectptrlabel: {let proc = get_proc_address("glObjectPtrLabel"); if proc == null() {dummy_pfnglobjectptrlabelproc} else {unsafe{transmute(proc)}}},
			getobjectptrlabel: {let proc = get_proc_address("glGetObjectPtrLabel"); if proc == null() {dummy_pfnglgetobjectptrlabelproc} else {unsafe{transmute(proc)}}},
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
			clearbufferdata: dummy_pfnglclearbufferdataproc,
			clearbuffersubdata: dummy_pfnglclearbuffersubdataproc,
			dispatchcompute: dummy_pfngldispatchcomputeproc,
			dispatchcomputeindirect: dummy_pfngldispatchcomputeindirectproc,
			copyimagesubdata: dummy_pfnglcopyimagesubdataproc,
			framebufferparameteri: dummy_pfnglframebufferparameteriproc,
			getframebufferparameteriv: dummy_pfnglgetframebufferparameterivproc,
			getinternalformati64v: dummy_pfnglgetinternalformati64vproc,
			invalidatetexsubimage: dummy_pfnglinvalidatetexsubimageproc,
			invalidateteximage: dummy_pfnglinvalidateteximageproc,
			invalidatebuffersubdata: dummy_pfnglinvalidatebuffersubdataproc,
			invalidatebufferdata: dummy_pfnglinvalidatebufferdataproc,
			invalidateframebuffer: dummy_pfnglinvalidateframebufferproc,
			invalidatesubframebuffer: dummy_pfnglinvalidatesubframebufferproc,
			multidrawarraysindirect: dummy_pfnglmultidrawarraysindirectproc,
			multidrawelementsindirect: dummy_pfnglmultidrawelementsindirectproc,
			getprograminterfaceiv: dummy_pfnglgetprograminterfaceivproc,
			getprogramresourceindex: dummy_pfnglgetprogramresourceindexproc,
			getprogramresourcename: dummy_pfnglgetprogramresourcenameproc,
			getprogramresourceiv: dummy_pfnglgetprogramresourceivproc,
			getprogramresourcelocation: dummy_pfnglgetprogramresourcelocationproc,
			getprogramresourcelocationindex: dummy_pfnglgetprogramresourcelocationindexproc,
			shaderstorageblockbinding: dummy_pfnglshaderstorageblockbindingproc,
			texbufferrange: dummy_pfngltexbufferrangeproc,
			texstorage2dmultisample: dummy_pfngltexstorage2dmultisampleproc,
			texstorage3dmultisample: dummy_pfngltexstorage3dmultisampleproc,
			textureview: dummy_pfngltextureviewproc,
			bindvertexbuffer: dummy_pfnglbindvertexbufferproc,
			vertexattribformat: dummy_pfnglvertexattribformatproc,
			vertexattribiformat: dummy_pfnglvertexattribiformatproc,
			vertexattriblformat: dummy_pfnglvertexattriblformatproc,
			vertexattribbinding: dummy_pfnglvertexattribbindingproc,
			vertexbindingdivisor: dummy_pfnglvertexbindingdivisorproc,
			debugmessagecontrol: dummy_pfngldebugmessagecontrolproc,
			debugmessageinsert: dummy_pfngldebugmessageinsertproc,
			debugmessagecallback: dummy_pfngldebugmessagecallbackproc,
			getdebugmessagelog: dummy_pfnglgetdebugmessagelogproc,
			pushdebuggroup: dummy_pfnglpushdebuggroupproc,
			popdebuggroup: dummy_pfnglpopdebuggroupproc,
			objectlabel: dummy_pfnglobjectlabelproc,
			getobjectlabel: dummy_pfnglgetobjectlabelproc,
			objectptrlabel: dummy_pfnglobjectptrlabelproc,
			getobjectptrlabel: dummy_pfnglgetobjectptrlabelproc,
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
extern "system" fn dummy_pfnglbufferstorageproc (_: GLenum, _: GLsizeiptr, _: *const c_void, _: GLbitfield) {
	panic!("OpenGL Function pointer of `glBufferStorage()` is NULL");
}
extern "system" fn dummy_pfnglclearteximageproc (_: GLuint, _: GLint, _: GLenum, _: GLenum, _: *const c_void) {
	panic!("OpenGL Function pointer of `glClearTexImage()` is NULL");
}
extern "system" fn dummy_pfnglcleartexsubimageproc (_: GLuint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLsizei, _: GLenum, _: GLenum, _: *const c_void) {
	panic!("OpenGL Function pointer of `glClearTexSubImage()` is NULL");
}
extern "system" fn dummy_pfnglbindbuffersbaseproc (_: GLenum, _: GLuint, _: GLsizei, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glBindBuffersBase()` is NULL");
}
extern "system" fn dummy_pfnglbindbuffersrangeproc (_: GLenum, _: GLuint, _: GLsizei, _: *const GLuint, _: *const GLintptr, _: *const GLsizeiptr) {
	panic!("OpenGL Function pointer of `glBindBuffersRange()` is NULL");
}
extern "system" fn dummy_pfnglbindtexturesproc (_: GLuint, _: GLsizei, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glBindTextures()` is NULL");
}
extern "system" fn dummy_pfnglbindsamplersproc (_: GLuint, _: GLsizei, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glBindSamplers()` is NULL");
}
extern "system" fn dummy_pfnglbindimagetexturesproc (_: GLuint, _: GLsizei, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glBindImageTextures()` is NULL");
}
extern "system" fn dummy_pfnglbindvertexbuffersproc (_: GLuint, _: GLsizei, _: *const GLuint, _: *const GLintptr, _: *const GLsizei) {
	panic!("OpenGL Function pointer of `glBindVertexBuffers()` is NULL");
}
pub const GL_MAX_VERTEX_ATTRIB_STRIDE: GLenum = 0x82E5;
pub const GL_PRIMITIVE_RESTART_FOR_PATCHES_SUPPORTED: GLenum = 0x8221;
pub const GL_TEXTURE_BUFFER_BINDING: GLenum = 0x8C2A;
pub const GL_MAP_PERSISTENT_BIT: GLbitfield = 0x0040;
pub const GL_MAP_COHERENT_BIT: GLbitfield = 0x0080;
pub const GL_DYNAMIC_STORAGE_BIT: GLbitfield = 0x0100;
pub const GL_CLIENT_STORAGE_BIT: GLbitfield = 0x0200;
pub const GL_CLIENT_MAPPED_BUFFER_BARRIER_BIT: GLbitfield = 0x00004000;
pub const GL_BUFFER_IMMUTABLE_STORAGE: GLenum = 0x821F;
pub const GL_BUFFER_STORAGE_FLAGS: GLenum = 0x8220;
pub const GL_CLEAR_TEXTURE: GLenum = 0x9365;
pub const GL_LOCATION_COMPONENT: GLenum = 0x934A;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_INDEX: GLenum = 0x934B;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_STRIDE: GLenum = 0x934C;
pub const GL_QUERY_BUFFER: GLenum = 0x9192;
pub const GL_QUERY_BUFFER_BARRIER_BIT: GLbitfield = 0x00008000;
pub const GL_QUERY_BUFFER_BINDING: GLenum = 0x9193;
pub const GL_QUERY_RESULT_NO_WAIT: GLenum = 0x9194;
pub const GL_MIRROR_CLAMP_TO_EDGE: GLenum = 0x8743;

pub trait GL_4_4 {
	fn glBufferStorage(&self, target: GLenum, size: GLsizeiptr, data: *const c_void, flags: GLbitfield);
	fn glClearTexImage(&self, texture: GLuint, level: GLint, format: GLenum, type_: GLenum, data: *const c_void);
	fn glClearTexSubImage(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, data: *const c_void);
	fn glBindBuffersBase(&self, target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint);
	fn glBindBuffersRange(&self, target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, sizes: *const GLsizeiptr);
	fn glBindTextures(&self, first: GLuint, count: GLsizei, textures: *const GLuint);
	fn glBindSamplers(&self, first: GLuint, count: GLsizei, samplers: *const GLuint);
	fn glBindImageTextures(&self, first: GLuint, count: GLsizei, textures: *const GLuint);
	fn glBindVertexBuffers(&self, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
	pub fn new(base: impl GL_1_0, mut get_proc_address: impl FnMut(&'static str) -> *const c_void) -> Self {
		let (_spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 40400 {
			return Self::default();
		}
		Self {
			available: true,
			bufferstorage: {let proc = get_proc_address("glBufferStorage"); if proc == null() {dummy_pfnglbufferstorageproc} else {unsafe{transmute(proc)}}},
			clearteximage: {let proc = get_proc_address("glClearTexImage"); if proc == null() {dummy_pfnglclearteximageproc} else {unsafe{transmute(proc)}}},
			cleartexsubimage: {let proc = get_proc_address("glClearTexSubImage"); if proc == null() {dummy_pfnglcleartexsubimageproc} else {unsafe{transmute(proc)}}},
			bindbuffersbase: {let proc = get_proc_address("glBindBuffersBase"); if proc == null() {dummy_pfnglbindbuffersbaseproc} else {unsafe{transmute(proc)}}},
			bindbuffersrange: {let proc = get_proc_address("glBindBuffersRange"); if proc == null() {dummy_pfnglbindbuffersrangeproc} else {unsafe{transmute(proc)}}},
			bindtextures: {let proc = get_proc_address("glBindTextures"); if proc == null() {dummy_pfnglbindtexturesproc} else {unsafe{transmute(proc)}}},
			bindsamplers: {let proc = get_proc_address("glBindSamplers"); if proc == null() {dummy_pfnglbindsamplersproc} else {unsafe{transmute(proc)}}},
			bindimagetextures: {let proc = get_proc_address("glBindImageTextures"); if proc == null() {dummy_pfnglbindimagetexturesproc} else {unsafe{transmute(proc)}}},
			bindvertexbuffers: {let proc = get_proc_address("glBindVertexBuffers"); if proc == null() {dummy_pfnglbindvertexbuffersproc} else {unsafe{transmute(proc)}}},
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
			bufferstorage: dummy_pfnglbufferstorageproc,
			clearteximage: dummy_pfnglclearteximageproc,
			cleartexsubimage: dummy_pfnglcleartexsubimageproc,
			bindbuffersbase: dummy_pfnglbindbuffersbaseproc,
			bindbuffersrange: dummy_pfnglbindbuffersrangeproc,
			bindtextures: dummy_pfnglbindtexturesproc,
			bindsamplers: dummy_pfnglbindsamplersproc,
			bindimagetextures: dummy_pfnglbindimagetexturesproc,
			bindvertexbuffers: dummy_pfnglbindvertexbuffersproc,
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
extern "system" fn dummy_pfnglclipcontrolproc (_: GLenum, _: GLenum) {
	panic!("OpenGL Function pointer of `glClipControl()` is NULL");
}
extern "system" fn dummy_pfnglcreatetransformfeedbacksproc (_: GLsizei, _: *mut GLuint) {
	panic!("OpenGL Function pointer of `glCreateTransformFeedbacks()` is NULL");
}
extern "system" fn dummy_pfngltransformfeedbackbufferbaseproc (_: GLuint, _: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glTransformFeedbackBufferBase()` is NULL");
}
extern "system" fn dummy_pfngltransformfeedbackbufferrangeproc (_: GLuint, _: GLuint, _: GLuint, _: GLintptr, _: GLsizeiptr) {
	panic!("OpenGL Function pointer of `glTransformFeedbackBufferRange()` is NULL");
}
extern "system" fn dummy_pfnglgettransformfeedbackivproc (_: GLuint, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetTransformFeedbackiv()` is NULL");
}
extern "system" fn dummy_pfnglgettransformfeedbacki_vproc (_: GLuint, _: GLenum, _: GLuint, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetTransformFeedbacki_v()` is NULL");
}
extern "system" fn dummy_pfnglgettransformfeedbacki64_vproc (_: GLuint, _: GLenum, _: GLuint, _: *mut GLint64) {
	panic!("OpenGL Function pointer of `glGetTransformFeedbacki64_v()` is NULL");
}
extern "system" fn dummy_pfnglcreatebuffersproc (_: GLsizei, _: *mut GLuint) {
	panic!("OpenGL Function pointer of `glCreateBuffers()` is NULL");
}
extern "system" fn dummy_pfnglnamedbufferstorageproc (_: GLuint, _: GLsizeiptr, _: *const c_void, _: GLbitfield) {
	panic!("OpenGL Function pointer of `glNamedBufferStorage()` is NULL");
}
extern "system" fn dummy_pfnglnamedbufferdataproc (_: GLuint, _: GLsizeiptr, _: *const c_void, _: GLenum) {
	panic!("OpenGL Function pointer of `glNamedBufferData()` is NULL");
}
extern "system" fn dummy_pfnglnamedbuffersubdataproc (_: GLuint, _: GLintptr, _: GLsizeiptr, _: *const c_void) {
	panic!("OpenGL Function pointer of `glNamedBufferSubData()` is NULL");
}
extern "system" fn dummy_pfnglcopynamedbuffersubdataproc (_: GLuint, _: GLuint, _: GLintptr, _: GLintptr, _: GLsizeiptr) {
	panic!("OpenGL Function pointer of `glCopyNamedBufferSubData()` is NULL");
}
extern "system" fn dummy_pfnglclearnamedbufferdataproc (_: GLuint, _: GLenum, _: GLenum, _: GLenum, _: *const c_void) {
	panic!("OpenGL Function pointer of `glClearNamedBufferData()` is NULL");
}
extern "system" fn dummy_pfnglclearnamedbuffersubdataproc (_: GLuint, _: GLenum, _: GLintptr, _: GLsizeiptr, _: GLenum, _: GLenum, _: *const c_void) {
	panic!("OpenGL Function pointer of `glClearNamedBufferSubData()` is NULL");
}
extern "system" fn dummy_pfnglmapnamedbufferproc (_: GLuint, _: GLenum) -> *mut c_void {
	panic!("OpenGL Function pointer of `glMapNamedBuffer()` is NULL");
}
extern "system" fn dummy_pfnglmapnamedbufferrangeproc (_: GLuint, _: GLintptr, _: GLsizeiptr, _: GLbitfield) -> *mut c_void {
	panic!("OpenGL Function pointer of `glMapNamedBufferRange()` is NULL");
}
extern "system" fn dummy_pfnglunmapnamedbufferproc (_: GLuint) -> GLboolean {
	panic!("OpenGL Function pointer of `glUnmapNamedBuffer()` is NULL");
}
extern "system" fn dummy_pfnglflushmappednamedbufferrangeproc (_: GLuint, _: GLintptr, _: GLsizeiptr) {
	panic!("OpenGL Function pointer of `glFlushMappedNamedBufferRange()` is NULL");
}
extern "system" fn dummy_pfnglgetnamedbufferparameterivproc (_: GLuint, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetNamedBufferParameteriv()` is NULL");
}
extern "system" fn dummy_pfnglgetnamedbufferparameteri64vproc (_: GLuint, _: GLenum, _: *mut GLint64) {
	panic!("OpenGL Function pointer of `glGetNamedBufferParameteri64v()` is NULL");
}
extern "system" fn dummy_pfnglgetnamedbufferpointervproc (_: GLuint, _: GLenum, _: *mut *mut c_void) {
	panic!("OpenGL Function pointer of `glGetNamedBufferPointerv()` is NULL");
}
extern "system" fn dummy_pfnglgetnamedbuffersubdataproc (_: GLuint, _: GLintptr, _: GLsizeiptr, _: *mut c_void) {
	panic!("OpenGL Function pointer of `glGetNamedBufferSubData()` is NULL");
}
extern "system" fn dummy_pfnglcreateframebuffersproc (_: GLsizei, _: *mut GLuint) {
	panic!("OpenGL Function pointer of `glCreateFramebuffers()` is NULL");
}
extern "system" fn dummy_pfnglnamedframebufferrenderbufferproc (_: GLuint, _: GLenum, _: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glNamedFramebufferRenderbuffer()` is NULL");
}
extern "system" fn dummy_pfnglnamedframebufferparameteriproc (_: GLuint, _: GLenum, _: GLint) {
	panic!("OpenGL Function pointer of `glNamedFramebufferParameteri()` is NULL");
}
extern "system" fn dummy_pfnglnamedframebuffertextureproc (_: GLuint, _: GLenum, _: GLuint, _: GLint) {
	panic!("OpenGL Function pointer of `glNamedFramebufferTexture()` is NULL");
}
extern "system" fn dummy_pfnglnamedframebuffertexturelayerproc (_: GLuint, _: GLenum, _: GLuint, _: GLint, _: GLint) {
	panic!("OpenGL Function pointer of `glNamedFramebufferTextureLayer()` is NULL");
}
extern "system" fn dummy_pfnglnamedframebufferdrawbufferproc (_: GLuint, _: GLenum) {
	panic!("OpenGL Function pointer of `glNamedFramebufferDrawBuffer()` is NULL");
}
extern "system" fn dummy_pfnglnamedframebufferdrawbuffersproc (_: GLuint, _: GLsizei, _: *const GLenum) {
	panic!("OpenGL Function pointer of `glNamedFramebufferDrawBuffers()` is NULL");
}
extern "system" fn dummy_pfnglnamedframebufferreadbufferproc (_: GLuint, _: GLenum) {
	panic!("OpenGL Function pointer of `glNamedFramebufferReadBuffer()` is NULL");
}
extern "system" fn dummy_pfnglinvalidatenamedframebufferdataproc (_: GLuint, _: GLsizei, _: *const GLenum) {
	panic!("OpenGL Function pointer of `glInvalidateNamedFramebufferData()` is NULL");
}
extern "system" fn dummy_pfnglinvalidatenamedframebuffersubdataproc (_: GLuint, _: GLsizei, _: *const GLenum, _: GLint, _: GLint, _: GLsizei, _: GLsizei) {
	panic!("OpenGL Function pointer of `glInvalidateNamedFramebufferSubData()` is NULL");
}
extern "system" fn dummy_pfnglclearnamedframebufferivproc (_: GLuint, _: GLenum, _: GLint, _: *const GLint) {
	panic!("OpenGL Function pointer of `glClearNamedFramebufferiv()` is NULL");
}
extern "system" fn dummy_pfnglclearnamedframebufferuivproc (_: GLuint, _: GLenum, _: GLint, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glClearNamedFramebufferuiv()` is NULL");
}
extern "system" fn dummy_pfnglclearnamedframebufferfvproc (_: GLuint, _: GLenum, _: GLint, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glClearNamedFramebufferfv()` is NULL");
}
extern "system" fn dummy_pfnglclearnamedframebufferfiproc (_: GLuint, _: GLenum, _: GLint, _: GLfloat, _: GLint) {
	panic!("OpenGL Function pointer of `glClearNamedFramebufferfi()` is NULL");
}
extern "system" fn dummy_pfnglblitnamedframebufferproc (_: GLuint, _: GLuint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLbitfield, _: GLenum) {
	panic!("OpenGL Function pointer of `glBlitNamedFramebuffer()` is NULL");
}
extern "system" fn dummy_pfnglchecknamedframebufferstatusproc (_: GLuint, _: GLenum) -> GLenum {
	panic!("OpenGL Function pointer of `glCheckNamedFramebufferStatus()` is NULL");
}
extern "system" fn dummy_pfnglgetnamedframebufferparameterivproc (_: GLuint, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetNamedFramebufferParameteriv()` is NULL");
}
extern "system" fn dummy_pfnglgetnamedframebufferattachmentparameterivproc (_: GLuint, _: GLenum, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetNamedFramebufferAttachmentParameteriv()` is NULL");
}
extern "system" fn dummy_pfnglcreaterenderbuffersproc (_: GLsizei, _: *mut GLuint) {
	panic!("OpenGL Function pointer of `glCreateRenderbuffers()` is NULL");
}
extern "system" fn dummy_pfnglnamedrenderbufferstorageproc (_: GLuint, _: GLenum, _: GLsizei, _: GLsizei) {
	panic!("OpenGL Function pointer of `glNamedRenderbufferStorage()` is NULL");
}
extern "system" fn dummy_pfnglnamedrenderbufferstoragemultisampleproc (_: GLuint, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei) {
	panic!("OpenGL Function pointer of `glNamedRenderbufferStorageMultisample()` is NULL");
}
extern "system" fn dummy_pfnglgetnamedrenderbufferparameterivproc (_: GLuint, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetNamedRenderbufferParameteriv()` is NULL");
}
extern "system" fn dummy_pfnglcreatetexturesproc (_: GLenum, _: GLsizei, _: *mut GLuint) {
	panic!("OpenGL Function pointer of `glCreateTextures()` is NULL");
}
extern "system" fn dummy_pfngltexturebufferproc (_: GLuint, _: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glTextureBuffer()` is NULL");
}
extern "system" fn dummy_pfngltexturebufferrangeproc (_: GLuint, _: GLenum, _: GLuint, _: GLintptr, _: GLsizeiptr) {
	panic!("OpenGL Function pointer of `glTextureBufferRange()` is NULL");
}
extern "system" fn dummy_pfngltexturestorage1dproc (_: GLuint, _: GLsizei, _: GLenum, _: GLsizei) {
	panic!("OpenGL Function pointer of `glTextureStorage1D()` is NULL");
}
extern "system" fn dummy_pfngltexturestorage2dproc (_: GLuint, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei) {
	panic!("OpenGL Function pointer of `glTextureStorage2D()` is NULL");
}
extern "system" fn dummy_pfngltexturestorage3dproc (_: GLuint, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei, _: GLsizei) {
	panic!("OpenGL Function pointer of `glTextureStorage3D()` is NULL");
}
extern "system" fn dummy_pfngltexturestorage2dmultisampleproc (_: GLuint, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei, _: GLboolean) {
	panic!("OpenGL Function pointer of `glTextureStorage2DMultisample()` is NULL");
}
extern "system" fn dummy_pfngltexturestorage3dmultisampleproc (_: GLuint, _: GLsizei, _: GLenum, _: GLsizei, _: GLsizei, _: GLsizei, _: GLboolean) {
	panic!("OpenGL Function pointer of `glTextureStorage3DMultisample()` is NULL");
}
extern "system" fn dummy_pfngltexturesubimage1dproc (_: GLuint, _: GLint, _: GLint, _: GLsizei, _: GLenum, _: GLenum, _: *const c_void) {
	panic!("OpenGL Function pointer of `glTextureSubImage1D()` is NULL");
}
extern "system" fn dummy_pfngltexturesubimage2dproc (_: GLuint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLenum, _: GLenum, _: *const c_void) {
	panic!("OpenGL Function pointer of `glTextureSubImage2D()` is NULL");
}
extern "system" fn dummy_pfngltexturesubimage3dproc (_: GLuint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLsizei, _: GLenum, _: GLenum, _: *const c_void) {
	panic!("OpenGL Function pointer of `glTextureSubImage3D()` is NULL");
}
extern "system" fn dummy_pfnglcompressedtexturesubimage1dproc (_: GLuint, _: GLint, _: GLint, _: GLsizei, _: GLenum, _: GLsizei, _: *const c_void) {
	panic!("OpenGL Function pointer of `glCompressedTextureSubImage1D()` is NULL");
}
extern "system" fn dummy_pfnglcompressedtexturesubimage2dproc (_: GLuint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLenum, _: GLsizei, _: *const c_void) {
	panic!("OpenGL Function pointer of `glCompressedTextureSubImage2D()` is NULL");
}
extern "system" fn dummy_pfnglcompressedtexturesubimage3dproc (_: GLuint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLsizei, _: GLenum, _: GLsizei, _: *const c_void) {
	panic!("OpenGL Function pointer of `glCompressedTextureSubImage3D()` is NULL");
}
extern "system" fn dummy_pfnglcopytexturesubimage1dproc (_: GLuint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei) {
	panic!("OpenGL Function pointer of `glCopyTextureSubImage1D()` is NULL");
}
extern "system" fn dummy_pfnglcopytexturesubimage2dproc (_: GLuint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei) {
	panic!("OpenGL Function pointer of `glCopyTextureSubImage2D()` is NULL");
}
extern "system" fn dummy_pfnglcopytexturesubimage3dproc (_: GLuint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei) {
	panic!("OpenGL Function pointer of `glCopyTextureSubImage3D()` is NULL");
}
extern "system" fn dummy_pfngltextureparameterfproc (_: GLuint, _: GLenum, _: GLfloat) {
	panic!("OpenGL Function pointer of `glTextureParameterf()` is NULL");
}
extern "system" fn dummy_pfngltextureparameterfvproc (_: GLuint, _: GLenum, _: *const GLfloat) {
	panic!("OpenGL Function pointer of `glTextureParameterfv()` is NULL");
}
extern "system" fn dummy_pfngltextureparameteriproc (_: GLuint, _: GLenum, _: GLint) {
	panic!("OpenGL Function pointer of `glTextureParameteri()` is NULL");
}
extern "system" fn dummy_pfngltextureparameteriivproc (_: GLuint, _: GLenum, _: *const GLint) {
	panic!("OpenGL Function pointer of `glTextureParameterIiv()` is NULL");
}
extern "system" fn dummy_pfngltextureparameteriuivproc (_: GLuint, _: GLenum, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glTextureParameterIuiv()` is NULL");
}
extern "system" fn dummy_pfngltextureparameterivproc (_: GLuint, _: GLenum, _: *const GLint) {
	panic!("OpenGL Function pointer of `glTextureParameteriv()` is NULL");
}
extern "system" fn dummy_pfnglgeneratetexturemipmapproc (_: GLuint) {
	panic!("OpenGL Function pointer of `glGenerateTextureMipmap()` is NULL");
}
extern "system" fn dummy_pfnglbindtextureunitproc (_: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glBindTextureUnit()` is NULL");
}
extern "system" fn dummy_pfnglgettextureimageproc (_: GLuint, _: GLint, _: GLenum, _: GLenum, _: GLsizei, _: *mut c_void) {
	panic!("OpenGL Function pointer of `glGetTextureImage()` is NULL");
}
extern "system" fn dummy_pfnglgetcompressedtextureimageproc (_: GLuint, _: GLint, _: GLsizei, _: *mut c_void) {
	panic!("OpenGL Function pointer of `glGetCompressedTextureImage()` is NULL");
}
extern "system" fn dummy_pfnglgettexturelevelparameterfvproc (_: GLuint, _: GLint, _: GLenum, _: *mut GLfloat) {
	panic!("OpenGL Function pointer of `glGetTextureLevelParameterfv()` is NULL");
}
extern "system" fn dummy_pfnglgettexturelevelparameterivproc (_: GLuint, _: GLint, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetTextureLevelParameteriv()` is NULL");
}
extern "system" fn dummy_pfnglgettextureparameterfvproc (_: GLuint, _: GLenum, _: *mut GLfloat) {
	panic!("OpenGL Function pointer of `glGetTextureParameterfv()` is NULL");
}
extern "system" fn dummy_pfnglgettextureparameteriivproc (_: GLuint, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetTextureParameterIiv()` is NULL");
}
extern "system" fn dummy_pfnglgettextureparameteriuivproc (_: GLuint, _: GLenum, _: *mut GLuint) {
	panic!("OpenGL Function pointer of `glGetTextureParameterIuiv()` is NULL");
}
extern "system" fn dummy_pfnglgettextureparameterivproc (_: GLuint, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetTextureParameteriv()` is NULL");
}
extern "system" fn dummy_pfnglcreatevertexarraysproc (_: GLsizei, _: *mut GLuint) {
	panic!("OpenGL Function pointer of `glCreateVertexArrays()` is NULL");
}
extern "system" fn dummy_pfngldisablevertexarrayattribproc (_: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glDisableVertexArrayAttrib()` is NULL");
}
extern "system" fn dummy_pfnglenablevertexarrayattribproc (_: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glEnableVertexArrayAttrib()` is NULL");
}
extern "system" fn dummy_pfnglvertexarrayelementbufferproc (_: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glVertexArrayElementBuffer()` is NULL");
}
extern "system" fn dummy_pfnglvertexarrayvertexbufferproc (_: GLuint, _: GLuint, _: GLuint, _: GLintptr, _: GLsizei) {
	panic!("OpenGL Function pointer of `glVertexArrayVertexBuffer()` is NULL");
}
extern "system" fn dummy_pfnglvertexarrayvertexbuffersproc (_: GLuint, _: GLuint, _: GLsizei, _: *const GLuint, _: *const GLintptr, _: *const GLsizei) {
	panic!("OpenGL Function pointer of `glVertexArrayVertexBuffers()` is NULL");
}
extern "system" fn dummy_pfnglvertexarrayattribbindingproc (_: GLuint, _: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glVertexArrayAttribBinding()` is NULL");
}
extern "system" fn dummy_pfnglvertexarrayattribformatproc (_: GLuint, _: GLuint, _: GLint, _: GLenum, _: GLboolean, _: GLuint) {
	panic!("OpenGL Function pointer of `glVertexArrayAttribFormat()` is NULL");
}
extern "system" fn dummy_pfnglvertexarrayattribiformatproc (_: GLuint, _: GLuint, _: GLint, _: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glVertexArrayAttribIFormat()` is NULL");
}
extern "system" fn dummy_pfnglvertexarrayattriblformatproc (_: GLuint, _: GLuint, _: GLint, _: GLenum, _: GLuint) {
	panic!("OpenGL Function pointer of `glVertexArrayAttribLFormat()` is NULL");
}
extern "system" fn dummy_pfnglvertexarraybindingdivisorproc (_: GLuint, _: GLuint, _: GLuint) {
	panic!("OpenGL Function pointer of `glVertexArrayBindingDivisor()` is NULL");
}
extern "system" fn dummy_pfnglgetvertexarrayivproc (_: GLuint, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetVertexArrayiv()` is NULL");
}
extern "system" fn dummy_pfnglgetvertexarrayindexedivproc (_: GLuint, _: GLuint, _: GLenum, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetVertexArrayIndexediv()` is NULL");
}
extern "system" fn dummy_pfnglgetvertexarrayindexed64ivproc (_: GLuint, _: GLuint, _: GLenum, _: *mut GLint64) {
	panic!("OpenGL Function pointer of `glGetVertexArrayIndexed64iv()` is NULL");
}
extern "system" fn dummy_pfnglcreatesamplersproc (_: GLsizei, _: *mut GLuint) {
	panic!("OpenGL Function pointer of `glCreateSamplers()` is NULL");
}
extern "system" fn dummy_pfnglcreateprogrampipelinesproc (_: GLsizei, _: *mut GLuint) {
	panic!("OpenGL Function pointer of `glCreateProgramPipelines()` is NULL");
}
extern "system" fn dummy_pfnglcreatequeriesproc (_: GLenum, _: GLsizei, _: *mut GLuint) {
	panic!("OpenGL Function pointer of `glCreateQueries()` is NULL");
}
extern "system" fn dummy_pfnglgetquerybufferobjecti64vproc (_: GLuint, _: GLuint, _: GLenum, _: GLintptr) {
	panic!("OpenGL Function pointer of `glGetQueryBufferObjecti64v()` is NULL");
}
extern "system" fn dummy_pfnglgetquerybufferobjectivproc (_: GLuint, _: GLuint, _: GLenum, _: GLintptr) {
	panic!("OpenGL Function pointer of `glGetQueryBufferObjectiv()` is NULL");
}
extern "system" fn dummy_pfnglgetquerybufferobjectui64vproc (_: GLuint, _: GLuint, _: GLenum, _: GLintptr) {
	panic!("OpenGL Function pointer of `glGetQueryBufferObjectui64v()` is NULL");
}
extern "system" fn dummy_pfnglgetquerybufferobjectuivproc (_: GLuint, _: GLuint, _: GLenum, _: GLintptr) {
	panic!("OpenGL Function pointer of `glGetQueryBufferObjectuiv()` is NULL");
}
extern "system" fn dummy_pfnglmemorybarrierbyregionproc (_: GLbitfield) {
	panic!("OpenGL Function pointer of `glMemoryBarrierByRegion()` is NULL");
}
extern "system" fn dummy_pfnglgettexturesubimageproc (_: GLuint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLsizei, _: GLenum, _: GLenum, _: GLsizei, _: *mut c_void) {
	panic!("OpenGL Function pointer of `glGetTextureSubImage()` is NULL");
}
extern "system" fn dummy_pfnglgetcompressedtexturesubimageproc (_: GLuint, _: GLint, _: GLint, _: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLsizei, _: GLsizei, _: *mut c_void) {
	panic!("OpenGL Function pointer of `glGetCompressedTextureSubImage()` is NULL");
}
extern "system" fn dummy_pfnglgetgraphicsresetstatusproc () -> GLenum {
	panic!("OpenGL Function pointer of `glGetGraphicsResetStatus()` is NULL");
}
extern "system" fn dummy_pfnglgetncompressedteximageproc (_: GLenum, _: GLint, _: GLsizei, _: *mut c_void) {
	panic!("OpenGL Function pointer of `glGetnCompressedTexImage()` is NULL");
}
extern "system" fn dummy_pfnglgetnteximageproc (_: GLenum, _: GLint, _: GLenum, _: GLenum, _: GLsizei, _: *mut c_void) {
	panic!("OpenGL Function pointer of `glGetnTexImage()` is NULL");
}
extern "system" fn dummy_pfnglgetnuniformdvproc (_: GLuint, _: GLint, _: GLsizei, _: *mut GLdouble) {
	panic!("OpenGL Function pointer of `glGetnUniformdv()` is NULL");
}
extern "system" fn dummy_pfnglgetnuniformfvproc (_: GLuint, _: GLint, _: GLsizei, _: *mut GLfloat) {
	panic!("OpenGL Function pointer of `glGetnUniformfv()` is NULL");
}
extern "system" fn dummy_pfnglgetnuniformivproc (_: GLuint, _: GLint, _: GLsizei, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetnUniformiv()` is NULL");
}
extern "system" fn dummy_pfnglgetnuniformuivproc (_: GLuint, _: GLint, _: GLsizei, _: *mut GLuint) {
	panic!("OpenGL Function pointer of `glGetnUniformuiv()` is NULL");
}
extern "system" fn dummy_pfnglreadnpixelsproc (_: GLint, _: GLint, _: GLsizei, _: GLsizei, _: GLenum, _: GLenum, _: GLsizei, _: *mut c_void) {
	panic!("OpenGL Function pointer of `glReadnPixels()` is NULL");
}
extern "system" fn dummy_pfnglgetnmapdvproc (_: GLenum, _: GLenum, _: GLsizei, _: *mut GLdouble) {
	panic!("OpenGL Function pointer of `glGetnMapdv()` is NULL");
}
extern "system" fn dummy_pfnglgetnmapfvproc (_: GLenum, _: GLenum, _: GLsizei, _: *mut GLfloat) {
	panic!("OpenGL Function pointer of `glGetnMapfv()` is NULL");
}
extern "system" fn dummy_pfnglgetnmapivproc (_: GLenum, _: GLenum, _: GLsizei, _: *mut GLint) {
	panic!("OpenGL Function pointer of `glGetnMapiv()` is NULL");
}
extern "system" fn dummy_pfnglgetnpixelmapfvproc (_: GLenum, _: GLsizei, _: *mut GLfloat) {
	panic!("OpenGL Function pointer of `glGetnPixelMapfv()` is NULL");
}
extern "system" fn dummy_pfnglgetnpixelmapuivproc (_: GLenum, _: GLsizei, _: *mut GLuint) {
	panic!("OpenGL Function pointer of `glGetnPixelMapuiv()` is NULL");
}
extern "system" fn dummy_pfnglgetnpixelmapusvproc (_: GLenum, _: GLsizei, _: *mut GLushort) {
	panic!("OpenGL Function pointer of `glGetnPixelMapusv()` is NULL");
}
extern "system" fn dummy_pfnglgetnpolygonstippleproc (_: GLsizei, _: *mut GLubyte) {
	panic!("OpenGL Function pointer of `glGetnPolygonStipple()` is NULL");
}
extern "system" fn dummy_pfnglgetncolortableproc (_: GLenum, _: GLenum, _: GLenum, _: GLsizei, _: *mut c_void) {
	panic!("OpenGL Function pointer of `glGetnColorTable()` is NULL");
}
extern "system" fn dummy_pfnglgetnconvolutionfilterproc (_: GLenum, _: GLenum, _: GLenum, _: GLsizei, _: *mut c_void) {
	panic!("OpenGL Function pointer of `glGetnConvolutionFilter()` is NULL");
}
extern "system" fn dummy_pfnglgetnseparablefilterproc (_: GLenum, _: GLenum, _: GLenum, _: GLsizei, _: *mut c_void, _: GLsizei, _: *mut c_void, _: *mut c_void) {
	panic!("OpenGL Function pointer of `glGetnSeparableFilter()` is NULL");
}
extern "system" fn dummy_pfnglgetnhistogramproc (_: GLenum, _: GLboolean, _: GLenum, _: GLenum, _: GLsizei, _: *mut c_void) {
	panic!("OpenGL Function pointer of `glGetnHistogram()` is NULL");
}
extern "system" fn dummy_pfnglgetnminmaxproc (_: GLenum, _: GLboolean, _: GLenum, _: GLenum, _: GLsizei, _: *mut c_void) {
	panic!("OpenGL Function pointer of `glGetnMinmax()` is NULL");
}
extern "system" fn dummy_pfngltexturebarrierproc () {
	panic!("OpenGL Function pointer of `glTextureBarrier()` is NULL");
}
pub const GL_CONTEXT_LOST: GLenum = 0x0507;
pub const GL_NEGATIVE_ONE_TO_ONE: GLenum = 0x935E;
pub const GL_ZERO_TO_ONE: GLenum = 0x935F;
pub const GL_CLIP_ORIGIN: GLenum = 0x935C;
pub const GL_CLIP_DEPTH_MODE: GLenum = 0x935D;
pub const GL_QUERY_WAIT_INVERTED: GLenum = 0x8E17;
pub const GL_QUERY_NO_WAIT_INVERTED: GLenum = 0x8E18;
pub const GL_QUERY_BY_REGION_WAIT_INVERTED: GLenum = 0x8E19;
pub const GL_QUERY_BY_REGION_NO_WAIT_INVERTED: GLenum = 0x8E1A;
pub const GL_MAX_CULL_DISTANCES: GLenum = 0x82F9;
pub const GL_MAX_COMBINED_CLIP_AND_CULL_DISTANCES: GLenum = 0x82FA;
pub const GL_TEXTURE_TARGET: GLenum = 0x1006;
pub const GL_QUERY_TARGET: GLenum = 0x82EA;
pub const GL_GUILTY_CONTEXT_RESET: GLenum = 0x8253;
pub const GL_INNOCENT_CONTEXT_RESET: GLenum = 0x8254;
pub const GL_UNKNOWN_CONTEXT_RESET: GLenum = 0x8255;
pub const GL_RESET_NOTIFICATION_STRATEGY: GLenum = 0x8256;
pub const GL_LOSE_CONTEXT_ON_RESET: GLenum = 0x8252;
pub const GL_NO_RESET_NOTIFICATION: GLenum = 0x8261;
pub const GL_CONTEXT_FLAG_ROBUST_ACCESS_BIT: GLbitfield = 0x00000004;
pub const GL_COLOR_TABLE: GLenum = 0x80D0;
pub const GL_POST_CONVOLUTION_COLOR_TABLE: GLenum = 0x80D1;
pub const GL_POST_COLOR_MATRIX_COLOR_TABLE: GLenum = 0x80D2;
pub const GL_PROXY_COLOR_TABLE: GLenum = 0x80D3;
pub const GL_PROXY_POST_CONVOLUTION_COLOR_TABLE: GLenum = 0x80D4;
pub const GL_PROXY_POST_COLOR_MATRIX_COLOR_TABLE: GLenum = 0x80D5;
pub const GL_CONVOLUTION_1D: GLenum = 0x8010;
pub const GL_CONVOLUTION_2D: GLenum = 0x8011;
pub const GL_SEPARABLE_2D: GLenum = 0x8012;
pub const GL_HISTOGRAM: GLenum = 0x8024;
pub const GL_PROXY_HISTOGRAM: GLenum = 0x8025;
pub const GL_MINMAX: GLenum = 0x802E;
pub const GL_CONTEXT_RELEASE_BEHAVIOR: GLenum = 0x82FB;
pub const GL_CONTEXT_RELEASE_BEHAVIOR_FLUSH: GLenum = 0x82FC;

pub trait GL_4_5 {
	fn glClipControl(&self, origin: GLenum, depth: GLenum);
	fn glCreateTransformFeedbacks(&self, n: GLsizei, ids: *mut GLuint);
	fn glTransformFeedbackBufferBase(&self, xfb: GLuint, index: GLuint, buffer: GLuint);
	fn glTransformFeedbackBufferRange(&self, xfb: GLuint, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);
	fn glGetTransformFeedbackiv(&self, xfb: GLuint, pname: GLenum, param: *mut GLint);
	fn glGetTransformFeedbacki_v(&self, xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint);
	fn glGetTransformFeedbacki64_v(&self, xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint64);
	fn glCreateBuffers(&self, n: GLsizei, buffers: *mut GLuint);
	fn glNamedBufferStorage(&self, buffer: GLuint, size: GLsizeiptr, data: *const c_void, flags: GLbitfield);
	fn glNamedBufferData(&self, buffer: GLuint, size: GLsizeiptr, data: *const c_void, usage: GLenum);
	fn glNamedBufferSubData(&self, buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const c_void);
	fn glCopyNamedBufferSubData(&self, readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);
	fn glClearNamedBufferData(&self, buffer: GLuint, internalformat: GLenum, format: GLenum, type_: GLenum, data: *const c_void);
	fn glClearNamedBufferSubData(&self, buffer: GLuint, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *const c_void);
	fn glMapNamedBuffer(&self, buffer: GLuint, access: GLenum) -> *mut c_void;
	fn glMapNamedBufferRange(&self, buffer: GLuint, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut c_void;
	fn glUnmapNamedBuffer(&self, buffer: GLuint) -> GLboolean;
	fn glFlushMappedNamedBufferRange(&self, buffer: GLuint, offset: GLintptr, length: GLsizeiptr);
	fn glGetNamedBufferParameteriv(&self, buffer: GLuint, pname: GLenum, params: *mut GLint);
	fn glGetNamedBufferParameteri64v(&self, buffer: GLuint, pname: GLenum, params: *mut GLint64);
	fn glGetNamedBufferPointerv(&self, buffer: GLuint, pname: GLenum, params: *mut *mut c_void);
	fn glGetNamedBufferSubData(&self, buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *mut c_void);
	fn glCreateFramebuffers(&self, n: GLsizei, framebuffers: *mut GLuint);
	fn glNamedFramebufferRenderbuffer(&self, framebuffer: GLuint, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint);
	fn glNamedFramebufferParameteri(&self, framebuffer: GLuint, pname: GLenum, param: GLint);
	fn glNamedFramebufferTexture(&self, framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint);
	fn glNamedFramebufferTextureLayer(&self, framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint);
	fn glNamedFramebufferDrawBuffer(&self, framebuffer: GLuint, buf: GLenum);
	fn glNamedFramebufferDrawBuffers(&self, framebuffer: GLuint, n: GLsizei, bufs: *const GLenum);
	fn glNamedFramebufferReadBuffer(&self, framebuffer: GLuint, src: GLenum);
	fn glInvalidateNamedFramebufferData(&self, framebuffer: GLuint, numAttachments: GLsizei, attachments: *const GLenum);
	fn glInvalidateNamedFramebufferSubData(&self, framebuffer: GLuint, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
	fn glClearNamedFramebufferiv(&self, framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLint);
	fn glClearNamedFramebufferuiv(&self, framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLuint);
	fn glClearNamedFramebufferfv(&self, framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLfloat);
	fn glClearNamedFramebufferfi(&self, framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint);
	fn glBlitNamedFramebuffer(&self, readFramebuffer: GLuint, drawFramebuffer: GLuint, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum);
	fn glCheckNamedFramebufferStatus(&self, framebuffer: GLuint, target: GLenum) -> GLenum;
	fn glGetNamedFramebufferParameteriv(&self, framebuffer: GLuint, pname: GLenum, param: *mut GLint);
	fn glGetNamedFramebufferAttachmentParameteriv(&self, framebuffer: GLuint, attachment: GLenum, pname: GLenum, params: *mut GLint);
	fn glCreateRenderbuffers(&self, n: GLsizei, renderbuffers: *mut GLuint);
	fn glNamedRenderbufferStorage(&self, renderbuffer: GLuint, internalformat: GLenum, width: GLsizei, height: GLsizei);
	fn glNamedRenderbufferStorageMultisample(&self, renderbuffer: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
	fn glGetNamedRenderbufferParameteriv(&self, renderbuffer: GLuint, pname: GLenum, params: *mut GLint);
	fn glCreateTextures(&self, target: GLenum, n: GLsizei, textures: *mut GLuint);
	fn glTextureBuffer(&self, texture: GLuint, internalformat: GLenum, buffer: GLuint);
	fn glTextureBufferRange(&self, texture: GLuint, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);
	fn glTextureStorage1D(&self, texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei);
	fn glTextureStorage2D(&self, texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei);
	fn glTextureStorage3D(&self, texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei);
	fn glTextureStorage2DMultisample(&self, texture: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);
	fn glTextureStorage3DMultisample(&self, texture: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);
	fn glTextureSubImage1D(&self, texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
	fn glTextureSubImage2D(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
	fn glTextureSubImage3D(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void);
	fn glCompressedTextureSubImage1D(&self, texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
	fn glCompressedTextureSubImage2D(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
	fn glCompressedTextureSubImage3D(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void);
	fn glCopyTextureSubImage1D(&self, texture: GLuint, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);
	fn glCopyTextureSubImage2D(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
	fn glCopyTextureSubImage3D(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
	fn glTextureParameterf(&self, texture: GLuint, pname: GLenum, param: GLfloat);
	fn glTextureParameterfv(&self, texture: GLuint, pname: GLenum, param: *const GLfloat);
	fn glTextureParameteri(&self, texture: GLuint, pname: GLenum, param: GLint);
	fn glTextureParameterIiv(&self, texture: GLuint, pname: GLenum, params: *const GLint);
	fn glTextureParameterIuiv(&self, texture: GLuint, pname: GLenum, params: *const GLuint);
	fn glTextureParameteriv(&self, texture: GLuint, pname: GLenum, param: *const GLint);
	fn glGenerateTextureMipmap(&self, texture: GLuint);
	fn glBindTextureUnit(&self, unit: GLuint, texture: GLuint);
	fn glGetTextureImage(&self, texture: GLuint, level: GLint, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut c_void);
	fn glGetCompressedTextureImage(&self, texture: GLuint, level: GLint, bufSize: GLsizei, pixels: *mut c_void);
	fn glGetTextureLevelParameterfv(&self, texture: GLuint, level: GLint, pname: GLenum, params: *mut GLfloat);
	fn glGetTextureLevelParameteriv(&self, texture: GLuint, level: GLint, pname: GLenum, params: *mut GLint);
	fn glGetTextureParameterfv(&self, texture: GLuint, pname: GLenum, params: *mut GLfloat);
	fn glGetTextureParameterIiv(&self, texture: GLuint, pname: GLenum, params: *mut GLint);
	fn glGetTextureParameterIuiv(&self, texture: GLuint, pname: GLenum, params: *mut GLuint);
	fn glGetTextureParameteriv(&self, texture: GLuint, pname: GLenum, params: *mut GLint);
	fn glCreateVertexArrays(&self, n: GLsizei, arrays: *mut GLuint);
	fn glDisableVertexArrayAttrib(&self, vaobj: GLuint, index: GLuint);
	fn glEnableVertexArrayAttrib(&self, vaobj: GLuint, index: GLuint);
	fn glVertexArrayElementBuffer(&self, vaobj: GLuint, buffer: GLuint);
	fn glVertexArrayVertexBuffer(&self, vaobj: GLuint, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei);
	fn glVertexArrayVertexBuffers(&self, vaobj: GLuint, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei);
	fn glVertexArrayAttribBinding(&self, vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint);
	fn glVertexArrayAttribFormat(&self, vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint);
	fn glVertexArrayAttribIFormat(&self, vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint);
	fn glVertexArrayAttribLFormat(&self, vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint);
	fn glVertexArrayBindingDivisor(&self, vaobj: GLuint, bindingindex: GLuint, divisor: GLuint);
	fn glGetVertexArrayiv(&self, vaobj: GLuint, pname: GLenum, param: *mut GLint);
	fn glGetVertexArrayIndexediv(&self, vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint);
	fn glGetVertexArrayIndexed64iv(&self, vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint64);
	fn glCreateSamplers(&self, n: GLsizei, samplers: *mut GLuint);
	fn glCreateProgramPipelines(&self, n: GLsizei, pipelines: *mut GLuint);
	fn glCreateQueries(&self, target: GLenum, n: GLsizei, ids: *mut GLuint);
	fn glGetQueryBufferObjecti64v(&self, id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);
	fn glGetQueryBufferObjectiv(&self, id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);
	fn glGetQueryBufferObjectui64v(&self, id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);
	fn glGetQueryBufferObjectuiv(&self, id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr);
	fn glMemoryBarrierByRegion(&self, barriers: GLbitfield);
	fn glGetTextureSubImage(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut c_void);
	fn glGetCompressedTextureSubImage(&self, texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, bufSize: GLsizei, pixels: *mut c_void);
	fn glGetGraphicsResetStatus(&self) -> GLenum;
	fn glGetnCompressedTexImage(&self, target: GLenum, lod: GLint, bufSize: GLsizei, pixels: *mut c_void);
	fn glGetnTexImage(&self, target: GLenum, level: GLint, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut c_void);
	fn glGetnUniformdv(&self, program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLdouble);
	fn glGetnUniformfv(&self, program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLfloat);
	fn glGetnUniformiv(&self, program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint);
	fn glGetnUniformuiv(&self, program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint);
	fn glReadnPixels(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, bufSize: GLsizei, data: *mut c_void);
	fn glGetnMapdv(&self, target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLdouble);
	fn glGetnMapfv(&self, target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLfloat);
	fn glGetnMapiv(&self, target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLint);
	fn glGetnPixelMapfv(&self, map: GLenum, bufSize: GLsizei, values: *mut GLfloat);
	fn glGetnPixelMapuiv(&self, map: GLenum, bufSize: GLsizei, values: *mut GLuint);
	fn glGetnPixelMapusv(&self, map: GLenum, bufSize: GLsizei, values: *mut GLushort);
	fn glGetnPolygonStipple(&self, bufSize: GLsizei, pattern: *mut GLubyte);
	fn glGetnColorTable(&self, target: GLenum, format: GLenum, type_: GLenum, bufSize: GLsizei, table: *mut c_void);
	fn glGetnConvolutionFilter(&self, target: GLenum, format: GLenum, type_: GLenum, bufSize: GLsizei, image: *mut c_void);
	fn glGetnSeparableFilter(&self, target: GLenum, format: GLenum, type_: GLenum, rowBufSize: GLsizei, row: *mut c_void, columnBufSize: GLsizei, column: *mut c_void, span: *mut c_void);
	fn glGetnHistogram(&self, target: GLenum, reset: GLboolean, format: GLenum, type_: GLenum, bufSize: GLsizei, values: *mut c_void);
	fn glGetnMinmax(&self, target: GLenum, reset: GLboolean, format: GLenum, type_: GLenum, bufSize: GLsizei, values: *mut c_void);
	fn glTextureBarrier(&self);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
	pub fn new(base: impl GL_1_0, mut get_proc_address: impl FnMut(&'static str) -> *const c_void) -> Self {
		let (_spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 40500 {
			return Self::default();
		}
		Self {
			available: true,
			clipcontrol: {let proc = get_proc_address("glClipControl"); if proc == null() {dummy_pfnglclipcontrolproc} else {unsafe{transmute(proc)}}},
			createtransformfeedbacks: {let proc = get_proc_address("glCreateTransformFeedbacks"); if proc == null() {dummy_pfnglcreatetransformfeedbacksproc} else {unsafe{transmute(proc)}}},
			transformfeedbackbufferbase: {let proc = get_proc_address("glTransformFeedbackBufferBase"); if proc == null() {dummy_pfngltransformfeedbackbufferbaseproc} else {unsafe{transmute(proc)}}},
			transformfeedbackbufferrange: {let proc = get_proc_address("glTransformFeedbackBufferRange"); if proc == null() {dummy_pfngltransformfeedbackbufferrangeproc} else {unsafe{transmute(proc)}}},
			gettransformfeedbackiv: {let proc = get_proc_address("glGetTransformFeedbackiv"); if proc == null() {dummy_pfnglgettransformfeedbackivproc} else {unsafe{transmute(proc)}}},
			gettransformfeedbacki_v: {let proc = get_proc_address("glGetTransformFeedbacki_v"); if proc == null() {dummy_pfnglgettransformfeedbacki_vproc} else {unsafe{transmute(proc)}}},
			gettransformfeedbacki64_v: {let proc = get_proc_address("glGetTransformFeedbacki64_v"); if proc == null() {dummy_pfnglgettransformfeedbacki64_vproc} else {unsafe{transmute(proc)}}},
			createbuffers: {let proc = get_proc_address("glCreateBuffers"); if proc == null() {dummy_pfnglcreatebuffersproc} else {unsafe{transmute(proc)}}},
			namedbufferstorage: {let proc = get_proc_address("glNamedBufferStorage"); if proc == null() {dummy_pfnglnamedbufferstorageproc} else {unsafe{transmute(proc)}}},
			namedbufferdata: {let proc = get_proc_address("glNamedBufferData"); if proc == null() {dummy_pfnglnamedbufferdataproc} else {unsafe{transmute(proc)}}},
			namedbuffersubdata: {let proc = get_proc_address("glNamedBufferSubData"); if proc == null() {dummy_pfnglnamedbuffersubdataproc} else {unsafe{transmute(proc)}}},
			copynamedbuffersubdata: {let proc = get_proc_address("glCopyNamedBufferSubData"); if proc == null() {dummy_pfnglcopynamedbuffersubdataproc} else {unsafe{transmute(proc)}}},
			clearnamedbufferdata: {let proc = get_proc_address("glClearNamedBufferData"); if proc == null() {dummy_pfnglclearnamedbufferdataproc} else {unsafe{transmute(proc)}}},
			clearnamedbuffersubdata: {let proc = get_proc_address("glClearNamedBufferSubData"); if proc == null() {dummy_pfnglclearnamedbuffersubdataproc} else {unsafe{transmute(proc)}}},
			mapnamedbuffer: {let proc = get_proc_address("glMapNamedBuffer"); if proc == null() {dummy_pfnglmapnamedbufferproc} else {unsafe{transmute(proc)}}},
			mapnamedbufferrange: {let proc = get_proc_address("glMapNamedBufferRange"); if proc == null() {dummy_pfnglmapnamedbufferrangeproc} else {unsafe{transmute(proc)}}},
			unmapnamedbuffer: {let proc = get_proc_address("glUnmapNamedBuffer"); if proc == null() {dummy_pfnglunmapnamedbufferproc} else {unsafe{transmute(proc)}}},
			flushmappednamedbufferrange: {let proc = get_proc_address("glFlushMappedNamedBufferRange"); if proc == null() {dummy_pfnglflushmappednamedbufferrangeproc} else {unsafe{transmute(proc)}}},
			getnamedbufferparameteriv: {let proc = get_proc_address("glGetNamedBufferParameteriv"); if proc == null() {dummy_pfnglgetnamedbufferparameterivproc} else {unsafe{transmute(proc)}}},
			getnamedbufferparameteri64v: {let proc = get_proc_address("glGetNamedBufferParameteri64v"); if proc == null() {dummy_pfnglgetnamedbufferparameteri64vproc} else {unsafe{transmute(proc)}}},
			getnamedbufferpointerv: {let proc = get_proc_address("glGetNamedBufferPointerv"); if proc == null() {dummy_pfnglgetnamedbufferpointervproc} else {unsafe{transmute(proc)}}},
			getnamedbuffersubdata: {let proc = get_proc_address("glGetNamedBufferSubData"); if proc == null() {dummy_pfnglgetnamedbuffersubdataproc} else {unsafe{transmute(proc)}}},
			createframebuffers: {let proc = get_proc_address("glCreateFramebuffers"); if proc == null() {dummy_pfnglcreateframebuffersproc} else {unsafe{transmute(proc)}}},
			namedframebufferrenderbuffer: {let proc = get_proc_address("glNamedFramebufferRenderbuffer"); if proc == null() {dummy_pfnglnamedframebufferrenderbufferproc} else {unsafe{transmute(proc)}}},
			namedframebufferparameteri: {let proc = get_proc_address("glNamedFramebufferParameteri"); if proc == null() {dummy_pfnglnamedframebufferparameteriproc} else {unsafe{transmute(proc)}}},
			namedframebuffertexture: {let proc = get_proc_address("glNamedFramebufferTexture"); if proc == null() {dummy_pfnglnamedframebuffertextureproc} else {unsafe{transmute(proc)}}},
			namedframebuffertexturelayer: {let proc = get_proc_address("glNamedFramebufferTextureLayer"); if proc == null() {dummy_pfnglnamedframebuffertexturelayerproc} else {unsafe{transmute(proc)}}},
			namedframebufferdrawbuffer: {let proc = get_proc_address("glNamedFramebufferDrawBuffer"); if proc == null() {dummy_pfnglnamedframebufferdrawbufferproc} else {unsafe{transmute(proc)}}},
			namedframebufferdrawbuffers: {let proc = get_proc_address("glNamedFramebufferDrawBuffers"); if proc == null() {dummy_pfnglnamedframebufferdrawbuffersproc} else {unsafe{transmute(proc)}}},
			namedframebufferreadbuffer: {let proc = get_proc_address("glNamedFramebufferReadBuffer"); if proc == null() {dummy_pfnglnamedframebufferreadbufferproc} else {unsafe{transmute(proc)}}},
			invalidatenamedframebufferdata: {let proc = get_proc_address("glInvalidateNamedFramebufferData"); if proc == null() {dummy_pfnglinvalidatenamedframebufferdataproc} else {unsafe{transmute(proc)}}},
			invalidatenamedframebuffersubdata: {let proc = get_proc_address("glInvalidateNamedFramebufferSubData"); if proc == null() {dummy_pfnglinvalidatenamedframebuffersubdataproc} else {unsafe{transmute(proc)}}},
			clearnamedframebufferiv: {let proc = get_proc_address("glClearNamedFramebufferiv"); if proc == null() {dummy_pfnglclearnamedframebufferivproc} else {unsafe{transmute(proc)}}},
			clearnamedframebufferuiv: {let proc = get_proc_address("glClearNamedFramebufferuiv"); if proc == null() {dummy_pfnglclearnamedframebufferuivproc} else {unsafe{transmute(proc)}}},
			clearnamedframebufferfv: {let proc = get_proc_address("glClearNamedFramebufferfv"); if proc == null() {dummy_pfnglclearnamedframebufferfvproc} else {unsafe{transmute(proc)}}},
			clearnamedframebufferfi: {let proc = get_proc_address("glClearNamedFramebufferfi"); if proc == null() {dummy_pfnglclearnamedframebufferfiproc} else {unsafe{transmute(proc)}}},
			blitnamedframebuffer: {let proc = get_proc_address("glBlitNamedFramebuffer"); if proc == null() {dummy_pfnglblitnamedframebufferproc} else {unsafe{transmute(proc)}}},
			checknamedframebufferstatus: {let proc = get_proc_address("glCheckNamedFramebufferStatus"); if proc == null() {dummy_pfnglchecknamedframebufferstatusproc} else {unsafe{transmute(proc)}}},
			getnamedframebufferparameteriv: {let proc = get_proc_address("glGetNamedFramebufferParameteriv"); if proc == null() {dummy_pfnglgetnamedframebufferparameterivproc} else {unsafe{transmute(proc)}}},
			getnamedframebufferattachmentparameteriv: {let proc = get_proc_address("glGetNamedFramebufferAttachmentParameteriv"); if proc == null() {dummy_pfnglgetnamedframebufferattachmentparameterivproc} else {unsafe{transmute(proc)}}},
			createrenderbuffers: {let proc = get_proc_address("glCreateRenderbuffers"); if proc == null() {dummy_pfnglcreaterenderbuffersproc} else {unsafe{transmute(proc)}}},
			namedrenderbufferstorage: {let proc = get_proc_address("glNamedRenderbufferStorage"); if proc == null() {dummy_pfnglnamedrenderbufferstorageproc} else {unsafe{transmute(proc)}}},
			namedrenderbufferstoragemultisample: {let proc = get_proc_address("glNamedRenderbufferStorageMultisample"); if proc == null() {dummy_pfnglnamedrenderbufferstoragemultisampleproc} else {unsafe{transmute(proc)}}},
			getnamedrenderbufferparameteriv: {let proc = get_proc_address("glGetNamedRenderbufferParameteriv"); if proc == null() {dummy_pfnglgetnamedrenderbufferparameterivproc} else {unsafe{transmute(proc)}}},
			createtextures: {let proc = get_proc_address("glCreateTextures"); if proc == null() {dummy_pfnglcreatetexturesproc} else {unsafe{transmute(proc)}}},
			texturebuffer: {let proc = get_proc_address("glTextureBuffer"); if proc == null() {dummy_pfngltexturebufferproc} else {unsafe{transmute(proc)}}},
			texturebufferrange: {let proc = get_proc_address("glTextureBufferRange"); if proc == null() {dummy_pfngltexturebufferrangeproc} else {unsafe{transmute(proc)}}},
			texturestorage1d: {let proc = get_proc_address("glTextureStorage1D"); if proc == null() {dummy_pfngltexturestorage1dproc} else {unsafe{transmute(proc)}}},
			texturestorage2d: {let proc = get_proc_address("glTextureStorage2D"); if proc == null() {dummy_pfngltexturestorage2dproc} else {unsafe{transmute(proc)}}},
			texturestorage3d: {let proc = get_proc_address("glTextureStorage3D"); if proc == null() {dummy_pfngltexturestorage3dproc} else {unsafe{transmute(proc)}}},
			texturestorage2dmultisample: {let proc = get_proc_address("glTextureStorage2DMultisample"); if proc == null() {dummy_pfngltexturestorage2dmultisampleproc} else {unsafe{transmute(proc)}}},
			texturestorage3dmultisample: {let proc = get_proc_address("glTextureStorage3DMultisample"); if proc == null() {dummy_pfngltexturestorage3dmultisampleproc} else {unsafe{transmute(proc)}}},
			texturesubimage1d: {let proc = get_proc_address("glTextureSubImage1D"); if proc == null() {dummy_pfngltexturesubimage1dproc} else {unsafe{transmute(proc)}}},
			texturesubimage2d: {let proc = get_proc_address("glTextureSubImage2D"); if proc == null() {dummy_pfngltexturesubimage2dproc} else {unsafe{transmute(proc)}}},
			texturesubimage3d: {let proc = get_proc_address("glTextureSubImage3D"); if proc == null() {dummy_pfngltexturesubimage3dproc} else {unsafe{transmute(proc)}}},
			compressedtexturesubimage1d: {let proc = get_proc_address("glCompressedTextureSubImage1D"); if proc == null() {dummy_pfnglcompressedtexturesubimage1dproc} else {unsafe{transmute(proc)}}},
			compressedtexturesubimage2d: {let proc = get_proc_address("glCompressedTextureSubImage2D"); if proc == null() {dummy_pfnglcompressedtexturesubimage2dproc} else {unsafe{transmute(proc)}}},
			compressedtexturesubimage3d: {let proc = get_proc_address("glCompressedTextureSubImage3D"); if proc == null() {dummy_pfnglcompressedtexturesubimage3dproc} else {unsafe{transmute(proc)}}},
			copytexturesubimage1d: {let proc = get_proc_address("glCopyTextureSubImage1D"); if proc == null() {dummy_pfnglcopytexturesubimage1dproc} else {unsafe{transmute(proc)}}},
			copytexturesubimage2d: {let proc = get_proc_address("glCopyTextureSubImage2D"); if proc == null() {dummy_pfnglcopytexturesubimage2dproc} else {unsafe{transmute(proc)}}},
			copytexturesubimage3d: {let proc = get_proc_address("glCopyTextureSubImage3D"); if proc == null() {dummy_pfnglcopytexturesubimage3dproc} else {unsafe{transmute(proc)}}},
			textureparameterf: {let proc = get_proc_address("glTextureParameterf"); if proc == null() {dummy_pfngltextureparameterfproc} else {unsafe{transmute(proc)}}},
			textureparameterfv: {let proc = get_proc_address("glTextureParameterfv"); if proc == null() {dummy_pfngltextureparameterfvproc} else {unsafe{transmute(proc)}}},
			textureparameteri: {let proc = get_proc_address("glTextureParameteri"); if proc == null() {dummy_pfngltextureparameteriproc} else {unsafe{transmute(proc)}}},
			textureparameteriiv: {let proc = get_proc_address("glTextureParameterIiv"); if proc == null() {dummy_pfngltextureparameteriivproc} else {unsafe{transmute(proc)}}},
			textureparameteriuiv: {let proc = get_proc_address("glTextureParameterIuiv"); if proc == null() {dummy_pfngltextureparameteriuivproc} else {unsafe{transmute(proc)}}},
			textureparameteriv: {let proc = get_proc_address("glTextureParameteriv"); if proc == null() {dummy_pfngltextureparameterivproc} else {unsafe{transmute(proc)}}},
			generatetexturemipmap: {let proc = get_proc_address("glGenerateTextureMipmap"); if proc == null() {dummy_pfnglgeneratetexturemipmapproc} else {unsafe{transmute(proc)}}},
			bindtextureunit: {let proc = get_proc_address("glBindTextureUnit"); if proc == null() {dummy_pfnglbindtextureunitproc} else {unsafe{transmute(proc)}}},
			gettextureimage: {let proc = get_proc_address("glGetTextureImage"); if proc == null() {dummy_pfnglgettextureimageproc} else {unsafe{transmute(proc)}}},
			getcompressedtextureimage: {let proc = get_proc_address("glGetCompressedTextureImage"); if proc == null() {dummy_pfnglgetcompressedtextureimageproc} else {unsafe{transmute(proc)}}},
			gettexturelevelparameterfv: {let proc = get_proc_address("glGetTextureLevelParameterfv"); if proc == null() {dummy_pfnglgettexturelevelparameterfvproc} else {unsafe{transmute(proc)}}},
			gettexturelevelparameteriv: {let proc = get_proc_address("glGetTextureLevelParameteriv"); if proc == null() {dummy_pfnglgettexturelevelparameterivproc} else {unsafe{transmute(proc)}}},
			gettextureparameterfv: {let proc = get_proc_address("glGetTextureParameterfv"); if proc == null() {dummy_pfnglgettextureparameterfvproc} else {unsafe{transmute(proc)}}},
			gettextureparameteriiv: {let proc = get_proc_address("glGetTextureParameterIiv"); if proc == null() {dummy_pfnglgettextureparameteriivproc} else {unsafe{transmute(proc)}}},
			gettextureparameteriuiv: {let proc = get_proc_address("glGetTextureParameterIuiv"); if proc == null() {dummy_pfnglgettextureparameteriuivproc} else {unsafe{transmute(proc)}}},
			gettextureparameteriv: {let proc = get_proc_address("glGetTextureParameteriv"); if proc == null() {dummy_pfnglgettextureparameterivproc} else {unsafe{transmute(proc)}}},
			createvertexarrays: {let proc = get_proc_address("glCreateVertexArrays"); if proc == null() {dummy_pfnglcreatevertexarraysproc} else {unsafe{transmute(proc)}}},
			disablevertexarrayattrib: {let proc = get_proc_address("glDisableVertexArrayAttrib"); if proc == null() {dummy_pfngldisablevertexarrayattribproc} else {unsafe{transmute(proc)}}},
			enablevertexarrayattrib: {let proc = get_proc_address("glEnableVertexArrayAttrib"); if proc == null() {dummy_pfnglenablevertexarrayattribproc} else {unsafe{transmute(proc)}}},
			vertexarrayelementbuffer: {let proc = get_proc_address("glVertexArrayElementBuffer"); if proc == null() {dummy_pfnglvertexarrayelementbufferproc} else {unsafe{transmute(proc)}}},
			vertexarrayvertexbuffer: {let proc = get_proc_address("glVertexArrayVertexBuffer"); if proc == null() {dummy_pfnglvertexarrayvertexbufferproc} else {unsafe{transmute(proc)}}},
			vertexarrayvertexbuffers: {let proc = get_proc_address("glVertexArrayVertexBuffers"); if proc == null() {dummy_pfnglvertexarrayvertexbuffersproc} else {unsafe{transmute(proc)}}},
			vertexarrayattribbinding: {let proc = get_proc_address("glVertexArrayAttribBinding"); if proc == null() {dummy_pfnglvertexarrayattribbindingproc} else {unsafe{transmute(proc)}}},
			vertexarrayattribformat: {let proc = get_proc_address("glVertexArrayAttribFormat"); if proc == null() {dummy_pfnglvertexarrayattribformatproc} else {unsafe{transmute(proc)}}},
			vertexarrayattribiformat: {let proc = get_proc_address("glVertexArrayAttribIFormat"); if proc == null() {dummy_pfnglvertexarrayattribiformatproc} else {unsafe{transmute(proc)}}},
			vertexarrayattriblformat: {let proc = get_proc_address("glVertexArrayAttribLFormat"); if proc == null() {dummy_pfnglvertexarrayattriblformatproc} else {unsafe{transmute(proc)}}},
			vertexarraybindingdivisor: {let proc = get_proc_address("glVertexArrayBindingDivisor"); if proc == null() {dummy_pfnglvertexarraybindingdivisorproc} else {unsafe{transmute(proc)}}},
			getvertexarrayiv: {let proc = get_proc_address("glGetVertexArrayiv"); if proc == null() {dummy_pfnglgetvertexarrayivproc} else {unsafe{transmute(proc)}}},
			getvertexarrayindexediv: {let proc = get_proc_address("glGetVertexArrayIndexediv"); if proc == null() {dummy_pfnglgetvertexarrayindexedivproc} else {unsafe{transmute(proc)}}},
			getvertexarrayindexed64iv: {let proc = get_proc_address("glGetVertexArrayIndexed64iv"); if proc == null() {dummy_pfnglgetvertexarrayindexed64ivproc} else {unsafe{transmute(proc)}}},
			createsamplers: {let proc = get_proc_address("glCreateSamplers"); if proc == null() {dummy_pfnglcreatesamplersproc} else {unsafe{transmute(proc)}}},
			createprogrampipelines: {let proc = get_proc_address("glCreateProgramPipelines"); if proc == null() {dummy_pfnglcreateprogrampipelinesproc} else {unsafe{transmute(proc)}}},
			createqueries: {let proc = get_proc_address("glCreateQueries"); if proc == null() {dummy_pfnglcreatequeriesproc} else {unsafe{transmute(proc)}}},
			getquerybufferobjecti64v: {let proc = get_proc_address("glGetQueryBufferObjecti64v"); if proc == null() {dummy_pfnglgetquerybufferobjecti64vproc} else {unsafe{transmute(proc)}}},
			getquerybufferobjectiv: {let proc = get_proc_address("glGetQueryBufferObjectiv"); if proc == null() {dummy_pfnglgetquerybufferobjectivproc} else {unsafe{transmute(proc)}}},
			getquerybufferobjectui64v: {let proc = get_proc_address("glGetQueryBufferObjectui64v"); if proc == null() {dummy_pfnglgetquerybufferobjectui64vproc} else {unsafe{transmute(proc)}}},
			getquerybufferobjectuiv: {let proc = get_proc_address("glGetQueryBufferObjectuiv"); if proc == null() {dummy_pfnglgetquerybufferobjectuivproc} else {unsafe{transmute(proc)}}},
			memorybarrierbyregion: {let proc = get_proc_address("glMemoryBarrierByRegion"); if proc == null() {dummy_pfnglmemorybarrierbyregionproc} else {unsafe{transmute(proc)}}},
			gettexturesubimage: {let proc = get_proc_address("glGetTextureSubImage"); if proc == null() {dummy_pfnglgettexturesubimageproc} else {unsafe{transmute(proc)}}},
			getcompressedtexturesubimage: {let proc = get_proc_address("glGetCompressedTextureSubImage"); if proc == null() {dummy_pfnglgetcompressedtexturesubimageproc} else {unsafe{transmute(proc)}}},
			getgraphicsresetstatus: {let proc = get_proc_address("glGetGraphicsResetStatus"); if proc == null() {dummy_pfnglgetgraphicsresetstatusproc} else {unsafe{transmute(proc)}}},
			getncompressedteximage: {let proc = get_proc_address("glGetnCompressedTexImage"); if proc == null() {dummy_pfnglgetncompressedteximageproc} else {unsafe{transmute(proc)}}},
			getnteximage: {let proc = get_proc_address("glGetnTexImage"); if proc == null() {dummy_pfnglgetnteximageproc} else {unsafe{transmute(proc)}}},
			getnuniformdv: {let proc = get_proc_address("glGetnUniformdv"); if proc == null() {dummy_pfnglgetnuniformdvproc} else {unsafe{transmute(proc)}}},
			getnuniformfv: {let proc = get_proc_address("glGetnUniformfv"); if proc == null() {dummy_pfnglgetnuniformfvproc} else {unsafe{transmute(proc)}}},
			getnuniformiv: {let proc = get_proc_address("glGetnUniformiv"); if proc == null() {dummy_pfnglgetnuniformivproc} else {unsafe{transmute(proc)}}},
			getnuniformuiv: {let proc = get_proc_address("glGetnUniformuiv"); if proc == null() {dummy_pfnglgetnuniformuivproc} else {unsafe{transmute(proc)}}},
			readnpixels: {let proc = get_proc_address("glReadnPixels"); if proc == null() {dummy_pfnglreadnpixelsproc} else {unsafe{transmute(proc)}}},
			getnmapdv: {let proc = get_proc_address("glGetnMapdv"); if proc == null() {dummy_pfnglgetnmapdvproc} else {unsafe{transmute(proc)}}},
			getnmapfv: {let proc = get_proc_address("glGetnMapfv"); if proc == null() {dummy_pfnglgetnmapfvproc} else {unsafe{transmute(proc)}}},
			getnmapiv: {let proc = get_proc_address("glGetnMapiv"); if proc == null() {dummy_pfnglgetnmapivproc} else {unsafe{transmute(proc)}}},
			getnpixelmapfv: {let proc = get_proc_address("glGetnPixelMapfv"); if proc == null() {dummy_pfnglgetnpixelmapfvproc} else {unsafe{transmute(proc)}}},
			getnpixelmapuiv: {let proc = get_proc_address("glGetnPixelMapuiv"); if proc == null() {dummy_pfnglgetnpixelmapuivproc} else {unsafe{transmute(proc)}}},
			getnpixelmapusv: {let proc = get_proc_address("glGetnPixelMapusv"); if proc == null() {dummy_pfnglgetnpixelmapusvproc} else {unsafe{transmute(proc)}}},
			getnpolygonstipple: {let proc = get_proc_address("glGetnPolygonStipple"); if proc == null() {dummy_pfnglgetnpolygonstippleproc} else {unsafe{transmute(proc)}}},
			getncolortable: {let proc = get_proc_address("glGetnColorTable"); if proc == null() {dummy_pfnglgetncolortableproc} else {unsafe{transmute(proc)}}},
			getnconvolutionfilter: {let proc = get_proc_address("glGetnConvolutionFilter"); if proc == null() {dummy_pfnglgetnconvolutionfilterproc} else {unsafe{transmute(proc)}}},
			getnseparablefilter: {let proc = get_proc_address("glGetnSeparableFilter"); if proc == null() {dummy_pfnglgetnseparablefilterproc} else {unsafe{transmute(proc)}}},
			getnhistogram: {let proc = get_proc_address("glGetnHistogram"); if proc == null() {dummy_pfnglgetnhistogramproc} else {unsafe{transmute(proc)}}},
			getnminmax: {let proc = get_proc_address("glGetnMinmax"); if proc == null() {dummy_pfnglgetnminmaxproc} else {unsafe{transmute(proc)}}},
			texturebarrier: {let proc = get_proc_address("glTextureBarrier"); if proc == null() {dummy_pfngltexturebarrierproc} else {unsafe{transmute(proc)}}},
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
			clipcontrol: dummy_pfnglclipcontrolproc,
			createtransformfeedbacks: dummy_pfnglcreatetransformfeedbacksproc,
			transformfeedbackbufferbase: dummy_pfngltransformfeedbackbufferbaseproc,
			transformfeedbackbufferrange: dummy_pfngltransformfeedbackbufferrangeproc,
			gettransformfeedbackiv: dummy_pfnglgettransformfeedbackivproc,
			gettransformfeedbacki_v: dummy_pfnglgettransformfeedbacki_vproc,
			gettransformfeedbacki64_v: dummy_pfnglgettransformfeedbacki64_vproc,
			createbuffers: dummy_pfnglcreatebuffersproc,
			namedbufferstorage: dummy_pfnglnamedbufferstorageproc,
			namedbufferdata: dummy_pfnglnamedbufferdataproc,
			namedbuffersubdata: dummy_pfnglnamedbuffersubdataproc,
			copynamedbuffersubdata: dummy_pfnglcopynamedbuffersubdataproc,
			clearnamedbufferdata: dummy_pfnglclearnamedbufferdataproc,
			clearnamedbuffersubdata: dummy_pfnglclearnamedbuffersubdataproc,
			mapnamedbuffer: dummy_pfnglmapnamedbufferproc,
			mapnamedbufferrange: dummy_pfnglmapnamedbufferrangeproc,
			unmapnamedbuffer: dummy_pfnglunmapnamedbufferproc,
			flushmappednamedbufferrange: dummy_pfnglflushmappednamedbufferrangeproc,
			getnamedbufferparameteriv: dummy_pfnglgetnamedbufferparameterivproc,
			getnamedbufferparameteri64v: dummy_pfnglgetnamedbufferparameteri64vproc,
			getnamedbufferpointerv: dummy_pfnglgetnamedbufferpointervproc,
			getnamedbuffersubdata: dummy_pfnglgetnamedbuffersubdataproc,
			createframebuffers: dummy_pfnglcreateframebuffersproc,
			namedframebufferrenderbuffer: dummy_pfnglnamedframebufferrenderbufferproc,
			namedframebufferparameteri: dummy_pfnglnamedframebufferparameteriproc,
			namedframebuffertexture: dummy_pfnglnamedframebuffertextureproc,
			namedframebuffertexturelayer: dummy_pfnglnamedframebuffertexturelayerproc,
			namedframebufferdrawbuffer: dummy_pfnglnamedframebufferdrawbufferproc,
			namedframebufferdrawbuffers: dummy_pfnglnamedframebufferdrawbuffersproc,
			namedframebufferreadbuffer: dummy_pfnglnamedframebufferreadbufferproc,
			invalidatenamedframebufferdata: dummy_pfnglinvalidatenamedframebufferdataproc,
			invalidatenamedframebuffersubdata: dummy_pfnglinvalidatenamedframebuffersubdataproc,
			clearnamedframebufferiv: dummy_pfnglclearnamedframebufferivproc,
			clearnamedframebufferuiv: dummy_pfnglclearnamedframebufferuivproc,
			clearnamedframebufferfv: dummy_pfnglclearnamedframebufferfvproc,
			clearnamedframebufferfi: dummy_pfnglclearnamedframebufferfiproc,
			blitnamedframebuffer: dummy_pfnglblitnamedframebufferproc,
			checknamedframebufferstatus: dummy_pfnglchecknamedframebufferstatusproc,
			getnamedframebufferparameteriv: dummy_pfnglgetnamedframebufferparameterivproc,
			getnamedframebufferattachmentparameteriv: dummy_pfnglgetnamedframebufferattachmentparameterivproc,
			createrenderbuffers: dummy_pfnglcreaterenderbuffersproc,
			namedrenderbufferstorage: dummy_pfnglnamedrenderbufferstorageproc,
			namedrenderbufferstoragemultisample: dummy_pfnglnamedrenderbufferstoragemultisampleproc,
			getnamedrenderbufferparameteriv: dummy_pfnglgetnamedrenderbufferparameterivproc,
			createtextures: dummy_pfnglcreatetexturesproc,
			texturebuffer: dummy_pfngltexturebufferproc,
			texturebufferrange: dummy_pfngltexturebufferrangeproc,
			texturestorage1d: dummy_pfngltexturestorage1dproc,
			texturestorage2d: dummy_pfngltexturestorage2dproc,
			texturestorage3d: dummy_pfngltexturestorage3dproc,
			texturestorage2dmultisample: dummy_pfngltexturestorage2dmultisampleproc,
			texturestorage3dmultisample: dummy_pfngltexturestorage3dmultisampleproc,
			texturesubimage1d: dummy_pfngltexturesubimage1dproc,
			texturesubimage2d: dummy_pfngltexturesubimage2dproc,
			texturesubimage3d: dummy_pfngltexturesubimage3dproc,
			compressedtexturesubimage1d: dummy_pfnglcompressedtexturesubimage1dproc,
			compressedtexturesubimage2d: dummy_pfnglcompressedtexturesubimage2dproc,
			compressedtexturesubimage3d: dummy_pfnglcompressedtexturesubimage3dproc,
			copytexturesubimage1d: dummy_pfnglcopytexturesubimage1dproc,
			copytexturesubimage2d: dummy_pfnglcopytexturesubimage2dproc,
			copytexturesubimage3d: dummy_pfnglcopytexturesubimage3dproc,
			textureparameterf: dummy_pfngltextureparameterfproc,
			textureparameterfv: dummy_pfngltextureparameterfvproc,
			textureparameteri: dummy_pfngltextureparameteriproc,
			textureparameteriiv: dummy_pfngltextureparameteriivproc,
			textureparameteriuiv: dummy_pfngltextureparameteriuivproc,
			textureparameteriv: dummy_pfngltextureparameterivproc,
			generatetexturemipmap: dummy_pfnglgeneratetexturemipmapproc,
			bindtextureunit: dummy_pfnglbindtextureunitproc,
			gettextureimage: dummy_pfnglgettextureimageproc,
			getcompressedtextureimage: dummy_pfnglgetcompressedtextureimageproc,
			gettexturelevelparameterfv: dummy_pfnglgettexturelevelparameterfvproc,
			gettexturelevelparameteriv: dummy_pfnglgettexturelevelparameterivproc,
			gettextureparameterfv: dummy_pfnglgettextureparameterfvproc,
			gettextureparameteriiv: dummy_pfnglgettextureparameteriivproc,
			gettextureparameteriuiv: dummy_pfnglgettextureparameteriuivproc,
			gettextureparameteriv: dummy_pfnglgettextureparameterivproc,
			createvertexarrays: dummy_pfnglcreatevertexarraysproc,
			disablevertexarrayattrib: dummy_pfngldisablevertexarrayattribproc,
			enablevertexarrayattrib: dummy_pfnglenablevertexarrayattribproc,
			vertexarrayelementbuffer: dummy_pfnglvertexarrayelementbufferproc,
			vertexarrayvertexbuffer: dummy_pfnglvertexarrayvertexbufferproc,
			vertexarrayvertexbuffers: dummy_pfnglvertexarrayvertexbuffersproc,
			vertexarrayattribbinding: dummy_pfnglvertexarrayattribbindingproc,
			vertexarrayattribformat: dummy_pfnglvertexarrayattribformatproc,
			vertexarrayattribiformat: dummy_pfnglvertexarrayattribiformatproc,
			vertexarrayattriblformat: dummy_pfnglvertexarrayattriblformatproc,
			vertexarraybindingdivisor: dummy_pfnglvertexarraybindingdivisorproc,
			getvertexarrayiv: dummy_pfnglgetvertexarrayivproc,
			getvertexarrayindexediv: dummy_pfnglgetvertexarrayindexedivproc,
			getvertexarrayindexed64iv: dummy_pfnglgetvertexarrayindexed64ivproc,
			createsamplers: dummy_pfnglcreatesamplersproc,
			createprogrampipelines: dummy_pfnglcreateprogrampipelinesproc,
			createqueries: dummy_pfnglcreatequeriesproc,
			getquerybufferobjecti64v: dummy_pfnglgetquerybufferobjecti64vproc,
			getquerybufferobjectiv: dummy_pfnglgetquerybufferobjectivproc,
			getquerybufferobjectui64v: dummy_pfnglgetquerybufferobjectui64vproc,
			getquerybufferobjectuiv: dummy_pfnglgetquerybufferobjectuivproc,
			memorybarrierbyregion: dummy_pfnglmemorybarrierbyregionproc,
			gettexturesubimage: dummy_pfnglgettexturesubimageproc,
			getcompressedtexturesubimage: dummy_pfnglgetcompressedtexturesubimageproc,
			getgraphicsresetstatus: dummy_pfnglgetgraphicsresetstatusproc,
			getncompressedteximage: dummy_pfnglgetncompressedteximageproc,
			getnteximage: dummy_pfnglgetnteximageproc,
			getnuniformdv: dummy_pfnglgetnuniformdvproc,
			getnuniformfv: dummy_pfnglgetnuniformfvproc,
			getnuniformiv: dummy_pfnglgetnuniformivproc,
			getnuniformuiv: dummy_pfnglgetnuniformuivproc,
			readnpixels: dummy_pfnglreadnpixelsproc,
			getnmapdv: dummy_pfnglgetnmapdvproc,
			getnmapfv: dummy_pfnglgetnmapfvproc,
			getnmapiv: dummy_pfnglgetnmapivproc,
			getnpixelmapfv: dummy_pfnglgetnpixelmapfvproc,
			getnpixelmapuiv: dummy_pfnglgetnpixelmapuivproc,
			getnpixelmapusv: dummy_pfnglgetnpixelmapusvproc,
			getnpolygonstipple: dummy_pfnglgetnpolygonstippleproc,
			getncolortable: dummy_pfnglgetncolortableproc,
			getnconvolutionfilter: dummy_pfnglgetnconvolutionfilterproc,
			getnseparablefilter: dummy_pfnglgetnseparablefilterproc,
			getnhistogram: dummy_pfnglgetnhistogramproc,
			getnminmax: dummy_pfnglgetnminmaxproc,
			texturebarrier: dummy_pfngltexturebarrierproc,
		}
	}
}

type PFNGLSPECIALIZESHADERPROC = extern "system" fn(GLuint, *const GLchar, GLuint, *const GLuint, *const GLuint);
type PFNGLMULTIDRAWARRAYSINDIRECTCOUNTPROC = extern "system" fn(GLenum, *const c_void, GLintptr, GLsizei, GLsizei);
type PFNGLMULTIDRAWELEMENTSINDIRECTCOUNTPROC = extern "system" fn(GLenum, GLenum, *const c_void, GLintptr, GLsizei, GLsizei);
type PFNGLPOLYGONOFFSETCLAMPPROC = extern "system" fn(GLfloat, GLfloat, GLfloat);
extern "system" fn dummy_pfnglspecializeshaderproc (_: GLuint, _: *const GLchar, _: GLuint, _: *const GLuint, _: *const GLuint) {
	panic!("OpenGL Function pointer of `glSpecializeShader()` is NULL");
}
extern "system" fn dummy_pfnglmultidrawarraysindirectcountproc (_: GLenum, _: *const c_void, _: GLintptr, _: GLsizei, _: GLsizei) {
	panic!("OpenGL Function pointer of `glMultiDrawArraysIndirectCount()` is NULL");
}
extern "system" fn dummy_pfnglmultidrawelementsindirectcountproc (_: GLenum, _: GLenum, _: *const c_void, _: GLintptr, _: GLsizei, _: GLsizei) {
	panic!("OpenGL Function pointer of `glMultiDrawElementsIndirectCount()` is NULL");
}
extern "system" fn dummy_pfnglpolygonoffsetclampproc (_: GLfloat, _: GLfloat, _: GLfloat) {
	panic!("OpenGL Function pointer of `glPolygonOffsetClamp()` is NULL");
}
pub const GL_SHADER_BINARY_FORMAT_SPIR_V: GLenum = 0x9551;
pub const GL_SPIR_V_BINARY: GLenum = 0x9552;
pub const GL_PARAMETER_BUFFER: GLenum = 0x80EE;
pub const GL_PARAMETER_BUFFER_BINDING: GLenum = 0x80EF;
pub const GL_CONTEXT_FLAG_NO_ERROR_BIT: GLbitfield = 0x00000008;
pub const GL_VERTICES_SUBMITTED: GLenum = 0x82EE;
pub const GL_PRIMITIVES_SUBMITTED: GLenum = 0x82EF;
pub const GL_VERTEX_SHADER_INVOCATIONS: GLenum = 0x82F0;
pub const GL_TESS_CONTROL_SHADER_PATCHES: GLenum = 0x82F1;
pub const GL_TESS_EVALUATION_SHADER_INVOCATIONS: GLenum = 0x82F2;
pub const GL_GEOMETRY_SHADER_PRIMITIVES_EMITTED: GLenum = 0x82F3;
pub const GL_FRAGMENT_SHADER_INVOCATIONS: GLenum = 0x82F4;
pub const GL_COMPUTE_SHADER_INVOCATIONS: GLenum = 0x82F5;
pub const GL_CLIPPING_INPUT_PRIMITIVES: GLenum = 0x82F6;
pub const GL_CLIPPING_OUTPUT_PRIMITIVES: GLenum = 0x82F7;
pub const GL_POLYGON_OFFSET_CLAMP: GLenum = 0x8E1B;
pub const GL_SPIR_V_EXTENSIONS: GLenum = 0x9553;
pub const GL_NUM_SPIR_V_EXTENSIONS: GLenum = 0x9554;
pub const GL_TEXTURE_MAX_ANISOTROPY: GLenum = 0x84FE;
pub const GL_MAX_TEXTURE_MAX_ANISOTROPY: GLenum = 0x84FF;
pub const GL_TRANSFORM_FEEDBACK_OVERFLOW: GLenum = 0x82EC;
pub const GL_TRANSFORM_FEEDBACK_STREAM_OVERFLOW: GLenum = 0x82ED;

pub trait GL_4_6 {
	fn glSpecializeShader(&self, shader: GLuint, pEntryPoint: *const GLchar, numSpecializationConstants: GLuint, pConstantIndex: *const GLuint, pConstantValue: *const GLuint);
	fn glMultiDrawArraysIndirectCount(&self, mode: GLenum, indirect: *const c_void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei);
	fn glMultiDrawElementsIndirectCount(&self, mode: GLenum, type_: GLenum, indirect: *const c_void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei);
	fn glPolygonOffsetClamp(&self, factor: GLfloat, units: GLfloat, clamp: GLfloat);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
	pub fn new(base: impl GL_1_0, mut get_proc_address: impl FnMut(&'static str) -> *const c_void) -> Self {
		let (_spec, major, minor, release) = base.get_version();
		let bigver = major.clamp(0, 100) * 10000 + minor.clamp(0, 99) * 100 + release.clamp(0, 99);
		if bigver < 40600 {
			return Self::default();
		}
		Self {
			available: true,
			specializeshader: {let proc = get_proc_address("glSpecializeShader"); if proc == null() {dummy_pfnglspecializeshaderproc} else {unsafe{transmute(proc)}}},
			multidrawarraysindirectcount: {let proc = get_proc_address("glMultiDrawArraysIndirectCount"); if proc == null() {dummy_pfnglmultidrawarraysindirectcountproc} else {unsafe{transmute(proc)}}},
			multidrawelementsindirectcount: {let proc = get_proc_address("glMultiDrawElementsIndirectCount"); if proc == null() {dummy_pfnglmultidrawelementsindirectcountproc} else {unsafe{transmute(proc)}}},
			polygonoffsetclamp: {let proc = get_proc_address("glPolygonOffsetClamp"); if proc == null() {dummy_pfnglpolygonoffsetclampproc} else {unsafe{transmute(proc)}}},
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
			specializeshader: dummy_pfnglspecializeshaderproc,
			multidrawarraysindirectcount: dummy_pfnglmultidrawarraysindirectcountproc,
			multidrawelementsindirectcount: dummy_pfnglmultidrawelementsindirectcountproc,
			polygonoffsetclamp: dummy_pfnglpolygonoffsetclampproc,
		}
	}
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
	pub fn new(mut get_proc_address: impl FnMut(&'static str) -> *const c_void) -> Self {
		let version_1_0 = Version10::new(&mut get_proc_address);
		if !version_1_0.available {
			return Self::default();
		}
		Self {
			version_1_0,
			version_1_1: Version11::new(version_1_0, &mut get_proc_address),
			version_1_2: Version12::new(version_1_0, &mut get_proc_address),
			version_1_3: Version13::new(version_1_0, &mut get_proc_address),
			version_1_4: Version14::new(version_1_0, &mut get_proc_address),
			version_1_5: Version15::new(version_1_0, &mut get_proc_address),
			version_2_0: Version20::new(version_1_0, &mut get_proc_address),
			version_2_1: Version21::new(version_1_0, &mut get_proc_address),
			version_3_0: Version30::new(version_1_0, &mut get_proc_address),
			version_3_1: Version31::new(version_1_0, &mut get_proc_address),
			version_3_2: Version32::new(version_1_0, &mut get_proc_address),
			version_3_3: Version33::new(version_1_0, &mut get_proc_address),
			version_4_0: Version40::new(version_1_0, &mut get_proc_address),
			version_4_1: Version41::new(version_1_0, &mut get_proc_address),
			version_4_2: Version42::new(version_1_0, &mut get_proc_address),
			version_4_3: Version43::new(version_1_0, &mut get_proc_address),
			version_4_4: Version44::new(version_1_0, &mut get_proc_address),
			version_4_5: Version45::new(version_1_0, &mut get_proc_address),
			version_4_6: Version46::new(version_1_0, &mut get_proc_address),
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

