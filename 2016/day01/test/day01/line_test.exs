defmodule Day01.LineTest do
  use ExUnit.Case

  alias Day01.Line

  test "calculates where two overlapping line segments meet" do
    line1 = {{1, 3}, {4, 3}}
    line2 = {{2, 1}, {2, 4}}

    assert Line.intersect(line1, line2) == {2, 3}
    assert Line.intersect(line2, line1) == {2, 3}
  end

  test "returns nil for two horizontal line segments" do
    line1 = {{1, 3}, {4, 3}}
    line2 = {{1, 4}, {4, 4}}

    assert Line.intersect(line1, line2) == nil
    assert Line.intersect(line2, line1) == nil
  end

  test "returns nil for two vertical line segments" do
    line1 = {{3, 1}, {3, 4}}
    line2 = {{4, 1}, {4, 4}}

    assert Line.intersect(line1, line2) == nil
    assert Line.intersect(line2, line1) == nil
  end

  test "returns nil when the segments do not overlap" do
    line1 = {{1, 3}, {4, 3}}
    line2 = {{5, 1}, {5, 4}}

    assert Line.intersect(line1, line2) == nil
    assert Line.intersect(line2, line1) == nil
  end
end
