use super::*;

#[test]
fn add_schema_support_to_entity_success() {
    with_test_externalities(|| {
        // Add entity schemas support
        let (first_entity, second_entity) = add_entity_schemas_support();

        // Ensure supported schemas set and properties of first entity updated succesfully
        assert_eq!(first_entity, entity_by_id(FIRST_ENTITY_ID));

        // Ensure reference counter of second entity updated succesfully
        assert_eq!(second_entity, entity_by_id(SECOND_ENTITY_ID));
    })
}

#[test]
fn add_schema_support_to_non_existent_entity() {
    with_test_externalities(|| {
        let actor = emulate_entity_access_state_for_failure_case(
            EntityAccessStateFailureType::EntityNotFound,
        );

        // Create property
        let property = Property::<ClassId>::default_with_name(
            PropertyNameLengthConstraint::get().max() as usize,
        );

        // Add Schema to the Class
        assert_ok!(add_class_schema(
            LEAD_ORIGIN,
            FIRST_CLASS_ID,
            BTreeSet::new(),
            vec![property]
        ));

        let mut schema_property_values = BTreeMap::new();
        schema_property_values.insert(FIRST_PROPERTY_ID, InputPropertyValue::default());

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        // Make an attempt to add schema support to non existent Entity
        let add_schema_support_to_entity_result = add_schema_support_to_entity(
            LEAD_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_SCHEMA_ID,
            schema_property_values,
        );

        // Failure checked
        assert_failure(
            add_schema_support_to_entity_result,
            Error::<Runtime>::EntityNotFound,
            number_of_events_before_call,
        );
    })
}

#[test]
fn add_schema_support_lead_auth_failed() {
    with_test_externalities(|| {
        let actor = emulate_entity_access_state_for_failure_case(
            EntityAccessStateFailureType::LeadAuthFailed,
        );

        // Create property
        let property = Property::<ClassId>::default_with_name(
            PropertyNameLengthConstraint::get().max() as usize,
        );

        // Add Schema to the Class
        assert_ok!(add_class_schema(
            LEAD_ORIGIN,
            FIRST_CLASS_ID,
            BTreeSet::new(),
            vec![property]
        ));

        let mut schema_property_values = BTreeMap::new();
        schema_property_values.insert(FIRST_PROPERTY_ID, InputPropertyValue::default());

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        // Make an attempt to add schema support under non lead origin
        let add_schema_support_to_entity_result = add_schema_support_to_entity(
            UNKNOWN_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_SCHEMA_ID,
            schema_property_values,
        );

        // Failure checked
        assert_failure(
            add_schema_support_to_entity_result,
            Error::<Runtime>::LeadAuthFailed,
            number_of_events_before_call,
        );
    })
}

#[test]
fn add_schema_support_member_auth_failed() {
    with_test_externalities(|| {
        let actor = emulate_entity_access_state_for_failure_case(
            EntityAccessStateFailureType::MemberAuthFailed,
        );

        // Create property
        let property = Property::<ClassId>::default_with_name(
            PropertyNameLengthConstraint::get().max() as usize,
        );

        // Add Schema to the Class
        assert_ok!(add_class_schema(
            LEAD_ORIGIN,
            FIRST_CLASS_ID,
            BTreeSet::new(),
            vec![property]
        ));

        let mut schema_property_values = BTreeMap::new();
        schema_property_values.insert(FIRST_PROPERTY_ID, InputPropertyValue::default());

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        // Make an attempt to add schema support to entity using unknown origin and member actor
        let add_schema_support_to_entity_result = add_schema_support_to_entity(
            UNKNOWN_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_SCHEMA_ID,
            schema_property_values,
        );

        // Failure checked
        assert_failure(
            add_schema_support_to_entity_result,
            Error::<Runtime>::MemberAuthFailed,
            number_of_events_before_call,
        );
    })
}

#[test]
fn add_schema_support_curator_group_is_not_active() {
    with_test_externalities(|| {
        let actor = emulate_entity_access_state_for_failure_case(
            EntityAccessStateFailureType::CuratorAuthFailed,
        );

        // Make curator group inactive to block it from any entity operations
        assert_ok!(set_curator_group_status(
            LEAD_ORIGIN,
            FIRST_CURATOR_GROUP_ID,
            false
        ));

        // Create property
        let property = Property::<ClassId>::default_with_name(
            PropertyNameLengthConstraint::get().max() as usize,
        );

        // Add Schema to the Class
        assert_ok!(add_class_schema(
            LEAD_ORIGIN,
            FIRST_CLASS_ID,
            BTreeSet::new(),
            vec![property]
        ));

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        let mut schema_property_values = BTreeMap::new();
        schema_property_values.insert(FIRST_PROPERTY_ID, InputPropertyValue::default());

        // Make an attempt to add schema support to entity using curator group, which is not active as actor
        let add_schema_support_to_entity_result = add_schema_support_to_entity(
            FIRST_CURATOR_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_SCHEMA_ID,
            schema_property_values,
        );

        // Failure checked
        assert_failure(
            add_schema_support_to_entity_result,
            Error::<Runtime>::CuratorGroupIsNotActive,
            number_of_events_before_call,
        );
    })
}

