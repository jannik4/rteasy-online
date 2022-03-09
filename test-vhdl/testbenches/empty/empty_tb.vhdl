LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
USE ieee.numeric_std.ALL;

ENTITY empty_tb IS
END empty_tb;

ARCHITECTURE tb OF empty_tb IS
    SIGNAL clock_p : STD_LOGIC := '0';
    SIGNAL clock_n : STD_LOGIC := '0';
    SIGNAL reset : STD_LOGIC;
    SIGNAL c : STD_LOGIC_VECTOR(0 DOWNTO 0);
    SIGNAL k : STD_LOGIC_VECTOR(0 DOWNTO 0);

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

    Test : PROCESS
    BEGIN
        -- Reset
        reset <= '1';
        WAIT FOR 100 ns;
        reset <= '0';
        WAIT FOR 100 ns;

        -- Test
        advance_clock(clock_p);
        advance_clock(clock_p);

        -- Finished
        REPORT "Testbench finished";
        WAIT;
    END PROCESS;
END tb;
