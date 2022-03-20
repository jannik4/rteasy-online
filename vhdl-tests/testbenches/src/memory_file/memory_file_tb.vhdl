LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
USE ieee.numeric_std.ALL;

ENTITY memory_file_tb IS
END memory_file_tb;

ARCHITECTURE tb OF memory_file_tb IS
    SIGNAL clock_p : STD_LOGIC := '0';
    SIGNAL clock_n : STD_LOGIC;
    SIGNAL reset : STD_LOGIC := '0';
    SIGNAL c : STD_LOGIC_VECTOR(17 DOWNTO 0);
    SIGNAL k : STD_LOGIC_VECTOR(0 DOWNTO 0);

    SIGNAL output_OUT : unsigned(7 DOWNTO 0);
BEGIN
    -- Clock
    clock_n <= NOT clock_p;

    -- Connect ports
    MAP_EU : ENTITY work.EU_memory_file PORT MAP(
        clock => clock_p,
        c => c,
        k => k,
        output_OUT => output_OUT
        );
    MAP_CU : ENTITY work.CU_memory_file PORT MAP(
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
        -- Reset
        do_reset;

        -- Test MEM_A
        advance_clock(3);
        ASSERT output_OUT = 16#FB#;
        advance_clock(3);
        ASSERT output_OUT = 16#4#;
        advance_clock(3);
        ASSERT output_OUT = 16#0#;
        advance_clock(3);
        ASSERT output_OUT = 16#7#;
        advance_clock(3);
        ASSERT output_OUT = 16#A#;

        -- Test MEM_A
        advance_clock(3);
        ASSERT output_OUT = "00001111";
        advance_clock(3);
        ASSERT output_OUT = "11110011";

        -- Test MEM_C
        advance_clock(3);
        ASSERT output_OUT = 1;
        advance_clock(3);
        ASSERT output_OUT = 1;
        advance_clock(3);
        ASSERT output_OUT = 0;
        advance_clock(3);
        ASSERT output_OUT = 1;
        advance_clock(3);
        ASSERT output_OUT = 0;

        -- Finished
        REPORT "Testbench finished";
        WAIT;
    END PROCESS;
END tb;