#[test]
fn add_schema_support_curator_auth_failed() {
    with_test_externalities(|| {
        let actor = emulate_entity_access_state_for_failure_case(
            EntityAccessStateFailureType::CuratorAuthFailed,
        );

        // Create property
        let property = Property::<ClassId>::default_with_name(
            PropertyNameLengthConstraint::get().max() as usize,
        );

        // Add Schema to the Class
        assert_ok!(add_class_schema(
            LEAD_ORIGIN,
            FIRST_CLASS_ID,
            BTreeSet::new(),
            vec![property]
        ));

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        let mut schema_property_values = BTreeMap::new();
        schema_property_values.insert(FIRST_PROPERTY_ID, InputPropertyValue::default());

        // Make an attempt to add schema support to entity under unknown origin and curator actor,
        // which corresponding group is current entity controller
        let add_schema_support_to_entity_result = add_schema_support_to_entity(
            UNKNOWN_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_SCHEMA_ID,
            schema_property_values,
        );

        // Failure checked
        assert_failure(
            add_schema_support_to_entity_result,
            Error::<Runtime>::CuratorAuthFailed,
            number_of_events_before_call,
        );
    })
}

#[test]
fn add_schema_support_curator_not_found_in_curator_group() {
    with_test_externalities(|| {
        let actor = emulate_entity_access_state_for_failure_case(
            EntityAccessStateFailureType::CuratorNotFoundInCuratorGroup,
        );

        // Create property
        let property = Property::<ClassId>::default_with_name(
            PropertyNameLengthConstraint::get().max() as usize,
        );

        // Add Schema to the Class
        assert_ok!(add_class_schema(
            LEAD_ORIGIN,
            FIRST_CLASS_ID,
            BTreeSet::new(),
            vec![property]
        ));

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        let mut schema_property_values = BTreeMap::new();
        schema_property_values.insert(FIRST_PROPERTY_ID, InputPropertyValue::default());

        // Make an attempt to add schema support to entity, using actor in group,
        // which curator id was not added to corresponding group set
        let add_schema_support_to_entity_result = add_schema_support_to_entity(
            SECOND_CURATOR_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_SCHEMA_ID,
            schema_property_values,
        );

        // Failure checked
        assert_failure(
            add_schema_support_to_entity_result,
            Error::<Runtime>::CuratorIsNotAMemberOfGivenCuratorGroup,
            number_of_events_before_call,
        );
    })
}

#[test]
fn add_schema_support_access_denied() {
    with_test_externalities(|| {
        let actor = emulate_entity_access_state_for_failure_case(
            EntityAccessStateFailureType::EntityAccessDenied,
        );

        // Create property
        let property = Property::<ClassId>::default_with_name(
            PropertyNameLengthConstraint::get().max() as usize,
        );

        // Add Schema to the Class
        assert_ok!(add_class_schema(
            LEAD_ORIGIN,
            FIRST_CLASS_ID,
            BTreeSet::new(),
            vec![property]
        ));

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        let mut schema_property_values = BTreeMap::new();
        schema_property_values.insert(FIRST_PROPERTY_ID, InputPropertyValue::default());

        // Make an attempt to add schema support to entity, using origin,
        // which corresponding actor is neither entity maintainer, nor controller.
        let add_schema_support_to_entity_result = add_schema_support_to_entity(
            SECOND_MEMBER_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_SCHEMA_ID,
            schema_property_values,
        );

        // Failure checked
        assert_failure(
            add_schema_support_to_entity_result,
            Error::<Runtime>::EntityAccessDenied,
            number_of_events_before_call,
        );
    })
}

#[test]
fn add_schema_support_to_entity_schema_does_not_exist() {
    with_test_externalities(|| {
        // Create class with default permissions
        assert_ok!(create_simple_class(LEAD_ORIGIN, ClassType::Valid));

        let actor = Actor::Lead;

        // Create entity
        assert_ok!(create_entity(LEAD_ORIGIN, FIRST_CLASS_ID, actor.clone()));

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        let mut schema_property_values = BTreeMap::new();
        schema_property_values.insert(FIRST_PROPERTY_ID, InputPropertyValue::default());

        // Make an attempt to add schema support to entity, providing schema_id,
        // which corresponding Schema does not exist on Class level
        let add_schema_support_to_entity_result = add_schema_support_to_entity(
            LEAD_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_SCHEMA_ID,
            schema_property_values,
        );

        // Failure checked
        assert_failure(
            add_schema_support_to_entity_result,
            Error::<Runtime>::UnknownClassSchemaId,
            number_of_events_before_call,
        );
    })
}

