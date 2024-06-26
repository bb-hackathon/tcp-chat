// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.25.0-devel
// 	protoc        v3.14.0
// source: requests.proto

package proto

import (
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

type UserLookupRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Types that are assignable to Identifier:
	//	*UserLookupRequest_Uuid
	//	*UserLookupRequest_Username
	Identifier isUserLookupRequest_Identifier `protobuf_oneof:"identifier"`
}

func (x *UserLookupRequest) Reset() {
	*x = UserLookupRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_requests_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *UserLookupRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*UserLookupRequest) ProtoMessage() {}

func (x *UserLookupRequest) ProtoReflect() protoreflect.Message {
	mi := &file_requests_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use UserLookupRequest.ProtoReflect.Descriptor instead.
func (*UserLookupRequest) Descriptor() ([]byte, []int) {
	return file_requests_proto_rawDescGZIP(), []int{0}
}

func (m *UserLookupRequest) GetIdentifier() isUserLookupRequest_Identifier {
	if m != nil {
		return m.Identifier
	}
	return nil
}

func (x *UserLookupRequest) GetUuid() *UUID {
	if x, ok := x.GetIdentifier().(*UserLookupRequest_Uuid); ok {
		return x.Uuid
	}
	return nil
}

func (x *UserLookupRequest) GetUsername() string {
	if x, ok := x.GetIdentifier().(*UserLookupRequest_Username); ok {
		return x.Username
	}
	return ""
}

type isUserLookupRequest_Identifier interface {
	isUserLookupRequest_Identifier()
}

type UserLookupRequest_Uuid struct {
	Uuid *UUID `protobuf:"bytes,2,opt,name=uuid,proto3,oneof"`
}

type UserLookupRequest_Username struct {
	Username string `protobuf:"bytes,1,opt,name=username,proto3,oneof"`
}

func (*UserLookupRequest_Uuid) isUserLookupRequest_Identifier() {}

func (*UserLookupRequest_Username) isUserLookupRequest_Identifier() {}

type RoomCreationRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Name      string  `protobuf:"bytes,1,opt,name=name,proto3" json:"name,omitempty"`
	UserUuids []*UUID `protobuf:"bytes,2,rep,name=user_uuids,json=userUuids,proto3" json:"user_uuids,omitempty"`
}

func (x *RoomCreationRequest) Reset() {
	*x = RoomCreationRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_requests_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *RoomCreationRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*RoomCreationRequest) ProtoMessage() {}

func (x *RoomCreationRequest) ProtoReflect() protoreflect.Message {
	mi := &file_requests_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use RoomCreationRequest.ProtoReflect.Descriptor instead.
func (*RoomCreationRequest) Descriptor() ([]byte, []int) {
	return file_requests_proto_rawDescGZIP(), []int{1}
}

func (x *RoomCreationRequest) GetName() string {
	if x != nil {
		return x.Name
	}
	return ""
}

func (x *RoomCreationRequest) GetUserUuids() []*UUID {
	if x != nil {
		return x.UserUuids
	}
	return nil
}

type RoomWithUserCreationRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	UserUuid *UUID `protobuf:"bytes,1,opt,name=user_uuid,json=userUuid,proto3" json:"user_uuid,omitempty"`
}

func (x *RoomWithUserCreationRequest) Reset() {
	*x = RoomWithUserCreationRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_requests_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *RoomWithUserCreationRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*RoomWithUserCreationRequest) ProtoMessage() {}

func (x *RoomWithUserCreationRequest) ProtoReflect() protoreflect.Message {
	mi := &file_requests_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use RoomWithUserCreationRequest.ProtoReflect.Descriptor instead.
func (*RoomWithUserCreationRequest) Descriptor() ([]byte, []int) {
	return file_requests_proto_rawDescGZIP(), []int{2}
}

func (x *RoomWithUserCreationRequest) GetUserUuid() *UUID {
	if x != nil {
		return x.UserUuid
	}
	return nil
}

type RoomList struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Rooms []*ServersideRoom `protobuf:"bytes,1,rep,name=rooms,proto3" json:"rooms,omitempty"`
}

func (x *RoomList) Reset() {
	*x = RoomList{}
	if protoimpl.UnsafeEnabled {
		mi := &file_requests_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *RoomList) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*RoomList) ProtoMessage() {}

