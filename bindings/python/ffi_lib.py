from __future__ import annotations
import ctypes
import typing

T = typing.TypeVar("T")
c_lib = None

def init_lib(path):
    """Initializes the native library. Must be called at least once before anything else."""
    global c_lib
    c_lib = ctypes.cdll.LoadLibrary(path)

    c_lib.my_function.argtypes = [Vec2]
    c_lib.vec3_new.argtypes = [ctypes.c_int32, ctypes.c_int32, ctypes.c_int32]
    c_lib.add.argtypes = [ctypes.POINTER(Vec3), ctypes.c_int32, ctypes.c_int32, ctypes.c_int32]
    c_lib.add_reverse_args.argtypes = [ctypes.c_int32, ctypes.c_int32, ctypes.c_int32, ctypes.POINTER(Vec3)]
    c_lib.dot.argtypes = [ctypes.POINTER(Vec3), ctypes.POINTER(Vec3)]
    c_lib.cross.argtypes = [ctypes.POINTER(Vec3), ctypes.POINTER(Vec3)]
    c_lib.normalize.argtypes = [ctypes.POINTER(Vec3)]
    c_lib.hello.argtypes = [ctypes.POINTER(ctypes.c_int8)]
    c_lib.benchmark_rust.argtypes = []
    c_lib.benchmark_rust_async.argtypes = []
    c_lib.simple_service_destroy.argtypes = [ctypes.POINTER(ctypes.c_void_p)]
    c_lib.simple_service_new_with.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.c_uint32]
    c_lib.vec4_destroy.argtypes = [ctypes.POINTER(ctypes.c_void_p)]
    c_lib.vec4_new.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.c_float, ctypes.c_float, ctypes.c_float, ctypes.c_float]
    c_lib.vec4_dot.argtypes = [ctypes.c_void_p, ctypes.c_void_p]
    c_lib.vec4_get_x.argtypes = [ctypes.c_void_p]
    c_lib.vec4_get_y.argtypes = [ctypes.c_void_p]
    c_lib.vec4_get_z.argtypes = [ctypes.c_void_p]
    c_lib.vec4_get_w.argtypes = [ctypes.c_void_p]
    c_lib.vec4_set_x.argtypes = [ctypes.c_void_p, ctypes.c_float]
    c_lib.vec4_set_y.argtypes = [ctypes.c_void_p, ctypes.c_float]
    c_lib.vec4_set_z.argtypes = [ctypes.c_void_p, ctypes.c_float]
    c_lib.vec4_set_w.argtypes = [ctypes.c_void_p, ctypes.c_float]
    c_lib.init_logger.argtypes = []

    c_lib.my_function.restype = Vec2
    c_lib.vec3_new.restype = ctypes.POINTER(Vec3)
    c_lib.add.restype = ctypes.c_int32
    c_lib.add_reverse_args.restype = ctypes.c_int32
    c_lib.dot.restype = ctypes.c_int32
    c_lib.cross.restype = Vec3
    c_lib.normalize.restype = Vec3
    c_lib.benchmark_rust.restype = ctypes.c_double
    c_lib.benchmark_rust_async.restype = ctypes.c_double
    c_lib.simple_service_destroy.restype = ctypes.c_int
    c_lib.simple_service_new_with.restype = ctypes.c_int
    c_lib.vec4_destroy.restype = ctypes.c_int
    c_lib.vec4_new.restype = ctypes.c_int
    c_lib.vec4_dot.restype = ctypes.c_float
    c_lib.vec4_get_x.restype = ctypes.c_float
    c_lib.vec4_get_y.restype = ctypes.c_float
    c_lib.vec4_get_z.restype = ctypes.c_float
    c_lib.vec4_get_w.restype = ctypes.c_float

    c_lib.simple_service_destroy.errcheck = lambda rval, _fptr, _args: _errcheck(rval, 0)
    c_lib.simple_service_new_with.errcheck = lambda rval, _fptr, _args: _errcheck(rval, 0)
    c_lib.vec4_destroy.errcheck = lambda rval, _fptr, _args: _errcheck(rval, 0)
    c_lib.vec4_new.errcheck = lambda rval, _fptr, _args: _errcheck(rval, 0)


def my_function(input: Vec2) -> Vec2:
    """ Function using the type."""
    return c_lib.my_function(input)

def vec3_new(x: int, y: int, z: int) -> ctypes.POINTER(Vec3):
    return c_lib.vec3_new(x, y, z)

def add(slf: ctypes.POINTER(Vec3), x: int, y: int, z: int) -> int:
    return c_lib.add(slf, x, y, z)

def add_reverse_args(x: int, y: int, z: int, slf: ctypes.POINTER(Vec3)) -> int:
    return c_lib.add_reverse_args(x, y, z, slf)

def dot(slf: ctypes.POINTER(Vec3), other: ctypes.POINTER(Vec3)) -> int:
    return c_lib.dot(slf, other)

def cross(slf: ctypes.POINTER(Vec3), other: ctypes.POINTER(Vec3)) -> Vec3:
    return c_lib.cross(slf, other)

def normalize(slf: ctypes.POINTER(Vec3)) -> Vec3:
    return c_lib.normalize(slf)

def hello(name: ctypes.POINTER(ctypes.c_int8)):
    return c_lib.hello(name)