#[test]
fn add_schema_support_to_entity_class_property_not_found() {
    with_test_externalities(|| {
        // Create class with default permissions
        assert_ok!(create_simple_class(LEAD_ORIGIN, ClassType::Valid));

        // Create property
        let property = Property::<ClassId>::default_with_name(
            PropertyNameLengthConstraint::get().max() as usize,
        );

        // Add Schema to the Class
        assert_ok!(add_class_schema(
            LEAD_ORIGIN,
            FIRST_CLASS_ID,
            BTreeSet::new(),
            vec![property]
        ));

        let actor = Actor::Lead;

        // Create entity
        assert_ok!(create_entity(LEAD_ORIGIN, FIRST_CLASS_ID, actor.clone()));

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        let mut schema_property_values = BTreeMap::new();
        schema_property_values.insert(SECOND_PROPERTY_ID, InputPropertyValue::default());

        // Make an attempt to add schema support to Entity, providing property value under property_id,
        // which does not not yet added to corresponding Class properties
        let add_schema_support_to_entity_result = add_schema_support_to_entity(
            LEAD_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_SCHEMA_ID,
            schema_property_values,
        );

        // Failure checked
        assert_failure(
            add_schema_support_to_entity_result,
            Error::<Runtime>::ClassPropertyNotFound,
            number_of_events_before_call,
        );
    })
}

#[test]
fn add_schema_support_already_added_to_the_entity() {
    with_test_externalities(|| {
        // Create class with default permissions
        assert_ok!(create_simple_class(LEAD_ORIGIN, ClassType::Valid));

        // Create property
        let property = Property::<ClassId>::default_with_name(
            PropertyNameLengthConstraint::get().max() as usize,
        );

        // Add Schema to the Class
        assert_ok!(add_class_schema(
            LEAD_ORIGIN,
            FIRST_CLASS_ID,
            BTreeSet::new(),
            vec![property]
        ));

        let actor = Actor::Lead;

        // Create entity
        assert_ok!(create_entity(LEAD_ORIGIN, FIRST_CLASS_ID, actor.clone()));

        let mut schema_property_values = BTreeMap::new();
        schema_property_values.insert(FIRST_PROPERTY_ID, InputPropertyValue::default());

        assert_ok!(add_schema_support_to_entity(
            LEAD_ORIGIN,
            actor.clone(),
            FIRST_ENTITY_ID,
            FIRST_SCHEMA_ID,
            schema_property_values.clone(),
        ));

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        // Make an attempt to add schema support to entity, providing schema_id,
        // which was already added to the Entity
        let add_schema_support_to_entity_result = add_schema_support_to_entity(
            LEAD_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_SCHEMA_ID,
            schema_property_values,
        );

        // Failure checked
        assert_failure(
            add_schema_support_to_entity_result,
            Error::<Runtime>::SchemaAlreadyAddedToTheEntity,
            number_of_events_before_call,
        );
    })
}

#[test]
fn add_schema_support_already_contains_given_property_id() {
    with_test_externalities(|| {
        // Create class with default permissions
        assert_ok!(create_simple_class(LEAD_ORIGIN, ClassType::Valid));

        // Create first property
        let first_property = Property::<ClassId>::default_with_name(
            PropertyNameLengthConstraint::get().max() as usize,
        );

        // Add first Schema to the Class
        assert_ok!(add_class_schema(
            LEAD_ORIGIN,
            FIRST_CLASS_ID,
            BTreeSet::new(),
            vec![first_property]
        ));

        // Create second property
        let second_property_type =
            PropertyType::<ClassId>::single_text(TextMaxLengthConstraint::get());
        let second_property = Property::<ClassId>::with_name_and_type(
            PropertyNameLengthConstraint::get().max() as usize - 1,
            second_property_type,
            true,
            false,
        );

        // Add second Schema to the Class
        assert_ok!(add_class_schema(
            LEAD_ORIGIN,
            FIRST_CLASS_ID,
            BTreeSet::from_iter(vec![FIRST_PROPERTY_ID].into_iter()),
            vec![second_property]
        ));

        let actor = Actor::Lead;

        // Create entity
        assert_ok!(create_entity(LEAD_ORIGIN, FIRST_CLASS_ID, actor.clone()));

        let mut schema_property_values = BTreeMap::new();
        schema_property_values.insert(FIRST_PROPERTY_ID, InputPropertyValue::default());

        assert_ok!(add_schema_support_to_entity(
            LEAD_ORIGIN,
            actor.clone(),
            FIRST_ENTITY_ID,
            FIRST_SCHEMA_ID,
            schema_property_values.clone(),
        ));

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        schema_property_values.insert(
            SECOND_PROPERTY_ID,
            InputPropertyValue::<Runtime>::single_text(TextMaxLengthConstraint::get()),
        );

        // Make an attempt to add schema support to entity, providing Schema property_values,
        // some of which were already added to this Entity
        let add_schema_support_to_entity_result = add_schema_support_to_entity(
            LEAD_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            SECOND_SCHEMA_ID,
            schema_property_values,
        );

        // Failure checked
        assert_failure(
            add_schema_support_to_entity_result,
            Error::<Runtime>::EntityAlreadyContainsGivenPropertyId,
            number_of_events_before_call,
        );
    })
}

