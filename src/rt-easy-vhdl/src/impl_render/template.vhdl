LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
USE ieee.numeric_std.ALL;

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
        IF x THEN
            RETURN '1';
        ELSE
            RETURN '0';
        END IF;
    END FUNCTION;

    FUNCTION to_unsigned(x : BOOLEAN) RETURN unsigned IS BEGIN
        IF x THEN
            RETURN to_unsigned(1, 1);
        ELSE
            RETURN to_unsigned(0, 1);
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
USE work.HELPER_{{ module_name }}.ALL;

ENTITY CU_{{ module_name }} IS
    PORT (
        clock : IN STD_LOGIC;
        reset : IN STD_LOGIC;
        c : OUT STD_LOGIC_VECTOR({{ operations.len().checked_sub(1).unwrap_or(0) }} DOWNTO 0);
        k : IN STD_LOGIC_VECTOR({{ criteria.len().checked_sub(1).unwrap_or(0) }} DOWNTO 0)
    );
    ATTRIBUTE KEEP_HIERARCHY : STRING;
    ATTRIBUTE KEEP_HIERARCHY OF CU_{{ module_name }} : ENTITY IS "YES";
END CU_{{ module_name }};

ARCHITECTURE Behavioral OF CU_{{ module_name }} IS
    TYPE state_type IS (
        {% for (idx, statement) in statements.iter().enumerate() %}
            {{ statement.label }}{% if idx != statements.len() - 1 %},{% endif %}
        {% endfor %}
    );
    SIGNAL state, next_state : state_type := {{ statements[0].label }};
BEGIN
    StateReg : PROCESS (clock, reset)
    BEGIN
        IF reset = '1' THEN
            state <= {{ statements[0].label }};
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

                WHEN {{ &statement.label }} =>
                    {% macro render_logic |logic: &NextStateLogic| %}
                        {% match logic %}
                            {% where NextStateLogic::Label(label) %}
                                next_state <= {{ label }};
                            {% endwhere %}
                            {% where NextStateLogic::Cond { conditional, default } %}
                                {% for (idx, (criteria_expr, logic)) in conditional.iter().enumerate() %}
                                {{ if idx == 0 { "IF" } else { "ELSIF" } }} {{ RenderAsVhdl(criteria_expr) }} THEN
                                    {% call render_logic(logic) %}
                                {% endfor %}
                                ELSE
                                    {% call render_logic(&**default) %}
                                END IF;
                            {% endwhere %}
                        {% endmatch %}
                    {% endmacro %}{% call render_logic(&statement.next_state_logic) %}
            {% endfor %}

            -- reset on error
            WHEN OTHERS =>
                next_state <= {{ statements[0].label }};
        END CASE;
    END PROCESS;

    OutputLogic : PROCESS (state, k)
    BEGIN
        c <= (OTHERS => '0');
        CASE state IS
            {% for statement in statements.iter() %}

                WHEN {{ statement.label }} =>
                    {% if statement.operations.is_empty() %}
                        NULL;
                    {% else %}
                        {% for (operation_id, criteria_expr) in statement.operations.iter() %}
                            {% match criteria_expr %}
                                {% where Some(criteria_expr) %}
                                    c({{ operation_id.0 }}) <= to_std_logic({{ RenderAsVhdl(criteria_expr) }});
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
USE work.HELPER_{{ module_name }}.ALL;

ENTITY EU_{{ module_name }} IS
    PORT (
        clock : IN STD_LOGIC;
        c : IN STD_LOGIC_VECTOR({{ operations.len().checked_sub(1).unwrap_or(0) }} DOWNTO 0);
        k : OUT STD_LOGIC_VECTOR({{ criteria.len().checked_sub(1).unwrap_or(0) }} DOWNTO 0){% if self.any_port() %};{% endif %}

        -- Inputs
        {% for (name, range, is_last) in self.ports_input() %}
            input_{{ name }} : IN unsigned{{ RenderAsVhdl(range) }}{% if !is_last %};{% endif %}
        {% endfor %}

        -- Outputs
        {% for (name, range, is_last) in self.ports_output() %}
            output_{{ name }} : OUT unsigned{{ RenderAsVhdl(range) }} := (OTHERS => '0'){% if !is_last %};{% endif %}
        {% endfor %}
    );
    ATTRIBUTE KEEP_HIERARCHY : STRING;
    ATTRIBUTE KEEP_HIERARCHY OF EU_{{ module_name }} : ENTITY IS "YES";
