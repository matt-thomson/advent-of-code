defmodule Day06.MessageTest do
  use ExUnit.Case

  alias Day06.Message

  test "groups letters correctly" do
    input = ["abc\n", "def\n", "xyz\n"]
    assert input |> Message.group_letters == ['adx', 'bey', 'cfz']
  end

  test "finds the most common character in a group" do
    input = 'aabbaccba'
    assert input |> Message.most_common == 'a'
  end

  test "finds the least common character in a group" do
    input = 'aabbaccba'
    assert input |> Message.least_common == 'c'
  end
end