#[test]
fn add_schema_support_is_not_active() {
    with_test_externalities(|| {
        // Create class with default permissions
        assert_ok!(create_simple_class(LEAD_ORIGIN, ClassType::Valid));

        // Create property
        let property = Property::<ClassId>::default_with_name(
            PropertyNameLengthConstraint::get().max() as usize,
        );

        // Add Schema to the Class
        assert_ok!(add_class_schema(
            LEAD_ORIGIN,
            FIRST_CLASS_ID,
            BTreeSet::new(),
            vec![property]
        ));

        let actor = Actor::Lead;

        // Create entity
        assert_ok!(create_entity(LEAD_ORIGIN, FIRST_CLASS_ID, actor.clone()));

        // Make Class Schema inactive
        assert_ok!(update_class_schema_status(
            LEAD_ORIGIN,
            FIRST_CLASS_ID,
            FIRST_SCHEMA_ID,
            false
        ));

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        let mut schema_property_values = BTreeMap::new();
        schema_property_values.insert(FIRST_PROPERTY_ID, InputPropertyValue::default());

        // Make an attempt to add schema support to Entity, providing schema id,
        // which corresponding class Schema is not active
        let add_schema_support_to_entity_result = add_schema_support_to_entity(
            LEAD_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_SCHEMA_ID,
            schema_property_values,
        );

        // Failure checked
        assert_failure(
            add_schema_support_to_entity_result,
            Error::<Runtime>::ClassSchemaNotActive,
            number_of_events_before_call,
        );
    })
}

#[test]
fn add_schema_support_does_not_contain_provided_property_id() {
    with_test_externalities(|| {
        // Create class with default permissions
        assert_ok!(create_simple_class(LEAD_ORIGIN, ClassType::Valid));

        // Create first property
        let first_property = Property::<ClassId>::default_with_name(
            PropertyNameLengthConstraint::get().max() as usize,
        );

        // Add Schema to the Class
        assert_ok!(add_class_schema(
            LEAD_ORIGIN,
            FIRST_CLASS_ID,
            BTreeSet::new(),
            vec![first_property]
        ));

        // Create second property
        let second_property = Property::<ClassId>::default_with_name(
            PropertyNameLengthConstraint::get().max() as usize - 1,
        );

        // Add Schema to the Class
        assert_ok!(add_class_schema(
            LEAD_ORIGIN,
            FIRST_CLASS_ID,
            BTreeSet::new(),
            vec![second_property]
        ));

        let actor = Actor::Lead;

        // Create entity
        assert_ok!(create_entity(LEAD_ORIGIN, FIRST_CLASS_ID, actor.clone()));

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        let mut schema_property_values = BTreeMap::new();
        schema_property_values.insert(SECOND_PROPERTY_ID, InputPropertyValue::default());

        // Make an attempt to add schema support to Entity, providing property values, which are not a members of
        // provided Schema
        let add_schema_support_to_entity_result = add_schema_support_to_entity(
            LEAD_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_SCHEMA_ID,
            schema_property_values,
        );

        // Failure checked
        assert_failure(
            add_schema_support_to_entity_result,
            Error::<Runtime>::SchemaDoesNotContainProvidedPropertyId,
            number_of_events_before_call,
        );
    })
}

#[test]
fn add_schema_support_missing_required_property() {
    with_test_externalities(|| {
        // Create class with default permissions
        assert_ok!(create_simple_class(LEAD_ORIGIN, ClassType::Valid));

        // Create first property
        let first_property = Property::<ClassId>::default_with_name(
            PropertyNameLengthConstraint::get().max() as usize,
        );

        // Create second property
        let second_property_type =
            PropertyType::<ClassId>::single_text(TextMaxLengthConstraint::get());
        let second_property = Property::<ClassId>::with_name_and_type(
            PropertyNameLengthConstraint::get().max() as usize - 1,
            second_property_type,
            true,
            false,
        );

        // Add Schema to the Class
        assert_ok!(add_class_schema(
            LEAD_ORIGIN,
            FIRST_CLASS_ID,
            BTreeSet::new(),
            vec![first_property, second_property]
        ));

        let actor = Actor::Lead;

        // Create entity
        assert_ok!(create_entity(LEAD_ORIGIN, FIRST_CLASS_ID, actor.clone()));

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        let mut schema_property_values = BTreeMap::new();
        schema_property_values.insert(FIRST_PROPERTY_ID, InputPropertyValue::default());

        // Make an attempt to add schema support to Entity, do not providing some of required property values
        let add_schema_support_to_entity_result = add_schema_support_to_entity(
            LEAD_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_SCHEMA_ID,
            schema_property_values,
        );

        // Failure checked
        assert_failure(
            add_schema_support_to_entity_result,
            Error::<Runtime>::MissingRequiredProperty,
            number_of_events_before_call,
        );
    })
}

