defmodule Day07.AbbaTest do
  use ExUnit.Case

  alias Day07.Abba

  test "determines whether a string is an ABBA" do
    assert "abba" |> to_char_list |> Abba.abba? == true
    assert "xyyx" |> to_char_list |> Abba.abba? == true
    assert "aaaa" |> to_char_list |> Abba.abba? == false
    assert "abcd" |> to_char_list |> Abba.abba? == false
  end

  test "determines whether a string contains an ABBA" do
    assert "abba"   |> Abba.has_abba? == true
    assert "ioxxoj" |> Abba.has_abba? == true
    assert "zxcvbn" |> Abba.has_abba? == false
  end
end
