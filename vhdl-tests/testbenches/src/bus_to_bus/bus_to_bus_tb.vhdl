LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
USE ieee.numeric_std.ALL;

ENTITY bus_to_bus_tb IS
END bus_to_bus_tb;

ARCHITECTURE tb OF bus_to_bus_tb IS
    SIGNAL clock_p : STD_LOGIC := '0';
    SIGNAL clock_n : STD_LOGIC;
    SIGNAL reset : STD_LOGIC := '0';
    SIGNAL c : STD_LOGIC_VECTOR(8 DOWNTO 0);
    SIGNAL k : STD_LOGIC_VECTOR(0 DOWNTO 0);

    SIGNAL output_OUTPUT : unsigned(7 DOWNTO 0);
BEGIN
    -- Clock
    clock_n <= NOT clock_p;

    -- Connect ports
    MAP_EU : ENTITY work.EU_bus_to_bus PORT MAP(
        clock => clock_p,
        c => c,
        k => k,
        output_OUTPUT => output_OUTPUT
        );
    MAP_CU : ENTITY work.CU_bus_to_bus PORT MAP(
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

        -- Test
        advance_clock;
        ASSERT output_OUTPUT = 1;
        advance_clock;
        ASSERT output_OUTPUT = 6;

        advance_clock;
        ASSERT output_OUTPUT = "01011010";

        -- Finished
        REPORT "Testbench finished";
        WAIT;
    END PROCESS;
END tb;