#[test]
fn add_schema_support_dont_match_type() {
    with_test_externalities(|| {
        // Create class with default permissions
        assert_ok!(create_simple_class(LEAD_ORIGIN, ClassType::Valid));

        // Create property
        let property_type = PropertyType::<ClassId>::single_text(TextMaxLengthConstraint::get());
        let property = Property::<ClassId>::with_name_and_type(
            PropertyNameLengthConstraint::get().max() as usize - 1,
            property_type,
            true,
            false,
        );

        // Add Schema to the Class
        assert_ok!(add_class_schema(
            LEAD_ORIGIN,
            FIRST_CLASS_ID,
            BTreeSet::new(),
            vec![property]
        ));

        let actor = Actor::Lead;

        // Create entity
        assert_ok!(create_entity(LEAD_ORIGIN, FIRST_CLASS_ID, actor.clone()));

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        let mut schema_property_values = BTreeMap::new();
        schema_property_values.insert(FIRST_PROPERTY_ID, InputPropertyValue::default());

        // Make an attempt to add schema support to Entity, providing property values, some of which do not match
        // Class level Property Type
        let add_schema_support_to_entity_result = add_schema_support_to_entity(
            LEAD_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_SCHEMA_ID,
            schema_property_values,
        );

        // Failure checked
        assert_failure(
            add_schema_support_to_entity_result,
            Error::<Runtime>::PropertyValueDoNotMatchType,
            number_of_events_before_call,
        );
    })
}

#[test]
fn add_schema_support_referenced_entity_does_not_match_class() {
    with_test_externalities(|| {
        // Create first class with default permissions
        assert_ok!(create_simple_class(LEAD_ORIGIN, ClassType::Valid));

        // Create second class with default permissions
        assert_ok!(create_simple_class(LEAD_ORIGIN, ClassType::Valid));

        let actor = Actor::Lead;

        // Create first entity
        assert_ok!(create_entity(LEAD_ORIGIN, FIRST_CLASS_ID, actor.to_owned()));

        // Create second entity
        assert_ok!(create_entity(
            LEAD_ORIGIN,
            SECOND_CLASS_ID,
            actor.to_owned()
        ));

        // Create property
        let property_type = PropertyType::<ClassId>::vec_reference(FIRST_CLASS_ID, true, 5);

        let property = Property::<ClassId>::with_name_and_type(
            (PropertyNameLengthConstraint::get().max() - 1) as usize,
            property_type,
            true,
            false,
        );

        // Add Schema to the first Class
        assert_ok!(add_class_schema(
            LEAD_ORIGIN,
            FIRST_CLASS_ID,
            BTreeSet::new(),
            vec![property]
        ));

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        let mut schema_property_values = BTreeMap::new();
        let schema_property_value =
            InputPropertyValue::<Runtime>::vec_reference(vec![SECOND_ENTITY_ID, SECOND_ENTITY_ID]);

        schema_property_values.insert(FIRST_PROPERTY_ID, schema_property_value);

        // Make an attempt to add schema support to the Entity, when provided schema property value(s) refer(s) Entity, which Class
        // does not match the class in corresponding Class Property
        let add_schema_support_to_entity_result = add_schema_support_to_entity(
            LEAD_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_SCHEMA_ID,
            schema_property_values,
        );

        // Failure checked
        assert_failure(
            add_schema_support_to_entity_result,
            Error::<Runtime>::ReferencedEntityDoesNotMatchItsClass,
            number_of_events_before_call,
        );
    })
}

#[test]
fn add_schema_support_referenced_entity_does_not_exist() {
    with_test_externalities(|| {
        // Create first class with default permissions
        assert_ok!(create_simple_class(LEAD_ORIGIN, ClassType::Valid));

        // Create second class with default permissions
        assert_ok!(create_simple_class(LEAD_ORIGIN, ClassType::Valid));

        let actor = Actor::Lead;

        // Create entity
        assert_ok!(create_entity(LEAD_ORIGIN, FIRST_CLASS_ID, actor.to_owned()));

        // Create property
        let property_type = PropertyType::<ClassId>::vec_reference(SECOND_CLASS_ID, true, 5);

        let property = Property::<ClassId>::with_name_and_type(
            (PropertyNameLengthConstraint::get().max() - 1) as usize,
            property_type,
            true,
            false,
        );

        // Add Schema to the first Class
        assert_ok!(add_class_schema(
            LEAD_ORIGIN,
            FIRST_CLASS_ID,
            BTreeSet::new(),
            vec![property]
        ));

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        let mut schema_property_values = BTreeMap::new();
        let schema_property_value = InputPropertyValue::<Runtime>::vec_reference(vec![
            UNKNOWN_ENTITY_ID,
            UNKNOWN_ENTITY_ID,
        ]);

        schema_property_values.insert(FIRST_PROPERTY_ID, schema_property_value);

        // Make an attempt to add schema support to the first entity, if provided property value(s) refer(s) to another Entity,
        // which does not exist in runtime
        let add_schema_support_to_entity_result = add_schema_support_to_entity(
            LEAD_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_SCHEMA_ID,
            schema_property_values,
        );

        // Failure checked
        assert_failure(
            add_schema_support_to_entity_result,
            Error::<Runtime>::EntityNotFound,
            number_of_events_before_call,
        );
    })
}

