// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.26.0
// 	protoc        v3.17.3
// source: order_management.proto

package ecommerce

import (
	context "context"
	wrappers "github.com/golang/protobuf/ptypes/wrappers"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
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

type Order struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Id          string   `protobuf:"bytes,1,opt,name=id,proto3" json:"id,omitempty"`
	Items       []string `protobuf:"bytes,2,rep,name=items,proto3" json:"items,omitempty"`
	Description string   `protobuf:"bytes,3,opt,name=description,proto3" json:"description,omitempty"`
	Price       float32  `protobuf:"fixed32,4,opt,name=price,proto3" json:"price,omitempty"`
	Destination string   `protobuf:"bytes,5,opt,name=destination,proto3" json:"destination,omitempty"`
}

func (x *Order) Reset() {
	*x = Order{}
	if protoimpl.UnsafeEnabled {
		mi := &file_order_management_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Order) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Order) ProtoMessage() {}

func (x *Order) ProtoReflect() protoreflect.Message {
	mi := &file_order_management_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Order.ProtoReflect.Descriptor instead.
func (*Order) Descriptor() ([]byte, []int) {
	return file_order_management_proto_rawDescGZIP(), []int{0}
}

func (x *Order) GetId() string {
	if x != nil {
		return x.Id
	}
	return ""
}

func (x *Order) GetItems() []string {
	if x != nil {
		return x.Items
	}
	return nil
}

func (x *Order) GetDescription() string {
	if x != nil {
		return x.Description
	}
	return ""
}

func (x *Order) GetPrice() float32 {
	if x != nil {
		return x.Price
	}
	return 0
}

func (x *Order) GetDestination() string {
	if x != nil {
		return x.Destination
	}
	return ""
}

type CombinedShipment struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Id        string   `protobuf:"bytes,1,opt,name=id,proto3" json:"id,omitempty"`
	Status    string   `protobuf:"bytes,2,opt,name=status,proto3" json:"status,omitempty"`
	OrderList []*Order `protobuf:"bytes,3,rep,name=orderList,proto3" json:"orderList,omitempty"`
}

func (x *CombinedShipment) Reset() {
	*x = CombinedShipment{}
	if protoimpl.UnsafeEnabled {
		mi := &file_order_management_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *CombinedShipment) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*CombinedShipment) ProtoMessage() {}

