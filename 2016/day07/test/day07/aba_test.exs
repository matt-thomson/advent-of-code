defmodule Day07.AbaTest do
  use ExUnit.Case

  alias Day07.Aba

  test "parses an ABA string" do
    assert "aba" |> to_char_list |> Aba.aba == ['ab']
    assert "xyx" |> to_char_list |> Aba.aba == ['xy']
    assert "aaa" |> to_char_list |> Aba.aba == []
    assert "abc" |> to_char_list |> Aba.aba == []
  end

  test "finds ABAs in a string" do
    assert "aba"   |> Aba.abas == ['ab']
    assert "zazbz" |> Aba.abas == ['za', 'zb']
    assert "zxcvb" |> Aba.abas == []
  end
end
