defmodule Day17Test do
  use ExUnit.Case

  test "solves part one" do
    assert Day17.part_one("ihgpwlah") == "DDRRRD"
    assert Day17.part_one("kglvqrro") == "DDUDRLRRUDRD"
    assert Day17.part_one("ulqzkmiv") == "DRURDRUDDLLDLUURRDULRLDUUDDDRR"
  end
end
