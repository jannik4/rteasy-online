LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
USE ieee.numeric_std.ALL;

ENTITY bit_range_tb IS
END bit_range_tb;

ARCHITECTURE tb OF bit_range_tb IS
    SIGNAL clock_p : STD_LOGIC := '0';
    SIGNAL clock_n : STD_LOGIC;
    SIGNAL reset : STD_LOGIC := '0';
    SIGNAL c : STD_LOGIC_VECTOR(4 DOWNTO 0);
    SIGNAL k : STD_LOGIC_VECTOR(0 DOWNTO 0);

    SIGNAL input_IN_A : unsigned(7 DOWNTO 0) := (OTHERS => '0');
    SIGNAL input_IN_B : unsigned(0 TO 7) := (OTHERS => '0');
    SIGNAL input_IN_C : unsigned(2 DOWNTO 2) := (OTHERS => '0');
    SIGNAL input_IN_D : unsigned(1 DOWNTO 1) := (OTHERS => '0');
    SIGNAL input_IN_E : unsigned(0 DOWNTO 0) := (OTHERS => '0');
    SIGNAL output_OUT_A : unsigned(7 DOWNTO 0);
    SIGNAL output_OUT_B : unsigned(0 TO 7);
    SIGNAL output_OUT_C : unsigned(2 DOWNTO 2);
    SIGNAL output_OUT_D : unsigned(1 DOWNTO 1);
    SIGNAL output_OUT_E : unsigned(0 DOWNTO 0);
BEGIN
    -- Clock
    clock_n <= NOT clock_p;

    -- Connect ports
    MAP_EU : ENTITY work.EU_bit_range PORT MAP(
        clock => clock_p,
        c => c,
        k => k,
        input_IN_A => input_IN_A,
        input_IN_B => input_IN_B,
        input_IN_C => input_IN_C,
        input_IN_D => input_IN_D,
        input_IN_E => input_IN_E,
        output_OUT_A => output_OUT_A,
        output_OUT_B => output_OUT_B,
        output_OUT_C => output_OUT_C,
        output_OUT_D => output_OUT_D,
        output_OUT_E => output_OUT_E
        );
    MAP_CU : ENTITY work.CU_bit_range PORT MAP(
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

        -- Input
        input_IN_A <= "10101010";
        input_IN_B <= "01011111";
        input_IN_C <= "1";
        input_IN_D <= "1";
        input_IN_E <= "1";

        advance_clock;

        -- Assert
        ASSERT output_OUT_A = "00001010";
        ASSERT output_OUT_B = "01110000";
        ASSERT output_OUT_C = "1";
        ASSERT output_OUT_D = "1";
        ASSERT output_OUT_E = "0";

        -- Finished
        REPORT "Testbench finished";
        WAIT;
    END PROCESS;
END tb;
