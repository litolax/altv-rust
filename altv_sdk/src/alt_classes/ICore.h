#pragma once
#define ALT_SERVER_API
#include "alt_bridge.h"

namespace ICore {

std::string GetVersion() {
    return alt::ICore::Instance().GetVersion();
}
std::string GetBranch() {
    return alt::ICore::Instance().GetBranch();
}
void LogInfo(const StdStringClone str, alt::IResource* resource) {
    return alt::ICore::Instance().LogInfo(str, resource);
}
void LogDebug(const StdStringClone str, alt::IResource* resource) {
    return alt::ICore::Instance().LogDebug(str, resource);
}
void LogWarning(const StdStringClone str, alt::IResource* resource) {
    return alt::ICore::Instance().LogWarning(str, resource);
}
void LogError(const StdStringClone str, alt::IResource* resource) {
    return alt::ICore::Instance().LogError(str, resource);
}
void LogColored(const StdStringClone str, alt::IResource* resource) {
    return alt::ICore::Instance().LogColored(str, resource);
}
bool IsDebug() {
    return alt::ICore::Instance().IsDebug();
}
u32 Hash(const StdStringClone str) {
    return alt::ICore::Instance().Hash(str);
}
bool FileExists(const StdStringClone path) {
    return alt::ICore::Instance().FileExists(path);
}
std::string FileRead(const StdStringClone path) {
    return alt::ICore::Instance().FileRead(path);
}
alt::IResource* GetResource(const StdStringClone name) {
    return alt::ICore::Instance().GetResource(name);
}
alt::IEntity* GetEntityByID(u16 id) {
    return alt::ICore::Instance().GetEntityByID(id);
}
std::vector<alt::IVirtualEntity*> GetVirtualEntities() {
    return alt::ICore::Instance().GetVirtualEntities();
}
bool HasMetaData(const StdStringClone key) {
    return alt::ICore::Instance().HasMetaData(key);
}
MValueWrapper GetMetaData(const StdStringClone key) {
    MValueWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValueConst>(alt::ICore::Instance().GetMetaData(key));
    return wrapper;
}
void SetMetaData(const StdStringClone key, MValueMutWrapper val) {
    return alt::ICore::Instance().SetMetaData(key, *(val.ptr));
}
void DeleteMetaData(const StdStringClone key) {
    return alt::ICore::Instance().DeleteMetaData(key);
}
bool HasSyncedMetaData(const StdStringClone key) {
    return alt::ICore::Instance().HasSyncedMetaData(key);
}
MValueWrapper GetSyncedMetaData(const StdStringClone key) {
    MValueWrapper wrapper;
    wrapper.ptr = std::make_shared<alt::MValueConst>(alt::ICore::Instance().GetSyncedMetaData(key));
    return wrapper;
}
void DestroyBaseObject(alt::IBaseObject* handle) {
    return alt::ICore::Instance().DestroyBaseObject(handle);
}
std::string StringToSHA256(const StdStringClone str) {
    return alt::ICore::Instance().StringToSHA256(str);
}
bool IsEventEnabled(u16 type) {
    return alt::ICore::Instance().IsEventEnabled(static_cast<alt::CEvent::Type>(type));
}
void ToggleEvent(u16 type, bool state) {
    return alt::ICore::Instance().ToggleEvent(static_cast<alt::CEvent::Type>(type), state);
}
StdStringClone GetRootDirectory() {
    return std::string { alt::ICore::Instance().GetRootDirectory() };
}
alt::IResource* StartResource(const StdStringClone name) {
    return alt::ICore::Instance().StartResource(name);
}
void StopResource(const StdStringClone name) {
    return alt::ICore::Instance().StopResource(name);
}
void RestartResource(const StdStringClone name) {
    return alt::ICore::Instance().RestartResource(name);
}
void SetSyncedMetaData(const StdStringClone key, MValueMutWrapper val) {
    return alt::ICore::Instance().SetSyncedMetaData(key, *(val.ptr));
}
void DeleteSyncedMetaData(const StdStringClone key) {
    return alt::ICore::Instance().DeleteSyncedMetaData(key);
}
alt::IVehicle* CreateVehicle(u32 model, f32 pos_x, f32 pos_y, f32 pos_z, f32 rot_x, f32 rot_y, f32 rot_z) {
    return alt::ICore::Instance().CreateVehicle(model, { pos_x, pos_y, pos_z }, { rot_x, rot_y, rot_z });
}
alt::IVirtualEntity* CreateVirtualEntity(alt::IVirtualEntityGroup* group, f32 pos_x, f32 pos_y, f32 pos_z, u32 streamingDistance) {
    return alt::ICore::Instance().CreateVirtualEntity(group, { pos_x, pos_y, pos_z }, streamingDistance);
}
alt::IVirtualEntityGroup* CreateVirtualEntityGroup(u32 streamingRangeLimit) {
    return alt::ICore::Instance().CreateVirtualEntityGroup(streamingRangeLimit);
}
alt::IColShape* CreateColShapeCylinder(f32 pos_x, f32 pos_y, f32 pos_z, f32 radius, f32 height) {
    return alt::ICore::Instance().CreateColShapeCylinder({ pos_x, pos_y, pos_z }, radius, height);
}
alt::IColShape* CreateColShapeSphere(f32 pos_x, f32 pos_y, f32 pos_z, f32 radius) {
    return alt::ICore::Instance().CreateColShapeSphere({ pos_x, pos_y, pos_z }, radius);
}
alt::IColShape* CreateColShapeCircle(f32 pos_x, f32 pos_y, f32 pos_z, f32 radius) {
    return alt::ICore::Instance().CreateColShapeCircle({ pos_x, pos_y, pos_z }, radius);
}
alt::IColShape* CreateColShapeCube(f32 pos_x, f32 pos_y, f32 pos_z, f32 pos2_x, f32 pos2_y, f32 pos2_z) {
    return alt::ICore::Instance().CreateColShapeCube({ pos_x, pos_y, pos_z }, { pos2_x, pos2_y, pos2_z });
}
alt::IColShape* CreateColShapeRectangle(f32 x1, f32 y1, f32 x2, f32 y2, f32 z) {
    return alt::ICore::Instance().CreateColShapeRectangle(x1, y1, x2, y2, z);
}
u32 GetNetTime() {
    return alt::ICore::Instance().GetNetTime();
}
void SetPassword(const StdStringClone password) {
    return alt::ICore::Instance().SetPassword(password);
}
u64 HashServerPassword(const StdStringClone password) {
    return alt::ICore::Instance().HashServerPassword(password);
}
void StopServer() {
    return alt::ICore::Instance().StopServer();
}
void SetWorldProfiler(bool state) {
    return alt::ICore::Instance().SetWorldProfiler(state);
}

} // namespace