END EU_{{ module_name }};

ARCHITECTURE Behavioral OF EU_{{ module_name }} IS
    ATTRIBUTE KEEP : STRING;

    -- Registers
    {% for (name, range, _) in declarations.registers.iter() %}
        SIGNAL register_{{ name }} : unsigned{{ RenderAsVhdl(*range) }} := (OTHERS => '0');
        ATTRIBUTE KEEP OF register_{{ name }} : SIGNAL IS "TRUE";
    {% endfor %}

    -- Buses
    {% for (name, range, _) in declarations.buses.iter().filter(|(_, _, kind)| *kind == BusKind::Intern) %}
        SIGNAL bus_{{ name }} : unsigned{{ RenderAsVhdl(*range) }} := (OTHERS => '0');
        ATTRIBUTE KEEP OF bus_{{ name }} : SIGNAL IS "TRUE";
    {% endfor %}

    -- Register arrays
    {% for (name, range, length) in &declarations.register_arrays %}
        TYPE type_of_register_array_{{ name }} IS ARRAY(0 TO {{ length - 1 }}) OF unsigned{{ RenderAsVhdl(*range) }};
        SIGNAL register_array_{{ name }} : type_of_register_array_{{ name }} := (
            OTHERS => (OTHERS => '0')
        );
        ATTRIBUTE KEEP OF register_array_{{ name }} : SIGNAL IS "TRUE";
    {% endfor %}

    -- Memories
    {% for (name, ar, dr) in &declarations.memories %}
        TYPE type_of_memory_{{ name }} IS ARRAY(0 TO {{ 2usize.pow(ar.1.size() as u32) - 1 }}) OF unsigned{{ RenderAsVhdl(dr.1) }};
        SIGNAL memory_{{ name }} : type_of_memory_{{ name }} := (
            -- Initialize memory here
            {% if let Some(memory_data) = memories.get(name) %}
                {% for (addr, val) in &memory_data.data %}
                    {{ addr.as_dec() }} => "{{ val.as_bin(true) }}",
                {% endfor %}
            {% endif %}
            OTHERS => (OTHERS => '0')
        );
        ATTRIBUTE KEEP OF memory_{{ name }} : SIGNAL IS "TRUE";
    {% endfor %}
BEGIN
    -- Map registers to output
    {% for (name, _, _) in declarations.registers.iter().filter(|(_, _, kind)| *kind == RegisterKind::Output) %}
        output_{{ name }} <= register_{{ name }};
    {% endfor %}

    -- Unclocked operations
    BusMux : PROCESS {{ self.sensitivity_list_bus_mux() }}
        {% for (idx, range) in self.operations_tmp_var(false) %}
            VARIABLE tmp_c_{{ idx }} : unsigned{{ RenderAsVhdl(range) }};
        {% endfor %}
    BEGIN
        -- Set buses to zero
        {% for (name, _, _) in declarations.buses.iter().filter(|(_, _, kind)| *kind == BusKind::Intern) %}
            bus_{{ name }} <= (OTHERS => '0');
        {% endfor %}

        {% for (idx, operation) in self.operations(false) %}

            -- control signal {{ idx }}: {{ RenderAsRt(operation) }}
            IF c({{ idx }}) = '1' THEN
                {{ RenderAsVhdl((operation, idx)) }}
            END IF;
        {% endfor %}
    END PROCESS;

    -- Clocked operations
    ClockedOp : PROCESS (clock)
        {% for (idx, range) in self.operations_tmp_var(true) %}
            VARIABLE tmp_c_{{ idx }} : unsigned{{ RenderAsVhdl(range) }};
        {% endfor %}
    BEGIN
        IF rising_edge(clock) THEN
            {% for (idx, operation) in self.operations(true) %}

                -- control signal {{ idx }}: {{ RenderAsRt(operation) }}
                IF c({{ idx }}) = '1' THEN
                    {{ RenderAsVhdl((operation, idx)) }}
                END IF;
            {% endfor %}
        END IF;
    END PROCESS;

    -- Generate criteria
    CriteriaGen : PROCESS {{ self.sensitivity_list_criteria_gen() }}
    BEGIN
        {% for (idx, expression) in criteria.iter().enumerate() %}
            -- criterion {{ idx }}: {{ RenderAsRt(expression) }}
            k({{ idx }}) <= to_std_logic({{ RenderAsVhdl(expression) }} = "1");
        {% endfor %}
    END PROCESS;
END Behavioral;
