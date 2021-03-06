defmodule Day05.IntegersTest do
  use ExUnit.Case

  alias Day05.Integers

  test "provides a stream of integers" do
    output = Integers.stream |> Stream.take(5) |> Enum.to_list
    assert output == [0, 1, 2, 3, 4]
  end
end
