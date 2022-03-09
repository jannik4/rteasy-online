LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
USE ieee.numeric_std.ALL;

ENTITY add_tb IS
END add_tb;

ARCHITECTURE tb OF add_tb IS
    SIGNAL clock_p : STD_LOGIC := '0';
    SIGNAL clock_n : STD_LOGIC := '0';
    SIGNAL reset : STD_LOGIC;
    SIGNAL c : STD_LOGIC_VECTOR(0 DOWNTO 0);
    SIGNAL k : STD_LOGIC_VECTOR(0 DOWNTO 0);

    SIGNAL input_A : unsigned(7 DOWNTO 0);
    SIGNAL input_B : unsigned(7 DOWNTO 0);
    SIGNAL output_OUT : unsigned(7 DOWNTO 0);

    PROCEDURE advance_clock(SIGNAL clock : OUT STD_LOGIC) IS
    BEGIN
        clock <= '1';
        WAIT FOR 100 ns;
        clock <= '0';
        WAIT FOR 100 ns;
    END PROCEDURE;
BEGIN
    -- Clock
    clock_n <= NOT clock_p;

    -- Connect ports
    MAP_EU : ENTITY work.EU_add PORT MAP(
        clock => clock_p,
        c => c,
        k => k,
        input_A => input_A,
        input_B => input_B,
        output_OUT => output_OUT
        );
    MAP_CU : ENTITY work.CU_add PORT MAP(
        clock => clock_n,
        reset => reset,
        c => c,
        k => k
        );

    Test : PROCESS
    BEGIN
        -- Reset
        reset <= '1';
        WAIT FOR 100 ns;
        reset <= '0';
        WAIT FOR 100 ns;

        -- Test 4 + 3
        input_A <= to_unsigned(4, 8);
        input_B <= to_unsigned(3, 8);
        WAIT FOR 100 ns;
        advance_clock(clock_p);
        ASSERT output_OUT = 7;

        -- Reset
        reset <= '1';
        WAIT FOR 100 ns;
        reset <= '0';
        WAIT FOR 100 ns;

        -- Test 255 + 7 (overflow)
        input_A <= to_unsigned(255, 8);
        input_B <= to_unsigned(7, 8);
        WAIT FOR 100 ns;
        advance_clock(clock_p);
        ASSERT output_OUT = 6;

        -- Finished
        REPORT "Testbench finished";
        WAIT;
    END PROCESS;
END tb;