func (x *RoomList) ProtoReflect() protoreflect.Message {
	mi := &file_requests_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use RoomList.ProtoReflect.Descriptor instead.
func (*RoomList) Descriptor() ([]byte, []int) {
	return file_requests_proto_rawDescGZIP(), []int{3}
}

func (x *RoomList) GetRooms() []*ServersideRoom {
	if x != nil {
		return x.Rooms
	}
	return nil
}

type MessageList struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Messages []*ServersideMessage `protobuf:"bytes,1,rep,name=messages,proto3" json:"messages,omitempty"`
}

func (x *MessageList) Reset() {
	*x = MessageList{}
	if protoimpl.UnsafeEnabled {
		mi := &file_requests_proto_msgTypes[4]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *MessageList) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*MessageList) ProtoMessage() {}

func (x *MessageList) ProtoReflect() protoreflect.Message {
	mi := &file_requests_proto_msgTypes[4]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use MessageList.ProtoReflect.Descriptor instead.
func (*MessageList) Descriptor() ([]byte, []int) {
	return file_requests_proto_rawDescGZIP(), []int{4}
}

func (x *MessageList) GetMessages() []*ServersideMessage {
	if x != nil {
		return x.Messages
	}
	return nil
}

var File_requests_proto protoreflect.FileDescriptor

var file_requests_proto_rawDesc = []byte{
	0x0a, 0x0e, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x12, 0x08, 0x74, 0x63, 0x70, 0x5f, 0x63, 0x68, 0x61, 0x74, 0x1a, 0x0e, 0x65, 0x6e, 0x74, 0x69,
	0x74, 0x69, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x65, 0x0a, 0x11, 0x55, 0x73,
	0x65, 0x72, 0x4c, 0x6f, 0x6f, 0x6b, 0x75, 0x70, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12,
	0x24, 0x0a, 0x04, 0x75, 0x75, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e,
	0x74, 0x63, 0x70, 0x5f, 0x63, 0x68, 0x61, 0x74, 0x2e, 0x55, 0x55, 0x49, 0x44, 0x48, 0x00, 0x52,
	0x04, 0x75, 0x75, 0x69, 0x64, 0x12, 0x1c, 0x0a, 0x08, 0x75, 0x73, 0x65, 0x72, 0x6e, 0x61, 0x6d,
	0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x08, 0x75, 0x73, 0x65, 0x72, 0x6e,
	0x61, 0x6d, 0x65, 0x42, 0x0c, 0x0a, 0x0a, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65,
	0x72, 0x22, 0x58, 0x0a, 0x13, 0x52, 0x6f, 0x6f, 0x6d, 0x43, 0x72, 0x65, 0x61, 0x74, 0x69, 0x6f,
	0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65,
	0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x2d, 0x0a, 0x0a,
	0x75, 0x73, 0x65, 0x72, 0x5f, 0x75, 0x75, 0x69, 0x64, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b,
	0x32, 0x0e, 0x2e, 0x74, 0x63, 0x70, 0x5f, 0x63, 0x68, 0x61, 0x74, 0x2e, 0x55, 0x55, 0x49, 0x44,
	0x52, 0x09, 0x75, 0x73, 0x65, 0x72, 0x55, 0x75, 0x69, 0x64, 0x73, 0x22, 0x4a, 0x0a, 0x1b, 0x52,
	0x6f, 0x6f, 0x6d, 0x57, 0x69, 0x74, 0x68, 0x55, 0x73, 0x65, 0x72, 0x43, 0x72, 0x65, 0x61, 0x74,
	0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x2b, 0x0a, 0x09, 0x75, 0x73,
	0x65, 0x72, 0x5f, 0x75, 0x75, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e,
	0x74, 0x63, 0x70, 0x5f, 0x63, 0x68, 0x61, 0x74, 0x2e, 0x55, 0x55, 0x49, 0x44, 0x52, 0x08, 0x75,
	0x73, 0x65, 0x72, 0x55, 0x75, 0x69, 0x64, 0x22, 0x3a, 0x0a, 0x08, 0x52, 0x6f, 0x6f, 0x6d, 0x4c,
	0x69, 0x73, 0x74, 0x12, 0x2e, 0x0a, 0x05, 0x72, 0x6f, 0x6f, 0x6d, 0x73, 0x18, 0x01, 0x20, 0x03,
	0x28, 0x0b, 0x32, 0x18, 0x2e, 0x74, 0x63, 0x70, 0x5f, 0x63, 0x68, 0x61, 0x74, 0x2e, 0x53, 0x65,
	0x72, 0x76, 0x65, 0x72, 0x73, 0x69, 0x64, 0x65, 0x52, 0x6f, 0x6f, 0x6d, 0x52, 0x05, 0x72, 0x6f,
	0x6f, 0x6d, 0x73, 0x22, 0x46, 0x0a, 0x0b, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x4c, 0x69,
	0x73, 0x74, 0x12, 0x37, 0x0a, 0x08, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x18, 0x01,
	0x20, 0x03, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x74, 0x63, 0x70, 0x5f, 0x63, 0x68, 0x61, 0x74, 0x2e,
	0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x73, 0x69, 0x64, 0x65, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67,
	0x65, 0x52, 0x08, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x42, 0x33, 0x5a, 0x31, 0x67,
	0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x67, 0x6f, 0x6c, 0x61, 0x6e, 0x67, 0x2e, 0x6f, 0x72, 0x67,
	0x2f, 0x62, 0x62, 0x2d, 0x68, 0x61, 0x63, 0x6b, 0x61, 0x74, 0x68, 0x6f, 0x6e, 0x2f, 0x74, 0x63,
	0x70, 0x2d, 0x63, 0x68, 0x61, 0x74, 0x2e, 0x67, 0x69, 0x74, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_requests_proto_rawDescOnce sync.Once
	file_requests_proto_rawDescData = file_requests_proto_rawDesc
)

func file_requests_proto_rawDescGZIP() []byte {
	file_requests_proto_rawDescOnce.Do(func() {
		file_requests_proto_rawDescData = protoimpl.X.CompressGZIP(file_requests_proto_rawDescData)
	})
	return file_requests_proto_rawDescData
}

var file_requests_proto_msgTypes = make([]protoimpl.MessageInfo, 5)
var file_requests_proto_goTypes = []interface{}{
	(*UserLookupRequest)(nil),           // 0: tcp_chat.UserLookupRequest
	(*RoomCreationRequest)(nil),         // 1: tcp_chat.RoomCreationRequest
	(*RoomWithUserCreationRequest)(nil), // 2: tcp_chat.RoomWithUserCreationRequest
	(*RoomList)(nil),                    // 3: tcp_chat.RoomList
	(*MessageList)(nil),                 // 4: tcp_chat.MessageList
	(*UUID)(nil),                        // 5: tcp_chat.UUID
	(*ServersideRoom)(nil),              // 6: tcp_chat.ServersideRoom
	(*ServersideMessage)(nil),           // 7: tcp_chat.ServersideMessage
}
var file_requests_proto_depIdxs = []int32{
	5, // 0: tcp_chat.UserLookupRequest.uuid:type_name -> tcp_chat.UUID
	5, // 1: tcp_chat.RoomCreationRequest.user_uuids:type_name -> tcp_chat.UUID
	5, // 2: tcp_chat.RoomWithUserCreationRequest.user_uuid:type_name -> tcp_chat.UUID
	6, // 3: tcp_chat.RoomList.rooms:type_name -> tcp_chat.ServersideRoom
	7, // 4: tcp_chat.MessageList.messages:type_name -> tcp_chat.ServersideMessage
	5, // [5:5] is the sub-list for method output_type
	5, // [5:5] is the sub-list for method input_type
	5, // [5:5] is the sub-list for extension type_name
	5, // [5:5] is the sub-list for extension extendee
	0, // [0:5] is the sub-list for field type_name
}

func init() { file_requests_proto_init() }
func file_requests_proto_init() {
	if File_requests_proto != nil {
		return
	}
	file_entities_proto_init()
	if !protoimpl.UnsafeEnabled {
		file_requests_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*UserLookupRequest); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_requests_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*RoomCreationRequest); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_requests_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*RoomWithUserCreationRequest); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_requests_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*RoomList); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_requests_proto_msgTypes[4].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*MessageList); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
	}
	file_requests_proto_msgTypes[0].OneofWrappers = []interface{}{
		(*UserLookupRequest_Uuid)(nil),
		(*UserLookupRequest_Username)(nil),
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_requests_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   5,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_requests_proto_goTypes,
		DependencyIndexes: file_requests_proto_depIdxs,
		MessageInfos:      file_requests_proto_msgTypes,
	}.Build()
	File_requests_proto = out.File
	file_requests_proto_rawDesc = nil
	file_requests_proto_goTypes = nil
	file_requests_proto_depIdxs = nil
}
