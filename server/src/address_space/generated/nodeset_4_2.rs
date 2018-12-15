// This file was autogenerated from Opc.Ua.NodeSet2.Part4.xml by tools/schema/gen_address_space.js
// DO NOT EDIT THIS FILE

#[allow(unused_imports)]
use std::str::FromStr;

#[allow(unused_imports)]
use opcua_types::{
    node_id::NodeId,
    data_value::DataValue,
    variant::Variant, 
    extension_object::ExtensionObject, 
    string::UAString,
    basic_types::LocalizedText,
    service_types::{
        Argument
    },
    node_ids::*
};
#[allow(unused_imports)]
use crate::address_space::types::*;

#[allow(unused_variables)]
pub fn populate_address_space(address_space: &mut AddressSpace) {
    add_datatype_1(address_space);
    add_datatype_2(address_space);
    add_datatype_3(address_space);
    add_datatype_4(address_space);
    add_datatype_5(address_space);
    add_datatype_6(address_space);
    add_datatype_7(address_space);
    add_datatype_8(address_space);
    add_datatype_9(address_space);
    add_datatype_10(address_space);
    add_datatype_11(address_space);
    add_datatype_12(address_space);
    add_datatype_13(address_space);
    add_datatype_14(address_space);
    add_datatype_15(address_space);
    add_datatype_16(address_space);
    add_datatype_17(address_space);
    add_variable_18(address_space);
    add_variable_19(address_space);
    add_variable_20(address_space);
    add_variable_21(address_space);
    add_variable_22(address_space);
    add_variable_23(address_space);
    add_variable_24(address_space);
    add_variable_25(address_space);
    add_variable_26(address_space);
}