func (x *CombinedShipment) ProtoReflect() protoreflect.Message {
	mi := &file_order_management_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use CombinedShipment.ProtoReflect.Descriptor instead.
func (*CombinedShipment) Descriptor() ([]byte, []int) {
	return file_order_management_proto_rawDescGZIP(), []int{1}
}

func (x *CombinedShipment) GetId() string {
	if x != nil {
		return x.Id
	}
	return ""
}

func (x *CombinedShipment) GetStatus() string {
	if x != nil {
		return x.Status
	}
	return ""
}

func (x *CombinedShipment) GetOrderList() []*Order {
	if x != nil {
		return x.OrderList
	}
	return nil
}

var File_order_management_proto protoreflect.FileDescriptor

var file_order_management_proto_rawDesc = []byte{
	0x0a, 0x16, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x5f, 0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x6d, 0x65,
	0x6e, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x09, 0x65, 0x63, 0x6f, 0x6d, 0x6d, 0x65,
	0x72, 0x63, 0x65, 0x1a, 0x1e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x62, 0x75, 0x66, 0x2f, 0x77, 0x72, 0x61, 0x70, 0x70, 0x65, 0x72, 0x73, 0x2e, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x22, 0x87, 0x01, 0x0a, 0x05, 0x4f, 0x72, 0x64, 0x65, 0x72, 0x12, 0x0e, 0x0a,
	0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02, 0x69, 0x64, 0x12, 0x14, 0x0a,
	0x05, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x09, 0x52, 0x05, 0x69, 0x74,
	0x65, 0x6d, 0x73, 0x12, 0x20, 0x0a, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69,
	0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69,
	0x70, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x14, 0x0a, 0x05, 0x70, 0x72, 0x69, 0x63, 0x65, 0x18, 0x04,
	0x20, 0x01, 0x28, 0x02, 0x52, 0x05, 0x70, 0x72, 0x69, 0x63, 0x65, 0x12, 0x20, 0x0a, 0x0b, 0x64,
	0x65, 0x73, 0x74, 0x69, 0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x0b, 0x64, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0x6a, 0x0a,
	0x10, 0x43, 0x6f, 0x6d, 0x62, 0x69, 0x6e, 0x65, 0x64, 0x53, 0x68, 0x69, 0x70, 0x6d, 0x65, 0x6e,
	0x74, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02, 0x69,
	0x64, 0x12, 0x16, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28,
	0x09, 0x52, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x2e, 0x0a, 0x09, 0x6f, 0x72, 0x64,
	0x65, 0x72, 0x4c, 0x69, 0x73, 0x74, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x65,
	0x63, 0x6f, 0x6d, 0x6d, 0x65, 0x72, 0x63, 0x65, 0x2e, 0x4f, 0x72, 0x64, 0x65, 0x72, 0x52, 0x09,
	0x6f, 0x72, 0x64, 0x65, 0x72, 0x4c, 0x69, 0x73, 0x74, 0x32, 0xdd, 0x02, 0x0a, 0x0f, 0x4f, 0x72,
	0x64, 0x65, 0x72, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x12, 0x3a, 0x0a,
	0x08, 0x61, 0x64, 0x64, 0x4f, 0x72, 0x64, 0x65, 0x72, 0x12, 0x10, 0x2e, 0x65, 0x63, 0x6f, 0x6d,
	0x6d, 0x65, 0x72, 0x63, 0x65, 0x2e, 0x4f, 0x72, 0x64, 0x65, 0x72, 0x1a, 0x1c, 0x2e, 0x67, 0x6f,
	0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x53, 0x74,
	0x72, 0x69, 0x6e, 0x67, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x3a, 0x0a, 0x08, 0x67, 0x65, 0x74,
	0x4f, 0x72, 0x64, 0x65, 0x72, 0x12, 0x1c, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70,
	0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x56, 0x61,
	0x6c, 0x75, 0x65, 0x1a, 0x10, 0x2e, 0x65, 0x63, 0x6f, 0x6d, 0x6d, 0x65, 0x72, 0x63, 0x65, 0x2e,
	0x4f, 0x72, 0x64, 0x65, 0x72, 0x12, 0x40, 0x0a, 0x0c, 0x73, 0x65, 0x61, 0x72, 0x63, 0x68, 0x4f,
	0x72, 0x64, 0x65, 0x72, 0x73, 0x12, 0x1c, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70,
	0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x56, 0x61,
	0x6c, 0x75, 0x65, 0x1a, 0x10, 0x2e, 0x65, 0x63, 0x6f, 0x6d, 0x6d, 0x65, 0x72, 0x63, 0x65, 0x2e,
	0x4f, 0x72, 0x64, 0x65, 0x72, 0x30, 0x01, 0x12, 0x40, 0x0a, 0x0c, 0x75, 0x70, 0x64, 0x61, 0x74,
	0x65, 0x4f, 0x72, 0x64, 0x65, 0x72, 0x73, 0x12, 0x10, 0x2e, 0x65, 0x63, 0x6f, 0x6d, 0x6d, 0x65,
	0x72, 0x63, 0x65, 0x2e, 0x4f, 0x72, 0x64, 0x65, 0x72, 0x1a, 0x1c, 0x2e, 0x67, 0x6f, 0x6f, 0x67,
	0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x53, 0x74, 0x72, 0x69,
	0x6e, 0x67, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x28, 0x01, 0x12, 0x4e, 0x0a, 0x0d, 0x70, 0x72, 0x6f,
	0x63, 0x65, 0x73, 0x73, 0x4f, 0x72, 0x64, 0x65, 0x72, 0x73, 0x12, 0x1c, 0x2e, 0x67, 0x6f, 0x6f,
	0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x53, 0x74, 0x72,
	0x69, 0x6e, 0x67, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x1a, 0x1b, 0x2e, 0x65, 0x63, 0x6f, 0x6d, 0x6d,
	0x65, 0x72, 0x63, 0x65, 0x2e, 0x43, 0x6f, 0x6d, 0x62, 0x69, 0x6e, 0x65, 0x64, 0x53, 0x68, 0x69,
	0x70, 0x6d, 0x65, 0x6e, 0x74, 0x28, 0x01, 0x30, 0x01, 0x42, 0x0e, 0x5a, 0x0c, 0x2e, 0x2f, 0x3b,
	0x65, 0x63, 0x6f, 0x6d, 0x6d, 0x65, 0x72, 0x63, 0x65, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x33,
}

var (
	file_order_management_proto_rawDescOnce sync.Once
	file_order_management_proto_rawDescData = file_order_management_proto_rawDesc
)

func file_order_management_proto_rawDescGZIP() []byte {
	file_order_management_proto_rawDescOnce.Do(func() {
		file_order_management_proto_rawDescData = protoimpl.X.CompressGZIP(file_order_management_proto_rawDescData)
	})
	return file_order_management_proto_rawDescData
}

var file_order_management_proto_msgTypes = make([]protoimpl.MessageInfo, 2)
var file_order_management_proto_goTypes = []interface{}{
	(*Order)(nil),                // 0: ecommerce.Order
	(*CombinedShipment)(nil),     // 1: ecommerce.CombinedShipment
	(*wrappers.StringValue)(nil), // 2: google.protobuf.StringValue
}
var file_order_management_proto_depIdxs = []int32{
	0, // 0: ecommerce.CombinedShipment.orderList:type_name -> ecommerce.Order
	0, // 1: ecommerce.OrderManagement.addOrder:input_type -> ecommerce.Order
	2, // 2: ecommerce.OrderManagement.getOrder:input_type -> google.protobuf.StringValue
	2, // 3: ecommerce.OrderManagement.searchOrders:input_type -> google.protobuf.StringValue
	0, // 4: ecommerce.OrderManagement.updateOrders:input_type -> ecommerce.Order
	2, // 5: ecommerce.OrderManagement.processOrders:input_type -> google.protobuf.StringValue
	2, // 6: ecommerce.OrderManagement.addOrder:output_type -> google.protobuf.StringValue
	0, // 7: ecommerce.OrderManagement.getOrder:output_type -> ecommerce.Order
	0, // 8: ecommerce.OrderManagement.searchOrders:output_type -> ecommerce.Order
	2, // 9: ecommerce.OrderManagement.updateOrders:output_type -> google.protobuf.StringValue
	1, // 10: ecommerce.OrderManagement.processOrders:output_type -> ecommerce.CombinedShipment
	6, // [6:11] is the sub-list for method output_type
	1, // [1:6] is the sub-list for method input_type
	1, // [1:1] is the sub-list for extension type_name
	1, // [1:1] is the sub-list for extension extendee
	0, // [0:1] is the sub-list for field type_name
}

func init() { file_order_management_proto_init() }
func file_order_management_proto_init() {
	if File_order_management_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_order_management_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Order); i {
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
		file_order_management_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*CombinedShipment); i {
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
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_order_management_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   2,
			NumExtensions: 0,
			NumServices:   1,
		},
		GoTypes:           file_order_management_proto_goTypes,
		DependencyIndexes: file_order_management_proto_depIdxs,
		MessageInfos:      file_order_management_proto_msgTypes,
	}.Build()
	File_order_management_proto = out.File
	file_order_management_proto_rawDesc = nil
	file_order_management_proto_goTypes = nil
	file_order_management_proto_depIdxs = nil
}

