LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
USE ieee.numeric_std.ALL;

ENTITY empty_tb IS
END empty_tb;

ARCHITECTURE tb OF empty_tb IS
    SIGNAL clock_p : STD_LOGIC := '0';
    SIGNAL clock_n : STD_LOGIC;
    SIGNAL reset : STD_LOGIC := '0';
    SIGNAL c : STD_LOGIC_VECTOR(0 DOWNTO 0);
    SIGNAL k : STD_LOGIC_VECTOR(0 DOWNTO 0);
BEGIN
    -- Clock
    clock_n <= NOT clock_p;

    -- Connect ports
    MAP_EU : ENTITY work.EU_empty PORT MAP(
        clock => clock_p,
        c => c,
        k => k
        );
    MAP_CU : ENTITY work.CU_empty PORT MAP(
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

        -- Test
        advance_clock;
        advance_clock;

        -- Finished
        REPORT "Testbench finished";
        WAIT;
    END PROCESS;
END tb;
