LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
USE ieee.numeric_std.ALL;

LIBRARY work;

PACKAGE HELPER_{{ module_name }} IS
    -- helper
    FUNCTION to_std_logic(x : BOOLEAN) RETURN STD_LOGIC;
    FUNCTION to_unsigned(x : BOOLEAN) RETURN unsigned;

    -- extend
    FUNCTION zero_extend(in0 : unsigned; len : INTEGER) RETURN unsigned;
    FUNCTION sign_extend(in0 : unsigned; len : INTEGER) RETURN unsigned;

    -- binary operators
    FUNCTION f_eq(in0 : unsigned; in1 : unsigned) RETURN unsigned;
    FUNCTION f_ne(in0 : unsigned; in1 : unsigned) RETURN unsigned;
    FUNCTION f_le(in0 : unsigned; in1 : unsigned) RETURN unsigned;
    FUNCTION f_le_s(in0 : unsigned; in1 : unsigned) RETURN unsigned;
    FUNCTION f_lt(in0 : unsigned; in1 : unsigned) RETURN unsigned;
    FUNCTION f_lt_s(in0 : unsigned; in1 : unsigned) RETURN unsigned;
    FUNCTION f_ge(in0 : unsigned; in1 : unsigned) RETURN unsigned;
    FUNCTION f_ge_s(in0 : unsigned; in1 : unsigned) RETURN unsigned;
    FUNCTION f_gt(in0 : unsigned; in1 : unsigned) RETURN unsigned;
    FUNCTION f_gt_s(in0 : unsigned; in1 : unsigned) RETURN unsigned;
    FUNCTION f_add(in0 : unsigned; in1 : unsigned) RETURN unsigned;
    FUNCTION f_sub(in0 : unsigned; in1 : unsigned) RETURN unsigned;
    FUNCTION f_and(in0 : unsigned; in1 : unsigned) RETURN unsigned;
    FUNCTION f_nand(in0 : unsigned; in1 : unsigned) RETURN unsigned;
    FUNCTION f_or(in0 : unsigned; in1 : unsigned) RETURN unsigned;
    FUNCTION f_nor(in0 : unsigned; in1 : unsigned) RETURN unsigned;
    FUNCTION f_xor(in0 : unsigned; in1 : unsigned) RETURN unsigned;

    -- unary operators
    FUNCTION f_neg(in0 : unsigned) RETURN unsigned;
    FUNCTION f_not(in0 : unsigned) RETURN unsigned;
    FUNCTION f_sxt(in0 : unsigned) RETURN unsigned;
END PACKAGE HELPER_{{ module_name }};