#[test]
fn add_schema_support_entity_can_not_be_referenced() {
    with_test_externalities(|| {
        // Create first class with default permissions
        assert_ok!(create_simple_class(LEAD_ORIGIN, ClassType::Valid));

        let actor = Actor::Lead;

        // Create first entity
        assert_ok!(create_entity(LEAD_ORIGIN, FIRST_CLASS_ID, actor.to_owned()));

        // Create second entity
        assert_ok!(create_entity(LEAD_ORIGIN, FIRST_CLASS_ID, actor.to_owned()));

        // Update second entity permissions to forbid it from being referencable
        assert_ok!(update_entity_permissions(
            LEAD_ORIGIN,
            SECOND_ENTITY_ID,
            None,
            Some(false)
        ));

        // Create property
        let property_type = PropertyType::<ClassId>::vec_reference(FIRST_CLASS_ID, true, 5);

        let property = Property::<ClassId>::with_name_and_type(
            (PropertyNameLengthConstraint::get().max() - 1) as usize,
            property_type,
            true,
            false,
        );

        // Add Schema to the Class
        assert_ok!(add_class_schema(
            LEAD_ORIGIN,
            FIRST_CLASS_ID,
            BTreeSet::new(),
            vec![property]
        ));

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        let mut schema_property_values = BTreeMap::new();
        let schema_property_value =
            InputPropertyValue::<Runtime>::vec_reference(vec![SECOND_ENTITY_ID, SECOND_ENTITY_ID]);

        schema_property_values.insert(FIRST_PROPERTY_ID, schema_property_value);

        // Make an attempt to add schema support to the first entity, when provided schema property value(s)
        // refer(s) to Entity which can not be referenced
        let add_schema_support_to_entity_result = add_schema_support_to_entity(
            LEAD_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_SCHEMA_ID,
            schema_property_values,
        );

        // Failure checked
        assert_failure(
            add_schema_support_to_entity_result,
            Error::<Runtime>::EntityCanNotBeReferenced,
            number_of_events_before_call,
        );
    })
}

#[test]
fn add_schema_support_same_controller_constraint_violation() {
    with_test_externalities(|| {
        // Create first class with default permissions
        assert_ok!(create_simple_class(LEAD_ORIGIN, ClassType::Valid));

        // Update class permissions to force any member be available to create Entities
        assert_ok!(update_class_permissions(
            LEAD_ORIGIN,
            FIRST_CLASS_ID,
            Some(true),
            None,
            None,
            None
        ));

        let actor = Actor::Lead;

        // Create first entity
        assert_ok!(create_entity(LEAD_ORIGIN, FIRST_CLASS_ID, actor.to_owned()));

        // Create second entity
        assert_ok!(create_entity(
            FIRST_MEMBER_ORIGIN,
            FIRST_CLASS_ID,
            Actor::Member(FIRST_MEMBER_ID)
        ));

        // Create property
        let property_type = PropertyType::<ClassId>::vec_reference(FIRST_CLASS_ID, true, 5);

        let property = Property::<ClassId>::with_name_and_type(
            (PropertyNameLengthConstraint::get().max() - 1) as usize,
            property_type,
            true,
            false,
        );

        // Add Schema to the Class
        assert_ok!(add_class_schema(
            LEAD_ORIGIN,
            FIRST_CLASS_ID,
            BTreeSet::new(),
            vec![property]
        ));

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        let mut schema_property_values = BTreeMap::new();
        let schema_property_value =
            InputPropertyValue::<Runtime>::vec_reference(vec![SECOND_ENTITY_ID, SECOND_ENTITY_ID]);

        schema_property_values.insert(FIRST_PROPERTY_ID, schema_property_value);

        // Make an attempt to add schema support to the first entity, providing reference property value(s) in case,
        // when corresponding Entity can only be referenced from Entity with the same controller.
        let add_schema_support_to_entity_result = add_schema_support_to_entity(
            LEAD_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_SCHEMA_ID,
            schema_property_values,
        );

        // Failure checked
        assert_failure(
            add_schema_support_to_entity_result,
            Error::<Runtime>::SameControllerConstraintViolation,
            number_of_events_before_call,
        );
    })
}

