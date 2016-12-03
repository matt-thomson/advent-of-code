defmodule Day02.PositionTest do
  use ExUnit.Case

  alias Day02.Position

  test "translates co-ordinates to buttons for part one" do
    assert Position.part_one({0, 0}) == 1
    assert Position.part_one({1, 1}) == 5
    assert Position.part_one({2, 2}) == 9
  end

  test "returns nil when out of range for part one" do
    assert Position.part_one({-1,  1}) == nil
    assert Position.part_one({ 1, -1}) == nil
    assert Position.part_one({ 3,  1}) == nil
    assert Position.part_one({ 1,  3}) == nil
  end

  test "translates co-ordinates to buttons for part two" do
    assert Position.part_two({0, 2}) == 1
    assert Position.part_two({1, 1}) == 2
    assert Position.part_two({2, 0}) == 5
    assert Position.part_two({2, 4}) == 9
    assert Position.part_two({3, 3}) == 12
    assert Position.part_two({4, 2}) == 13
  end

  test "returns nil when out of range for part two" do
    assert Position.part_two({ 0,  1}) == nil
    assert Position.part_two({ 0,  3}) == nil
    assert Position.part_two({ 1,  0}) == nil
    assert Position.part_two({ 1,  4}) == nil
    assert Position.part_two({ 2, -1}) == nil
    assert Position.part_two({ 2,  5}) == nil
    assert Position.part_two({ 3,  0}) == nil
    assert Position.part_two({ 3,  4}) == nil
    assert Position.part_two({ 4,  1}) == nil
    assert Position.part_two({ 4,  3}) == nil
  end
end