// Reference imports to suppress errors if they are not otherwise used.
var _ context.Context
var _ grpc.ClientConnInterface

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
const _ = grpc.SupportPackageIsVersion6

// OrderManagementClient is the client API for OrderManagement service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://godoc.org/google.golang.org/grpc#ClientConn.NewStream.
type OrderManagementClient interface {
	AddOrder(ctx context.Context, in *Order, opts ...grpc.CallOption) (*wrappers.StringValue, error)
	GetOrder(ctx context.Context, in *wrappers.StringValue, opts ...grpc.CallOption) (*Order, error)
	SearchOrders(ctx context.Context, in *wrappers.StringValue, opts ...grpc.CallOption) (OrderManagement_SearchOrdersClient, error)
	UpdateOrders(ctx context.Context, opts ...grpc.CallOption) (OrderManagement_UpdateOrdersClient, error)
	ProcessOrders(ctx context.Context, opts ...grpc.CallOption) (OrderManagement_ProcessOrdersClient, error)
}

type orderManagementClient struct {
	cc grpc.ClientConnInterface
}

func NewOrderManagementClient(cc grpc.ClientConnInterface) OrderManagementClient {
	return &orderManagementClient{cc}
}

func (c *orderManagementClient) AddOrder(ctx context.Context, in *Order, opts ...grpc.CallOption) (*wrappers.StringValue, error) {
	out := new(wrappers.StringValue)
	err := c.cc.Invoke(ctx, "/ecommerce.OrderManagement/addOrder", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *orderManagementClient) GetOrder(ctx context.Context, in *wrappers.StringValue, opts ...grpc.CallOption) (*Order, error) {
	out := new(Order)
	err := c.cc.Invoke(ctx, "/ecommerce.OrderManagement/getOrder", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *orderManagementClient) SearchOrders(ctx context.Context, in *wrappers.StringValue, opts ...grpc.CallOption) (OrderManagement_SearchOrdersClient, error) {
	stream, err := c.cc.NewStream(ctx, &_OrderManagement_serviceDesc.Streams[0], "/ecommerce.OrderManagement/searchOrders", opts...)
	if err != nil {
		return nil, err
	}
	x := &orderManagementSearchOrdersClient{stream}
	if err := x.ClientStream.SendMsg(in); err != nil {
		return nil, err
	}
	if err := x.ClientStream.CloseSend(); err != nil {
		return nil, err
	}
	return x, nil
}

type OrderManagement_SearchOrdersClient interface {
	Recv() (*Order, error)
	grpc.ClientStream
}

type orderManagementSearchOrdersClient struct {
	grpc.ClientStream
}

func (x *orderManagementSearchOrdersClient) Recv() (*Order, error) {
	m := new(Order)
	if err := x.ClientStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

func (c *orderManagementClient) UpdateOrders(ctx context.Context, opts ...grpc.CallOption) (OrderManagement_UpdateOrdersClient, error) {
	stream, err := c.cc.NewStream(ctx, &_OrderManagement_serviceDesc.Streams[1], "/ecommerce.OrderManagement/updateOrders", opts...)
	if err != nil {
		return nil, err
	}
	x := &orderManagementUpdateOrdersClient{stream}
	return x, nil
}

type OrderManagement_UpdateOrdersClient interface {
	Send(*Order) error
	CloseAndRecv() (*wrappers.StringValue, error)
	grpc.ClientStream
}

type orderManagementUpdateOrdersClient struct {
	grpc.ClientStream
}

func (x *orderManagementUpdateOrdersClient) Send(m *Order) error {
	return x.ClientStream.SendMsg(m)
}

func (x *orderManagementUpdateOrdersClient) CloseAndRecv() (*wrappers.StringValue, error) {
	if err := x.ClientStream.CloseSend(); err != nil {
		return nil, err
	}
	m := new(wrappers.StringValue)
	if err := x.ClientStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

func (c *orderManagementClient) ProcessOrders(ctx context.Context, opts ...grpc.CallOption) (OrderManagement_ProcessOrdersClient, error) {
	stream, err := c.cc.NewStream(ctx, &_OrderManagement_serviceDesc.Streams[2], "/ecommerce.OrderManagement/processOrders", opts...)
	if err != nil {
		return nil, err
	}
	x := &orderManagementProcessOrdersClient{stream}
	return x, nil
}

type OrderManagement_ProcessOrdersClient interface {
	Send(*wrappers.StringValue) error
	Recv() (*CombinedShipment, error)
	grpc.ClientStream
}

type orderManagementProcessOrdersClient struct {
	grpc.ClientStream
}

func (x *orderManagementProcessOrdersClient) Send(m *wrappers.StringValue) error {
	return x.ClientStream.SendMsg(m)
}

func (x *orderManagementProcessOrdersClient) Recv() (*CombinedShipment, error) {
	m := new(CombinedShipment)
	if err := x.ClientStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

// OrderManagementServer is the server API for OrderManagement service.
type OrderManagementServer interface {
	AddOrder(context.Context, *Order) (*wrappers.StringValue, error)
	GetOrder(context.Context, *wrappers.StringValue) (*Order, error)
	SearchOrders(*wrappers.StringValue, OrderManagement_SearchOrdersServer) error
	UpdateOrders(OrderManagement_UpdateOrdersServer) error
	ProcessOrders(OrderManagement_ProcessOrdersServer) error
}

// UnimplementedOrderManagementServer can be embedded to have forward compatible implementations.
type UnimplementedOrderManagementServer struct {
}

func (*UnimplementedOrderManagementServer) AddOrder(context.Context, *Order) (*wrappers.StringValue, error) {
	return nil, status.Errorf(codes.Unimplemented, "method AddOrder not implemented")
}
func (*UnimplementedOrderManagementServer) GetOrder(context.Context, *wrappers.StringValue) (*Order, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetOrder not implemented")
}
func (*UnimplementedOrderManagementServer) SearchOrders(*wrappers.StringValue, OrderManagement_SearchOrdersServer) error {
	return status.Errorf(codes.Unimplemented, "method SearchOrders not implemented")
}
func (*UnimplementedOrderManagementServer) UpdateOrders(OrderManagement_UpdateOrdersServer) error {
	return status.Errorf(codes.Unimplemented, "method UpdateOrders not implemented")
}
func (*UnimplementedOrderManagementServer) ProcessOrders(OrderManagement_ProcessOrdersServer) error {
	return status.Errorf(codes.Unimplemented, "method ProcessOrders not implemented")
}

func RegisterOrderManagementServer(s *grpc.Server, srv OrderManagementServer) {
	s.RegisterService(&_OrderManagement_serviceDesc, srv)
}

func _OrderManagement_AddOrder_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(Order)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(OrderManagementServer).AddOrder(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/ecommerce.OrderManagement/AddOrder",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(OrderManagementServer).AddOrder(ctx, req.(*Order))
	}
	return interceptor(ctx, in, info, handler)
}

func _OrderManagement_GetOrder_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(wrappers.StringValue)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(OrderManagementServer).GetOrder(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/ecommerce.OrderManagement/GetOrder",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(OrderManagementServer).GetOrder(ctx, req.(*wrappers.StringValue))
	}
	return interceptor(ctx, in, info, handler)
}

func _OrderManagement_SearchOrders_Handler(srv interface{}, stream grpc.ServerStream) error {
	m := new(wrappers.StringValue)
	if err := stream.RecvMsg(m); err != nil {
		return err
	}
	return srv.(OrderManagementServer).SearchOrders(m, &orderManagementSearchOrdersServer{stream})
}

type OrderManagement_SearchOrdersServer interface {
	Send(*Order) error
	grpc.ServerStream
}

type orderManagementSearchOrdersServer struct {
	grpc.ServerStream
}

func (x *orderManagementSearchOrdersServer) Send(m *Order) error {
	return x.ServerStream.SendMsg(m)
}

func _OrderManagement_UpdateOrders_Handler(srv interface{}, stream grpc.ServerStream) error {
	return srv.(OrderManagementServer).UpdateOrders(&orderManagementUpdateOrdersServer{stream})
}

type OrderManagement_UpdateOrdersServer interface {
	SendAndClose(*wrappers.StringValue) error
	Recv() (*Order, error)
	grpc.ServerStream
}

type orderManagementUpdateOrdersServer struct {
	grpc.ServerStream
}

func (x *orderManagementUpdateOrdersServer) SendAndClose(m *wrappers.StringValue) error {
	return x.ServerStream.SendMsg(m)
}

func (x *orderManagementUpdateOrdersServer) Recv() (*Order, error) {
	m := new(Order)
	if err := x.ServerStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

func _OrderManagement_ProcessOrders_Handler(srv interface{}, stream grpc.ServerStream) error {
	return srv.(OrderManagementServer).ProcessOrders(&orderManagementProcessOrdersServer{stream})
}

type OrderManagement_ProcessOrdersServer interface {
	Send(*CombinedShipment) error
	Recv() (*wrappers.StringValue, error)
	grpc.ServerStream
}

type orderManagementProcessOrdersServer struct {
	grpc.ServerStream
}

func (x *orderManagementProcessOrdersServer) Send(m *CombinedShipment) error {
	return x.ServerStream.SendMsg(m)
}

func (x *orderManagementProcessOrdersServer) Recv() (*wrappers.StringValue, error) {
	m := new(wrappers.StringValue)
	if err := x.ServerStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

var _OrderManagement_serviceDesc = grpc.ServiceDesc{
	ServiceName: "ecommerce.OrderManagement",
	HandlerType: (*OrderManagementServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "addOrder",
			Handler:    _OrderManagement_AddOrder_Handler,
		},
		{
			MethodName: "getOrder",
			Handler:    _OrderManagement_GetOrder_Handler,
		},
	},
	Streams: []grpc.StreamDesc{
		{
			StreamName:    "searchOrders",
			Handler:       _OrderManagement_SearchOrders_Handler,
			ServerStreams: true,
		},
		{
			StreamName:    "updateOrders",
			Handler:       _OrderManagement_UpdateOrders_Handler,
			ClientStreams: true,
		},
		{
			StreamName:    "processOrders",
			Handler:       _OrderManagement_ProcessOrders_Handler,
			ServerStreams: true,
			ClientStreams: true,
		},
	},
	Metadata: "order_management.proto",
}
