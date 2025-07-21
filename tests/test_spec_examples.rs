use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

use pretty_assertions::assert_eq;

fn roundtrip_payload<
    T: mimi_content::MimiContentSerialize + mimi_content::MimiContentDeserialize,
>(
    bytes: &[u8],
    skip_asserts: bool,
) -> T {
    let value: ciborium::Value = ciborium::from_reader(bytes).unwrap();
    let mimi_value = T::from_cbor_bytes(bytes).unwrap();
    let buf = mimi_value.to_cbor_bytes().unwrap();
    let value2: ciborium::Value = ciborium::from_reader(&buf[..]).unwrap();
    if !skip_asserts {
        assert_eq!(value, value2);
    }
    mimi_value
}

fn test_zerocopy_equivalence<'a, T, ZCT>(value: &'a T, skip_asserts: bool)
where
    ZCT: mimi_content::MimiContentSerialize,
    T: mimi_content::MimiContentSerialize
        + mimi_content::MimiContentDeserialize
        + mimi_content::MimiContentAsRef<Target<'a> = ZCT>,
{
    let value_cbor = value.to_cbor_bytes().unwrap();
    let value_ref_cbor = value.as_ref().to_cbor_bytes().unwrap();

    if !skip_asserts {
        assert_eq!(value_cbor, value_ref_cbor);
    }
}

macro_rules! test_mimi_content {
    ($testname:ident => $cbor_file:literal) => {
        test_mimi_content!(mimi_content::MimiContent; $testname => $cbor_file);
    };

    (basestruct $struct:path; $testname:ident => $cbor_file:literal) => {
        test_mimi_content!(basestruct $struct; $testname => $cbor_file; skipasserts false);
    };

    (basestruct $struct:path; $testname:ident => $cbor_file:literal; skipasserts $skip_asserts:literal) => {
        #[test]
        #[wasm_bindgen_test]
        fn $testname() {
            const TEST_BYTES: &'static [u8] = include_bytes!($cbor_file);
            let _ = roundtrip_payload::<$struct>(TEST_BYTES, $skip_asserts);
        }
    };

    (
        basestruct $struct:path;
        zerocopy $zerocopy_struct:path;
        $testname:ident => $cbor_file:literal
    ) => {
        test_mimi_content!(
            basestruct $struct;
            zerocopy $zerocopy_struct;
            $testname => $cbor_file;
            skipasserts false
        );
    };

    (
        basestruct $struct:path;
        zerocopy $zerocopy_struct:path;
        $testname:ident => $cbor_file:literal;
        skipasserts $skip_asserts:literal
    ) => {
        #[test]
        #[wasm_bindgen_test]
        fn $testname() {
            const TEST_BYTES: &'static [u8] = include_bytes!($cbor_file);
            let value = roundtrip_payload::<$struct>(TEST_BYTES, $skip_asserts);
            test_zerocopy_equivalence::<$struct, $zerocopy_struct>(&value, $skip_asserts);
        }
    };
}

test_mimi_content!(
    basestruct mimi_content::delivery_report::MessageStatusReport;
    zerocopy mimi_content::delivery_report::MessageStatusReportRef;
    roundtrip_message_status_report => "./examples/report.cbor"
);
test_mimi_content!(
    basestruct mimi_content::derived::MessageDerivedValues;
    zerocopy mimi_content::derived::MessageDerivedValuesRef;
    roundtrip_derived_implied => "./examples/implied-original.cbor"
);
test_mimi_content!(
    basestruct mimi_content::MimiContent;
    zerocopy mimi_content::MimiContentRef;
    roundtrip_original => "./examples/original.cbor"
);
test_mimi_content!(
    basestruct mimi_content::MimiContent;
    zerocopy mimi_content::MimiContentRef;
    roundtrip_reaction => "./examples/reaction.cbor"
);
test_mimi_content!(
    basestruct mimi_content::MimiContent;
    zerocopy mimi_content::MimiContentRef;
    roundtrip_unlike => "./examples/unlike.cbor"
);
test_mimi_content!(
    basestruct mimi_content::MimiContent;
    zerocopy mimi_content::MimiContentRef;
    roundtrip_reply => "./examples/reply.cbor"
);
test_mimi_content!(
    basestruct mimi_content::MimiContent;
    zerocopy mimi_content::MimiContentRef;
    roundtrip_mention => "./examples/mention.cbor"
);
test_mimi_content!(
    basestruct mimi_content::MimiContent;
    zerocopy mimi_content::MimiContentRef;
    roundtrip_mention_html => "./examples/mention-html.cbor"
);
test_mimi_content!(
    basestruct mimi_content::MimiContent;
    zerocopy mimi_content::MimiContentRef;
    roundtrip_attachment => "./examples/attachment.cbor"
);
test_mimi_content!(
    basestruct mimi_content::MimiContent;
    zerocopy mimi_content::MimiContentRef;
    roundtrip_conferencing => "./examples/conferencing.cbor"
);
test_mimi_content!(
    basestruct mimi_content::MimiContent;
    zerocopy mimi_content::MimiContentRef;
    roundtrip_delete => "./examples/delete.cbor"
);
test_mimi_content!(
    basestruct mimi_content::MimiContent;
    zerocopy mimi_content::MimiContentRef;
    roundtrip_edit => "./examples/edit.cbor"
);
test_mimi_content!(
    basestruct mimi_content::MimiContent;
    zerocopy mimi_content::MimiContentRef;
    roundtrip_expiring => "./examples/expiring.cbor"
);
test_mimi_content!(
    basestruct mimi_content::MimiContent;
    zerocopy mimi_content::MimiContentRef;
    roundtrip_multipart_1 => "./examples/multipart-1.cbor"
);
test_mimi_content!(
    basestruct mimi_content::MimiContent;
    zerocopy mimi_content::MimiContentRef;
    roundtrip_multipart_2 => "./examples/multipart-2.cbor"
);
test_mimi_content!(
    basestruct mimi_content::MimiContent;
    zerocopy mimi_content::MimiContentRef;
    roundtrip_multipart_3 => "./examples/multipart-3.cbor"
);

#[test]
fn repro_dual_singlepart_in_multipart_usecase() {
    use mimi_content::{
        MimiContent, MimiContentDeserialize, MimiContentSerialize, MultiPart, NestedPart,
        NestedPartContent, PartSemantics, SinglePart,
    };

    let mut nested_parts = vec![];

    nested_parts.push(NestedPart {
        part_content: NestedPartContent::SinglePart(SinglePart {
            content_type: "text/plain".to_string().into(),
            content: b"Hello World".to_vec().into(),
        }),
        ..Default::default()
    });

    nested_parts.push(NestedPart {
        part_content: NestedPartContent::SinglePart(SinglePart {
            content_type: "text/plain".to_string().into(),
            content: b"Hello World".to_vec().into(),
        }),
        ..Default::default()
    });

    let nested_part = NestedPart::builder()
        .part_content(NestedPartContent::MultiPart(MultiPart {
            parts: nested_parts,
            part_semantics: PartSemantics::SingleUnit,
        }))
        .build();

    let mimi_content = MimiContent::builder()
        .salt_with_rng(&mut rand::thread_rng())
        .topic_id(vec![].into())
        .nested_part(nested_part)
        .build();

    let cbor_bytes = mimi_content.to_cbor_bytes().unwrap();
    let deser_mimi_content = MimiContent::from_cbor_bytes(&cbor_bytes).unwrap();
    assert_eq!(mimi_content, deser_mimi_content);
}