PACKAGE BODY HELPER_{{ module_name }} IS
    -- helper
    FUNCTION to_std_logic(x : BOOLEAN) RETURN STD_LOGIC IS BEGIN
        IF x THEN RETURN '1';
        ELSE      RETURN '0';
        END IF;
    END FUNCTION;

    FUNCTION to_unsigned(x : BOOLEAN) RETURN unsigned IS BEGIN
        IF x THEN RETURN to_unsigned(1, 1);
        ELSE      RETURN to_unsigned(0, 1);
        END IF;
    END FUNCTION;

    -- extend
    FUNCTION zero_extend(in0 : unsigned; len : INTEGER) RETURN unsigned IS BEGIN
        RETURN resize(in0, len);
    END FUNCTION;

    FUNCTION sign_extend(in0 : unsigned; len : INTEGER) RETURN unsigned IS BEGIN
        RETURN unsigned(resize(signed(in0), len));
    END FUNCTION;

    -- binary operators
    FUNCTION f_eq(in0 : unsigned; in1 : unsigned) RETURN unsigned IS BEGIN
        RETURN to_unsigned(in0 = in1);
    END FUNCTION;

    FUNCTION f_ne(in0 : unsigned; in1 : unsigned) RETURN unsigned IS BEGIN
        RETURN to_unsigned(in0 /= in1);
    END FUNCTION;

    FUNCTION f_le(in0 : unsigned; in1 : unsigned) RETURN unsigned IS BEGIN
        RETURN to_unsigned(in0 <= in1);
    END FUNCTION;
    FUNCTION f_le_s(in0 : unsigned; in1 : unsigned) RETURN unsigned IS BEGIN
        RETURN to_unsigned(signed(in0) <= signed(in1));
    END FUNCTION;

    FUNCTION f_lt(in0 : unsigned; in1 : unsigned) RETURN unsigned IS BEGIN
        RETURN to_unsigned(in0 < in1);
    END FUNCTION;
    FUNCTION f_lt_s(in0 : unsigned; in1 : unsigned) RETURN unsigned IS BEGIN
        RETURN to_unsigned(signed(in0) < signed(in1));
    END FUNCTION;

    FUNCTION f_ge(in0 : unsigned; in1 : unsigned) RETURN unsigned IS BEGIN
        RETURN to_unsigned(in0 >= in1);
    END FUNCTION;
    FUNCTION f_ge_s(in0 : unsigned; in1 : unsigned) RETURN unsigned IS BEGIN
        RETURN to_unsigned(signed(in0) >= signed(in1));
    END FUNCTION;

    FUNCTION f_gt(in0 : unsigned; in1 : unsigned) RETURN unsigned IS BEGIN
        RETURN to_unsigned(in0 > in1);
    END FUNCTION;
    FUNCTION f_gt_s(in0 : unsigned; in1 : unsigned) RETURN unsigned IS BEGIN
        RETURN to_unsigned(signed(in0) > signed(in1));
    END FUNCTION;

    FUNCTION f_add(in0 : unsigned; in1 : unsigned) RETURN unsigned IS BEGIN
        RETURN in0 + in1;
    END FUNCTION;

    FUNCTION f_sub(in0 : unsigned; in1 : unsigned) RETURN unsigned IS BEGIN
        RETURN in0 - in1;
    END FUNCTION;

    FUNCTION f_and(in0 : unsigned; in1 : unsigned) RETURN unsigned IS BEGIN
        RETURN in0 AND in1;
    END FUNCTION;

    FUNCTION f_nand(in0 : unsigned; in1 : unsigned) RETURN unsigned IS BEGIN
        RETURN in0 NAND in1;
    END FUNCTION;

    FUNCTION f_or(in0 : unsigned; in1 : unsigned) RETURN unsigned IS BEGIN
        RETURN in0 OR in1;
    END FUNCTION;

    FUNCTION f_nor(in0 : unsigned; in1 : unsigned) RETURN unsigned IS BEGIN
        RETURN in0 NOR in1;
    END FUNCTION;

    FUNCTION f_xor(in0 : unsigned; in1 : unsigned) RETURN unsigned IS BEGIN
        RETURN in0 XOR in1;
    END FUNCTION;

    -- unary operators
    FUNCTION f_neg(in0 : unsigned) RETURN unsigned IS BEGIN
        RETURN (NOT in0) + 1;
    END FUNCTION;

    FUNCTION f_not(in0 : unsigned) RETURN unsigned IS BEGIN
        RETURN NOT in0;
    END FUNCTION;

    FUNCTION f_sxt(in0 : unsigned) RETURN unsigned IS BEGIN
        RETURN in0;
    END FUNCTION;
END PACKAGE BODY HELPER_{{ module_name }};

-------------------------------------------------------------------------------

LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
USE ieee.numeric_std.ALL;

ENTITY CU_{{ module_name }} IS
    PORT (
        clock : IN STD_LOGIC;
        reset : IN STD_LOGIC;
        c : OUT STD_LOGIC_VECTOR({{ operations.len() }} DOWNTO 0);
        k : IN STD_LOGIC_VECTOR({{ criteria.len() }} DOWNTO 0)
    );
    ATTRIBUTE KEEP_HIERARCHY : STRING;
    ATTRIBUTE KEEP_HIERARCHY OF CU_{{ module_name }} : ENTITY IS "YES";
END CU_{{ module_name }};

ARCHITECTURE Behavioral OF CU_{{ module_name }} IS
    TYPE state_type IS (
        {% for (idx, statement) in statements.iter().enumerate() %}
        {{ RenderLabel(&statement.label) }}{% if idx != statements.len() - 1 %},{% endif %}
        {% endfor %}
    );
    SIGNAL state, next_state : state_type := {{ RenderLabel(&statements[0].label) }};