#[test]
fn add_schema_support_text_property_is_too_long() {
    with_test_externalities(|| {
        // Create class with default permissions
        assert_ok!(create_simple_class(LEAD_ORIGIN, ClassType::Valid));

        let actor = Actor::Lead;

        // Create entity
        assert_ok!(create_entity(LEAD_ORIGIN, FIRST_CLASS_ID, actor.to_owned()));

        // Create text property
        let property_type = PropertyType::<ClassId>::single_text(TextMaxLengthConstraint::get());

        let property = Property::<ClassId>::with_name_and_type(
            PropertyNameLengthConstraint::get().max() as usize,
            property_type,
            true,
            false,
        );

        // Add Schema to the first Class
        assert_ok!(add_class_schema(
            LEAD_ORIGIN,
            FIRST_CLASS_ID,
            BTreeSet::new(),
            vec![property]
        ));

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        let mut schema_property_values = BTreeMap::new();

        let schema_property_value =
            InputPropertyValue::<Runtime>::single_text(TextMaxLengthConstraint::get() + 1);

        schema_property_values.insert(FIRST_PROPERTY_ID, schema_property_value);

        // Make an attempt to add schema support to the entity, providing text property value(s), which
        // length exceeds TextMaxLengthConstraint.
        let add_schema_support_to_entity_result = add_schema_support_to_entity(
            LEAD_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_SCHEMA_ID,
            schema_property_values,
        );

        // Failure checked
        assert_failure(
            add_schema_support_to_entity_result,
            Error::<Runtime>::TextPropertyTooLong,
            number_of_events_before_call,
        );
    })
}

#[test]
fn add_schema_support_text_hash_property_is_too_long() {
    with_test_externalities(|| {
        // Create class with default permissions
        assert_ok!(create_simple_class(LEAD_ORIGIN, ClassType::Valid));

        let actor = Actor::Lead;

        // Create entity
        assert_ok!(create_entity(LEAD_ORIGIN, FIRST_CLASS_ID, actor.to_owned()));

        let hashed_text_max_length_constraint = HashedTextMaxLengthConstraint::get();

        // Create hash property
        let property_type =
            PropertyType::<ClassId>::single_text_hash(hashed_text_max_length_constraint);

        let property = Property::<ClassId>::with_name_and_type(
            PropertyNameLengthConstraint::get().max() as usize,
            property_type,
            true,
            false,
        );

        // Add Schema to the first Class
        assert_ok!(add_class_schema(
            LEAD_ORIGIN,
            FIRST_CLASS_ID,
            BTreeSet::new(),
            vec![property]
        ));

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        let mut schema_property_values = BTreeMap::new();

        let schema_property_value = InputPropertyValue::<Runtime>::single_text_to_hash(
            hashed_text_max_length_constraint.unwrap() + 1,
        );

        schema_property_values.insert(FIRST_PROPERTY_ID, schema_property_value);

        // Make an attempt to add schema support to the entity, providing text property value(s), which
        // length exceeds HashedTextMaxLengthConstraint.
        let add_schema_support_to_entity_result = add_schema_support_to_entity(
            LEAD_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_SCHEMA_ID,
            schema_property_values,
        );

        // Failure checked
        assert_failure(
            add_schema_support_to_entity_result,
            Error::<Runtime>::HashedTextPropertyTooLong,
            number_of_events_before_call,
        );
    })
}

#[test]
fn add_schema_support_vec_property_is_too_long() {
    with_test_externalities(|| {
        // Create first class with default permissions
        assert_ok!(create_simple_class(LEAD_ORIGIN, ClassType::Valid));

        // Create second class with default permissions
        assert_ok!(create_simple_class(LEAD_ORIGIN, ClassType::Valid));

        let actor = Actor::Lead;

        // Create first entity
        assert_ok!(create_entity(LEAD_ORIGIN, FIRST_CLASS_ID, actor.to_owned()));

        // Create second entity
        assert_ok!(create_entity(
            LEAD_ORIGIN,
            SECOND_CLASS_ID,
            actor.to_owned()
        ));

        // Create vec property
        let property_type = PropertyType::<ClassId>::vec_reference(
            SECOND_CLASS_ID,
            true,
            VecMaxLengthConstraint::get(),
        );

        let property = Property::<ClassId>::with_name_and_type(
            (PropertyNameLengthConstraint::get().max() - 1) as usize,
            property_type,
            true,
            false,
        );

        // Add Schema to the first Class
        assert_ok!(add_class_schema(
            LEAD_ORIGIN,
            FIRST_CLASS_ID,
            BTreeSet::new(),
            vec![property]
        ));

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        let mut schema_property_values = BTreeMap::new();
        let schema_property_value = InputPropertyValue::<Runtime>::vec_reference(vec![
                SECOND_ENTITY_ID;
                VecMaxLengthConstraint::get() as usize + 1
            ]);

        schema_property_values.insert(FIRST_PROPERTY_ID, schema_property_value);

        // Make an attempt to add schema support to the Entity, providing vector property value(s), which
        // length exceeds VecMaxLengthConstraint.
        let add_schema_support_to_entity_result = add_schema_support_to_entity(
            LEAD_ORIGIN,
            actor,
            FIRST_ENTITY_ID,
            FIRST_SCHEMA_ID,
            schema_property_values,
        );

        // Failure checked
        assert_failure(
            add_schema_support_to_entity_result,
            Error::<Runtime>::VecPropertyTooLong,
            number_of_events_before_call,
        );
    })
}

