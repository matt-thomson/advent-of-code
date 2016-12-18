defmodule Day18.RowTest do
  use ExUnit.Case

  alias Day18.Row

  test "parses a row" do
    assert "^..^^." |> Row.parse == [:trap, :safe, :safe, :trap, :trap, :safe]
  end

  test "takes a step from one row to the next" do
    row = "..^^." |> Row.parse
    assert row |> Row.step == [:safe, :trap, :trap, :trap, :trap]
  end

  test "counts the number of safe tiles" do
    row = "..^^." |> Row.parse
    assert row |> Row.count_safe == 3
  end
end