fn add_datatype_1(address_space: &mut AddressSpace) {
    // DataType
    let name = "Date";
    let description = "A date value.";
    let node_id = NodeId::new(0, 293);
    let node = DataType::new(&node_id, name, name, description, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 13), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_datatype_2(address_space: &mut AddressSpace) {
    // DataType
    let name = "EndpointConfiguration";
    let description = "";
    let node_id = NodeId::new(0, 331);
    let node = DataType::new(&node_id, name, name, description, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 22), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_datatype_3(address_space: &mut AddressSpace) {
    // DataType
    let name = "FilterOperator";
    let description = "";
    let node_id = NodeId::new(0, 576);
    let node = DataType::new(&node_id, name, name, description, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 7605), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 29), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_datatype_4(address_space: &mut AddressSpace) {
    // DataType
    let name = "ContentFilterElement";
    let description = "";
    let node_id = NodeId::new(0, 583);
    let node = DataType::new(&node_id, name, name, description, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 22), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_datatype_5(address_space: &mut AddressSpace) {
    // DataType
    let name = "ContentFilter";
    let description = "";
    let node_id = NodeId::new(0, 586);
    let node = DataType::new(&node_id, name, name, description, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 22), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_datatype_6(address_space: &mut AddressSpace) {
    // DataType
    let name = "FilterOperand";
    let description = "";
    let node_id = NodeId::new(0, 589);
    let node = DataType::new(&node_id, name, name, description, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 22), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_datatype_7(address_space: &mut AddressSpace) {
    // DataType
    let name = "ElementOperand";
    let description = "";
    let node_id = NodeId::new(0, 592);
    let node = DataType::new(&node_id, name, name, description, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 589), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_datatype_8(address_space: &mut AddressSpace) {
    // DataType
    let name = "LiteralOperand";
    let description = "";
    let node_id = NodeId::new(0, 595);
    let node = DataType::new(&node_id, name, name, description, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 589), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_datatype_9(address_space: &mut AddressSpace) {
    // DataType
    let name = "AttributeOperand";
    let description = "";
    let node_id = NodeId::new(0, 598);
    let node = DataType::new(&node_id, name, name, description, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 589), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_datatype_10(address_space: &mut AddressSpace) {
    // DataType
    let name = "SimpleAttributeOperand";
    let description = "";
    let node_id = NodeId::new(0, 601);
    let node = DataType::new(&node_id, name, name, description, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 589), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_datatype_11(address_space: &mut AddressSpace) {
    // DataType
    let name = "HistoryEvent";
    let description = "";
    let node_id = NodeId::new(0, 659);
    let node = DataType::new(&node_id, name, name, description, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 22), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_datatype_12(address_space: &mut AddressSpace) {
    // DataType
    let name = "HistoryUpdateType";
    let description = "";
    let node_id = NodeId::new(0, 11234);
    let node = DataType::new(&node_id, name, name, description, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 11884), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 29), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_datatype_13(address_space: &mut AddressSpace) {
    // DataType
    let name = "PerformUpdateType";
    let description = "";
    let node_id = NodeId::new(0, 11293);
    let node = DataType::new(&node_id, name, name, description, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 11885), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 29), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_datatype_14(address_space: &mut AddressSpace) {
    // DataType
    let name = "MonitoringFilter";
    let description = "";
    let node_id = NodeId::new(0, 719);
    let node = DataType::new(&node_id, name, name, description, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 22), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_datatype_15(address_space: &mut AddressSpace) {
    // DataType
    let name = "EventFilter";
    let description = "";
    let node_id = NodeId::new(0, 725);
    let node = DataType::new(&node_id, name, name, description, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 719), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_datatype_16(address_space: &mut AddressSpace) {
    // DataType
    let name = "AggregateConfiguration";
    let description = "";
    let node_id = NodeId::new(0, 948);
    let node = DataType::new(&node_id, name, name, description, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 22), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_datatype_17(address_space: &mut AddressSpace) {
    // DataType
    let name = "HistoryEventFieldList";
    let description = "";
    let node_id = NodeId::new(0, 920);
    let node = DataType::new(&node_id, name, name, description, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 22), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_18(address_space: &mut AddressSpace) {
    // Variable
    let name = "EnumStrings";
    let description = "";
    let data_value = DataValue::null();
    let node_id = NodeId::new(0, 7597);
    let node = Variable::new_data_value(&node_id, name, name, description, DataTypeId::LocalizedText, data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 307), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 307), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_19(address_space: &mut AddressSpace) {
    // Variable
    let name = "EnumStrings";
    let description = "";
    let data_value = DataValue::null();
    let node_id = NodeId::new(0, 7595);
    let node = Variable::new_data_value(&node_id, name, name, description, DataTypeId::LocalizedText, data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 302), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 302), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_20(address_space: &mut AddressSpace) {
    // Variable
    let name = "EnumStrings";
    let description = "";
    let data_value = DataValue::null();
    let node_id = NodeId::new(0, 7596);
    let node = Variable::new_data_value(&node_id, name, name, description, DataTypeId::LocalizedText, data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 303), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 303), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_21(address_space: &mut AddressSpace) {
    // Variable
    let name = "EnumStrings";
    let description = "";
    let data_value = DataValue::null();
    let node_id = NodeId::new(0, 7598);
    let node = Variable::new_data_value(&node_id, name, name, description, DataTypeId::LocalizedText, data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 315), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 315), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_22(address_space: &mut AddressSpace) {
    // Variable
    let name = "EnumValues";
    let description = "";
    let data_value = DataValue::null();
    let node_id = NodeId::new(0, 11881);
    let node = Variable::new_data_value(&node_id, name, name, description, DataTypeId::from_u32(7594u32).unwrap(), data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 348), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 348), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_23(address_space: &mut AddressSpace) {
    // Variable
    let name = "EnumValues";
    let description = "";
    let data_value = DataValue::null();
    let node_id = NodeId::new(0, 11882);
    let node = Variable::new_data_value(&node_id, name, name, description, DataTypeId::from_u32(7594u32).unwrap(), data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 347), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 347), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_24(address_space: &mut AddressSpace) {
    // Variable
    let name = "EnumStrings";
    let description = "";
    let data_value = DataValue::null();
    let node_id = NodeId::new(0, 7605);
    let node = Variable::new_data_value(&node_id, name, name, description, DataTypeId::LocalizedText, data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 576), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 576), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_25(address_space: &mut AddressSpace) {
    // Variable
    let name = "EnumValues";
    let description = "";
    let data_value = DataValue::null();
    let node_id = NodeId::new(0, 11884);
    let node = Variable::new_data_value(&node_id, name, name, description, DataTypeId::from_u32(7594u32).unwrap(), data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 11234), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 11234), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_26(address_space: &mut AddressSpace) {
    // Variable
    let name = "EnumValues";
    let description = "";
    let data_value = DataValue::null();
    let node_id = NodeId::new(0, 11885);
    let node = Variable::new_data_value(&node_id, name, name, description, DataTypeId::from_u32(7594u32).unwrap(), data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 11293), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 11293), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

