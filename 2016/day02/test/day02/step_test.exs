defmodule Day02.StepTest do
  use ExUnit.Case

  alias Day02.Step

  test "steps up correctly" do
    assert Step.step(:up, {0, 0}) == {0, 0}
    assert Step.step(:up, {1, 1}) == {0, 1}
    assert Step.step(:up, {2, 2}) == {1, 2}
  end

  test "steps down correctly" do
    assert Step.step(:down, {0, 0}) == {1, 0}
    assert Step.step(:down, {1, 1}) == {2, 1}
    assert Step.step(:down, {2, 2}) == {2, 2}
  end

  test "steps left correctly" do
    assert Step.step(:left, {0, 0}) == {0, 0}
    assert Step.step(:left, {1, 1}) == {1, 0}
    assert Step.step(:left, {2, 2}) == {2, 1}
  end

  test "steps right correctly" do
    assert Step.step(:right, {0, 0}) == {0, 1}
    assert Step.step(:right, {1, 1}) == {1, 2}
    assert Step.step(:right, {2, 2}) == {2, 2}
  end
end