#[test]
fn add_schema_support_property_should_be_unique() {
    with_test_externalities(|| {
        // Create class with default permissions
        assert_ok!(create_simple_class(LEAD_ORIGIN, ClassType::Valid));

        let actor = Actor::Lead;

        // Create first entity
        assert_ok!(create_entity(LEAD_ORIGIN, FIRST_CLASS_ID, actor.to_owned()));

        // Create second entity
        assert_ok!(create_entity(LEAD_ORIGIN, FIRST_CLASS_ID, actor.to_owned()));

        let property_type = PropertyType::<ClassId>::single_text(TextMaxLengthConstraint::get());

        // Create text property

        let property = Property::<ClassId>::with_name_and_type(
            PropertyNameLengthConstraint::get().max() as usize,
            property_type,
            true,
            true,
        );

        // Add Schema to the Class
        assert_ok!(add_class_schema(
            LEAD_ORIGIN,
            FIRST_CLASS_ID,
            BTreeSet::new(),
            vec![property]
        ));

        let mut schema_property_values = BTreeMap::new();

        let schema_property_value =
            InputPropertyValue::<Runtime>::single_text(TextMaxLengthConstraint::get());

        schema_property_values.insert(FIRST_PROPERTY_ID, schema_property_value);

        // Add Entity Schema support to the first Entity
        assert_ok!(add_schema_support_to_entity(
            LEAD_ORIGIN,
            actor.to_owned(),
            FIRST_ENTITY_ID,
            FIRST_SCHEMA_ID,
            schema_property_values.clone(),
        ));

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        // Make an attempt to add schema support to the Entity, providing property values, which respective Class properties have
        // unique flag set and same property values under same property_ids were already added to any Entity of this Class
        let add_schema_support_to_entity_result = add_schema_support_to_entity(
            LEAD_ORIGIN,
            actor,
            SECOND_ENTITY_ID,
            FIRST_SCHEMA_ID,
            schema_property_values,
        );

        // Failure checked
        assert_failure(
            add_schema_support_to_entity_result,
            Error::<Runtime>::PropertyValueShouldBeUnique,
            number_of_events_before_call,
        );
    })
}

#[test]
fn add_schema_support_properties_should_be_unique() {
    with_test_externalities(|| {
        // Create class with default permissions
        assert_ok!(create_simple_class(LEAD_ORIGIN, ClassType::Valid));

        let actor = Actor::Lead;

        // Create first entity
        assert_ok!(create_entity(LEAD_ORIGIN, FIRST_CLASS_ID, actor.to_owned()));

        // Create second entity
        assert_ok!(create_entity(LEAD_ORIGIN, FIRST_CLASS_ID, actor.to_owned()));

        // Create third entity
        assert_ok!(create_entity(LEAD_ORIGIN, FIRST_CLASS_ID, actor.to_owned()));

        let property_type = PropertyType::<ClassId>::single_text(TextMaxLengthConstraint::get());

        // Create text property

        let property = Property::<ClassId>::with_name_and_type(
            PropertyNameLengthConstraint::get().max() as usize,
            property_type,
            true,
            true,
        );

        // Add Schema to the Class
        assert_ok!(add_class_schema(
            LEAD_ORIGIN,
            FIRST_CLASS_ID,
            BTreeSet::new(),
            vec![property]
        ));

        let mut first_schema_property_values = BTreeMap::new();

        let first_schema_property_value =
            InputPropertyValue::<Runtime>::single_text(TextMaxLengthConstraint::get());

        first_schema_property_values.insert(FIRST_PROPERTY_ID, first_schema_property_value);

        // Add Entity Schema support to the first Entity (property unique on the Class level added)
        assert_ok!(add_schema_support_to_entity(
            LEAD_ORIGIN,
            actor.to_owned(),
            FIRST_ENTITY_ID,
            FIRST_SCHEMA_ID,
            first_schema_property_values.clone(),
        ));

        let mut second_schema_property_values = BTreeMap::new();

        let second_schema_property_value =
            InputPropertyValue::<Runtime>::single_text(TextMaxLengthConstraint::get() - 1);

        second_schema_property_values.insert(FIRST_PROPERTY_ID, second_schema_property_value);

        // Add Entity Schema support to the second Entity (property unique on the Class level added)
        assert_ok!(add_schema_support_to_entity(
            LEAD_ORIGIN,
            actor.to_owned(),
            SECOND_ENTITY_ID,
            FIRST_SCHEMA_ID,
            second_schema_property_values,
        ));

        // Runtime state before tested call

        // Events number before tested call
        let number_of_events_before_call = System::events().len();

        // Make an attempt to add schema support to the Entity, providing property values, which respective Class properties have
        // unique flag set and same property values under same property_ids were already added to any Entity of this Class
        let add_schema_support_to_entity_result = add_schema_support_to_entity(
            LEAD_ORIGIN,
            actor,
            THIRD_ENTITY_ID,
            FIRST_SCHEMA_ID,
            first_schema_property_values,
        );

        // Failure checked
        assert_failure(
            add_schema_support_to_entity_result,
            Error::<Runtime>::PropertyValueShouldBeUnique,
            number_of_events_before_call,
        );
    })
}
