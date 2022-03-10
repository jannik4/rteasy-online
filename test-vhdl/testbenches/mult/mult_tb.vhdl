LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
USE ieee.numeric_std.ALL;

ENTITY mult_tb IS
END mult_tb;

ARCHITECTURE tb OF mult_tb IS
    SIGNAL clock_p : STD_LOGIC := '0';
    SIGNAL clock_n : STD_LOGIC;
    SIGNAL reset : STD_LOGIC := '0';
    SIGNAL c : STD_LOGIC_VECTOR(5 DOWNTO 0);
    SIGNAL k : STD_LOGIC_VECTOR(0 DOWNTO 0);

    SIGNAL input_INPUT : unsigned(7 DOWNTO 0) := (OTHERS => '0');
    SIGNAL output_OUTPUT : unsigned(7 DOWNTO 0);
BEGIN
    -- Clock
    clock_n <= NOT clock_p;

    -- Connect ports
    MAP_EU : ENTITY work.EU_mult PORT MAP(
        clock => clock_p,
        c => c,
        k => k,
        input_INPUT => input_INPUT,
        output_OUTPUT => output_OUTPUT
        );
    MAP_CU : ENTITY work.CU_mult PORT MAP(
        clock => clock_n,
        reset => reset,
        c => c,
        k => k
        );

    Test : PROCESS IS
        PROCEDURE do_reset IS
        BEGIN
            WAIT FOR 50 ns;
            reset <= '1';
            WAIT FOR 100 ns;
            reset <= '0';
            WAIT FOR 50 ns;
        END PROCEDURE;
        PROCEDURE advance_clock(amount : INTEGER := 1) IS
        BEGIN
            FOR i IN 1 TO amount LOOP
                WAIT FOR 50 ns;
                clock_p <= '1';
                WAIT FOR 100 ns;
                clock_p <= '0';
                WAIT FOR 50 ns;
            END LOOP;
        END PROCEDURE;
    BEGIN
        -- Test (0..=10) * (0..=10)
        FOR a IN 0 TO 10 LOOP
            FOR b IN 0 TO 10 LOOP
                -- Reset
                do_reset;

                -- Test a * b

                -- Input
                input_INPUT <= to_unsigned(a, 8);
                advance_clock;
                input_INPUT <= to_unsigned(b, 8);
                advance_clock;

                -- Mult
                advance_clock(b + 1);

                -- Assert
                ASSERT output_OUTPUT = a * b;
            END LOOP;
        END LOOP;

        -- Finished
        REPORT "Testbench finished";
        WAIT;
    END PROCESS;
END tb;
