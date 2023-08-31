use bigdecimal::BigDecimal;
use jni::objects::JValueGen;

use crate::{test, util, wait};

test!(
    fn test_big_decimal() {
        let mut env = util::VM.attach_current_thread()?;
        let num = "-1290000000000000000000000.4167500";
        let rust_num = num.parse().unwrap_or(BigDecimal::from(0));
        let java_string = env.new_string(num)?;
        let java_num = env.new_object(
            "java/math/BigDecimal",
            "(Ljava/lang/String;)V",
            &[JValueGen::Object(&java_string)],
        )?;

        let java_num = jdbc::util::to_string(&mut env, &java_num)?;
        assert_eq!(rust_num.to_string(), java_num);

        for (_, ds) in crate::util::DB_MAP.iter() {
            let conn = wait!(ds.get_connection())?;
            let mut statement = wait!(conn.prepare_statement("select ? as value"))?
                .set_big_decimal(1, &rust_num)?;
            let result = wait!(statement.execute_query())?;
            assert_eq!(result.next()?, true);
            assert_eq!(result.get_string(1)?, Some(num.into()));
            assert_eq!(result.get_big_decimal(1)?.as_ref(), Some(&rust_num));
            assert_eq!(
                result.get_big_decimal_by_label("value")?.as_ref(),
                Some(&rust_num)
            );
        }
    }
);
