#include <napi.h>

Napi::String Hello(const Napi::CallbackInfo& info) {
  Napi::Env env = info.Env();
 
    // Code that might throw an exception
    // Example: int result = 1 / 0;  // This will throw a C++ exception
    return Napi::String::New(env, "Hello from JaxNode and C++!");
 
}

Napi::Object Init(Napi::Env env, Napi::Object exports) {
  exports.Set(Napi::String::New(env, "hello"), Napi::Function::New(env, Hello));
  return exports;
}

NODE_API_MODULE(addon, Init)
