/* ----------------------------------------------------------------------------
 * This file was automatically generated by SWIG (https://www.swig.org).
 * Version 4.1.1
 *
 * Do not make changes to this file unless you know what you are doing - modify
 * the SWIG interface file instead.
 * ----------------------------------------------------------------------------- */

package com.example;

public class Vec4 {
  private transient long swigCPtr;
  protected transient boolean swigCMemOwn;

  protected Vec4(long cPtr, boolean cMemoryOwn) {
    swigCMemOwn = cMemoryOwn;
    swigCPtr = cPtr;
  }

  protected static long getCPtr(Vec4 obj) {
    return (obj == null) ? 0 : obj.swigCPtr;
  }

  protected static long swigRelease(Vec4 obj) {
    long ptr = 0;
    if (obj != null) {
      if (!obj.swigCMemOwn)
        throw new RuntimeException("Cannot release ownership as memory is not owned");
      ptr = obj.swigCPtr;
      obj.swigCMemOwn = false;
      obj.delete();
    }
    return ptr;
  }

  @SuppressWarnings("deprecation")
  protected void finalize() {
    delete();
  }

  public synchronized void delete() {
    if (swigCPtr != 0) {
      if (swigCMemOwn) {
        swigCMemOwn = false;
        ffi_libJNI.delete_Vec4(swigCPtr);
      }
      swigCPtr = 0;
    }
  }

  public Vec4() {
    this(ffi_libJNI.new_Vec4(), true);
  }

  public static Vec4 New(float x, float y, float z, float w) {
    return new Vec4(ffi_libJNI.Vec4_New(x, y, z, w), true);
  }

  public void Dispose() {
    ffi_libJNI.Vec4_Dispose(swigCPtr, this);
  }

  public float Dot(SWIGTYPE_p_vec4 other) {
    return ffi_libJNI.Vec4_Dot(swigCPtr, this, SWIGTYPE_p_vec4.getCPtr(other));
  }

  public float GetX() {
    return ffi_libJNI.Vec4_GetX(swigCPtr, this);
  }

  public float GetY() {
    return ffi_libJNI.Vec4_GetY(swigCPtr, this);
  }

  public float GetZ() {
    return ffi_libJNI.Vec4_GetZ(swigCPtr, this);
  }

  public float GetW() {
    return ffi_libJNI.Vec4_GetW(swigCPtr, this);
  }

  public void SetX(float value) {
    ffi_libJNI.Vec4_SetX(swigCPtr, this, value);
  }

  public void SetY(float value) {
    ffi_libJNI.Vec4_SetY(swigCPtr, this, value);
  }

  public void SetZ(float value) {
    ffi_libJNI.Vec4_SetZ(swigCPtr, this, value);
  }

  public void SetW(float value) {
    ffi_libJNI.Vec4_SetW(swigCPtr, this, value);
  }

  public SWIGTYPE_p_vec4 Context() {
    long cPtr = ffi_libJNI.Vec4_Context(swigCPtr, this);
    return (cPtr == 0) ? null : new SWIGTYPE_p_vec4(cPtr, false);
  }

  public static Vec4 FromContext(SWIGTYPE_p_vec4 context) {
    return new Vec4(ffi_libJNI.Vec4_FromContext(SWIGTYPE_p_vec4.getCPtr(context)), true);
  }

}