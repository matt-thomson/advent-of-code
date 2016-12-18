defmodule Day17Test do
  use ExUnit.Case

  test "solves part one" do
    assert Day17.part_one("ihgpwlah") == "DDRRRD"
    assert Day17.part_one("kglvqrro") == "DDUDRLRRUDRD"
    assert Day17.part_one("ulqzkmiv") == "DRURDRUDDLLDLUURRDULRLDUUDDDRR"
  end

  test "solves part two" do
    assert Day17.part_two("ihgpwlah") == 370
    assert Day17.part_two("kglvqrro") == 492
    assert Day17.part_two("ulqzkmiv") == 830
  end
end
