/* ----------------------------------------------------------------------------
 * This file was automatically generated by SWIG (https://www.swig.org).
 * Version 4.1.1
 *
 * Do not make changes to this file unless you know what you are doing - modify
 * the SWIG interface file instead.
 * ----------------------------------------------------------------------------- */

package com.example;

public class SimpleService {
  private transient long swigCPtr;
  protected transient boolean swigCMemOwn;

  protected SimpleService(long cPtr, boolean cMemoryOwn) {
    swigCMemOwn = cMemoryOwn;
    swigCPtr = cPtr;
  }

  protected static long getCPtr(SimpleService obj) {
    return (obj == null) ? 0 : obj.swigCPtr;
  }

  protected static long swigRelease(SimpleService obj) {
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
        ffi_libJNI.delete_SimpleService(swigCPtr);
      }
      swigCPtr = 0;
    }
  }

  public SimpleService() {
    this(ffi_libJNI.new_SimpleService(), true);
  }

  public static SimpleService NewWith(long some_value) {
    return new SimpleService(ffi_libJNI.SimpleService_NewWith(some_value), true);
  }

  public void Dispose() {
    ffi_libJNI.SimpleService_Dispose(swigCPtr, this);
  }

  public SWIGTYPE_p_simpleservice Context() {
    long cPtr = ffi_libJNI.SimpleService_Context(swigCPtr, this);
    return (cPtr == 0) ? null : new SWIGTYPE_p_simpleservice(cPtr, false);
  }

  public static SimpleService FromContext(SWIGTYPE_p_simpleservice context) {
    return new SimpleService(ffi_libJNI.SimpleService_FromContext(SWIGTYPE_p_simpleservice.getCPtr(context)), true);
  }

}