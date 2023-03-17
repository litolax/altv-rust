#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace IBaseObject {

BaseObjectType GetType(const alt::IBaseObject* ptr) {
    return static_cast<uint8_t>(ptr->GetType());
}
bool HasMetaData(const alt::IBaseObject* ptr, const StdStringClone key) {
    return ptr->HasMetaData(key);
}
MValueWrapper GetMetaData(const alt::IBaseObject* ptr, const StdStringClone key) {
    MValueWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValueConst>(ptr->GetMetaData(key));
    return wrapper;
}
void SetMetaData(alt::IBaseObject* ptr, const StdStringClone key, MValueMutWrapper val) {
    return ptr->SetMetaData(key, *(val.ptr));
}
void DeleteMetaData(alt::IBaseObject* ptr, const StdStringClone key) {
    return ptr->DeleteMetaData(key);
}
std::vector<std::string> GetMetaDataKeys(const alt::IBaseObject* ptr) {
    return ptr->GetMetaDataKeys();
}
bool IsRemoved(const alt::IBaseObject* ptr) {
    return ptr->IsRemoved();
}

} // namespace