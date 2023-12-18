#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

jlong Java_com_shubham0204_text_1predictor_NextWordPredictor_createNativeInstance(JNIEnv env,
                                                                                  JClass,
                                                                                  JString filepath);

void Java_com_shubham0204_text_1predictor_NextWordPredictor_deleteNativeInstance(JNIEnv,
                                                                                 JClass,
                                                                                 jlong object_ptr);

JString Java_com_shubham0204_text_1predictor_NextWordPredictor_predictToken(JNIEnv env,
                                                                            JClass,
                                                                            jlong object_ptr,
                                                                            JString token);

jlong Java_com_shubham0204_text_1predictor_WordAutoCompletion_createNativeInstance(JNIEnv env,
                                                                                   JClass,
                                                                                   JString filepath);

void Java_com_shubham0204_text_1predictor_WordAutoCompletion_deleteNativeInstance(JNIEnv,
                                                                                  JClass,
                                                                                  jlong object_ptr);

JString Java_com_shubham0204_text_1predictor_WordAutoCompletion_predictWord(JNIEnv env,
                                                                            JClass,
                                                                            jlong object_ptr,
                                                                            JString phrase);