BEGIN
    StateReg : PROCESS (clock, reset)
    BEGIN
        IF reset = '1' THEN
            state <= {{ RenderLabel(&statements[0].label) }};
        ELSE
            IF rising_edge(clock) THEN
                state <= next_state;
            END IF;
        END IF;
    END PROCESS;

    NextStateLogic : PROCESS (state, k)
    BEGIN
        CASE state IS
            {% for statement in statements.iter() %}

            WHEN {{ RenderLabel(&statement.label) }} =>
                {% if statement.next_state_conditional.is_empty() %}
                next_state <= {{ RenderLabel(&statement.next_state_default) }};
                {% else %}
                {% for (idx, (label, criteria_expr)) in statement.next_state_conditional.iter().enumerate() %}
                {{ if idx == 0 { "IF" } else { "ELSIF" } }} {{ RenderCriteriaExpr(criteria_expr) }} THEN
                    next_state <= {{ RenderLabel(label) }};
                {% endfor %}
                ELSE
                    next_state <= {{ RenderLabel(&statement.next_state_default) }};
                END IF;
                {% endif %}
            {% endfor %}

            -- reset on error
            WHEN OTHERS =>
                next_state <= {{ RenderLabel(&statements[0].label) }};
        END CASE;
    END PROCESS;

    OutputLogic : PROCESS (state, k)
    BEGIN
        c <= (OTHERS => '0');
        CASE state IS
            {% for statement in statements.iter() %}

            WHEN {{ RenderLabel(&statement.label) }} =>
                {% if statement.operations.is_empty() %}
                NULL;
                {% else %}
                {% for (operation_id, criteria_expr) in statement.operations.iter() %}
                {% match criteria_expr %}
                    {% where Some(criteria_expr) %}
                        c({{ operation_id.0 }}) <= to_std_logic({{ RenderCriteriaExpr(criteria_expr) }});
                    {% endwhere %}
                    {% where None %}
                        c({{ operation_id.0 }}) <= '1';
                    {% endwhere %}
                {% endmatch %}
                {% endfor %}
                {% endif %}
            {% endfor %}

            WHEN OTHERS =>
                NULL;
        END CASE;
    END PROCESS;
END Behavioral;

-------------------------------------------------------------------------------

LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
USE ieee.numeric_std.ALL;

ENTITY EU_{{ module_name }} IS
    PORT (
        clock : IN STD_LOGIC;
        c : IN STD_LOGIC_VECTOR({{ operations.len() }} DOWNTO 0);
        k : OUT STD_LOGIC_VECTOR({{ criteria.len() }} DOWNTO 0);
        -- TODO: Generate all inputs
        -- TODO: Generate all outputs
    );
    ATTRIBUTE KEEP_HIERARCHY : STRING;
    ATTRIBUTE KEEP_HIERARCHY OF EU_{{ module_name }} : ENTITY IS "YES";
END EU_{{ module_name }};

ARCHITECTURE Behavioral OF EU_{{ module_name }} IS
    ATTRIBUTE KEEP : STRING;

    -- TODO: Generate (only intern?) registers

    -- TODO: Generate intern buses

    -- TODO: Generate reg arrays

    -- TODO: Generate memories
BEGIN
    BusMux : PROCESS ( *)
    BEGIN
        -- TODO: Set all internal buses to zero
        
        {% for (idx, operation) in operations.iter().enumerate().filter(|(_, op)| !op.is_clocked()) %}

        -- control signal {{ idx }}: {{ Fmt(operation) }}
        IF c({{ idx }}) = '1' THEN
            {{ RenderOperation { operation, _p: std::marker::PhantomData } }}
        END IF;
        {% endfor %}
    END PROCESS;

    ClockedOp : PROCESS (clock)
    BEGIN
        IF rising_edge(clock) THEN
            {% for (idx, operation) in operations.iter().enumerate().filter(|(_, op)| op.is_clocked()) %}

            -- control signal {{ idx }}: {{ Fmt(operation) }}
            IF c({{ idx }}) = '1' THEN
                {{ RenderOperation { operation, _p: std::marker::PhantomData } }}
            END IF;
            {% endfor %}
        END IF;
    END PROCESS;
    
    ConditionGen : PROCESS ( *)
    BEGIN
        {% for (idx, expression) in criteria.iter().enumerate() %}
        -- criterion {{ idx }}: {{ Fmt(expression) }}
        k({{ idx }}) <= to_std_logic({{ RenderExpression { expression, ctx_size: 1 } }});
        {% endfor %}
    END PROCESS;
END Behavioral;