def benchmark_rust() -> float:
    return c_lib.benchmark_rust()

def benchmark_rust_async() -> float:
    return c_lib.benchmark_rust_async()

def init_logger():
    return c_lib.init_logger()





TRUE = ctypes.c_uint8(1)
FALSE = ctypes.c_uint8(0)


def _errcheck(returned, success):
    """Checks for FFIErrors and converts them to an exception."""
    if returned == success: return
    else: raise Exception(f"Function returned error: {returned}")


class CallbackVars(object):
    """Helper to be used `lambda x: setattr(cv, "x", x)` when getting values from callbacks."""
    def __str__(self):
        rval = ""
        for var in  filter(lambda x: "__" not in x, dir(self)):
            rval += f"{var}: {getattr(self, var)}"
        return rval


class _Iter(object):
    """Helper for slice iterators."""
    def __init__(self, target):
        self.i = 0
        self.target = target

    def __iter__(self):
        self.i = 0
        return self

    def __next__(self):
        if self.i >= self.target.len:
            raise StopIteration()
        rval = self.target[self.i]
        self.i += 1
        return rval


class FFIError:
    Ok = 0
    Null = 100
    Panic = 200
    Fail = 300


class Vec2(ctypes.Structure):
    """ A simple type in our FFI layer."""

    # These fields represent the underlying C data layout
    _fields_ = [
        ("x", ctypes.c_float),
        ("y", ctypes.c_float),
    ]

    def __init__(self, x: float = None, y: float = None):
        if x is not None:
            self.x = x
        if y is not None:
            self.y = y

    @property
    def x(self) -> float:
        return ctypes.Structure.__get__(self, "x")

    @x.setter
    def x(self, value: float):
        return ctypes.Structure.__set__(self, "x", value)

    @property
    def y(self) -> float:
        return ctypes.Structure.__get__(self, "y")

    @y.setter
    def y(self, value: float):
        return ctypes.Structure.__set__(self, "y", value)


class Vec3(ctypes.Structure):
    """ A basic vector 3 in our FFI layer using integers."""

    # These fields represent the underlying C data layout
    _fields_ = [
        ("x", ctypes.c_int32),
        ("y", ctypes.c_int32),
        ("z", ctypes.c_int32),
    ]

    def __init__(self, x: int = None, y: int = None, z: int = None):
        if x is not None:
            self.x = x
        if y is not None:
            self.y = y
        if z is not None:
            self.z = z

    @property
    def x(self) -> int:
        return ctypes.Structure.__get__(self, "x")

    @x.setter
    def x(self, value: int):
        return ctypes.Structure.__set__(self, "x", value)

    @property
    def y(self) -> int:
        return ctypes.Structure.__get__(self, "y")

    @y.setter
    def y(self, value: int):
        return ctypes.Structure.__set__(self, "y", value)

    @property
    def z(self) -> int:
        return ctypes.Structure.__get__(self, "z")

    @z.setter
    def z(self, value: int):
        return ctypes.Structure.__set__(self, "z", value)




class callbacks:
    """Helpers to define callbacks."""


class SimpleService:
    __api_lock = object()

    def __init__(self, api_lock, ctx):
        assert(api_lock == SimpleService.__api_lock), "You must create this with a static constructor." 
        self._ctx = ctx

    @property
    def _as_parameter_(self):
        return self._ctx

    @staticmethod
    def new_with(some_value: int) -> SimpleService:
        """"""
        ctx = ctypes.c_void_p()
        c_lib.simple_service_new_with(ctx, some_value)
        self = SimpleService(SimpleService.__api_lock, ctx)
        return self

    def __del__(self):
        c_lib.simple_service_destroy(self._ctx, )


class Vec4:
    __api_lock = object()

    def __init__(self, api_lock, ctx):
        assert(api_lock == Vec4.__api_lock), "You must create this with a static constructor." 
        self._ctx = ctx

    @property
    def _as_parameter_(self):
        return self._ctx

    @staticmethod
    def new(x: float, y: float, z: float, w: float) -> Vec4:
        """"""
        ctx = ctypes.c_void_p()
        c_lib.vec4_new(ctx, x, y, z, w)
        self = Vec4(Vec4.__api_lock, ctx)
        return self

    def __del__(self):
        c_lib.vec4_destroy(self._ctx, )
    def dot(self, other: ctypes.c_void_p) -> float:
        """"""
        return c_lib.vec4_dot(self._ctx, other)

    def get_x(self, ) -> float:
        """"""
        return c_lib.vec4_get_x(self._ctx, )

    def get_y(self, ) -> float:
        """"""
        return c_lib.vec4_get_y(self._ctx, )

    def get_z(self, ) -> float:
        """"""
        return c_lib.vec4_get_z(self._ctx, )

    def get_w(self, ) -> float:
        """"""
        return c_lib.vec4_get_w(self._ctx, )

    def set_x(self, value: float):
        """"""
        return c_lib.vec4_set_x(self._ctx, value)

    def set_y(self, value: float):
        """"""
        return c_lib.vec4_set_y(self._ctx, value)

    def set_z(self, value: float):
        """"""
        return c_lib.vec4_set_z(self._ctx, value)

    def set_w(self, value: float):
        """"""
        return c_lib.vec4_set_w(self._ctx, value)



