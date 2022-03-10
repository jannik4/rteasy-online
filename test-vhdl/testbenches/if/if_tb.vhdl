LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
USE ieee.numeric_std.ALL;

ENTITY if_tb IS
END if_tb;

ARCHITECTURE tb OF if_tb IS
    SIGNAL clock_p : STD_LOGIC := '0';
    SIGNAL clock_n : STD_LOGIC;
    SIGNAL reset : STD_LOGIC := '0';
    SIGNAL c : STD_LOGIC_VECTOR(0 DOWNTO 0);
    SIGNAL k : STD_LOGIC_VECTOR(0 DOWNTO 0);

    SIGNAL input_IN : unsigned(0 TO 0) := (OTHERS => '0');
    SIGNAL output_OUT : unsigned(0 TO 0);
BEGIN
    -- Clock
    clock_n <= NOT clock_p;

    -- Connect ports
    MAP_EU : ENTITY work.EU_if PORT MAP(
        clock => clock_p,
        c => c,
        k => k,
        input_IN => input_IN,
        output_OUT => output_OUT
        );
    MAP_CU : ENTITY work.CU_if PORT MAP(
        clock => clock_n,
        reset => reset,
        c => c,
        k => k
        );

    Test : PROCESS IS
        PROCEDURE do_reset IS
        BEGIN
            reset <= '1';
            WAIT FOR 100 ns;
            reset <= '0';
            WAIT FOR 100 ns;
        END PROCEDURE;
        PROCEDURE advance_clock IS
        BEGIN
            clock_p <= '1';
            WAIT FOR 100 ns;
            clock_p <= '0';
            WAIT FOR 100 ns;
        END PROCEDURE;
    BEGIN
        -- Reset
        do_reset;

        -- Test false
        input_IN <= "0";
        WAIT FOR 100 ns;
        ASSERT output_OUT = "0";
        advance_clock;
        ASSERT output_OUT = "0";

        -- Reset
        do_reset;

        -- Test true
        input_IN <= "1";
        WAIT FOR 100 ns;
        ASSERT output_OUT = "0";
        advance_clock;
        ASSERT output_OUT = "1";

        -- Finished
        REPORT "Testbench finished";
        WAIT;
    END PROCESS;
END tb;
