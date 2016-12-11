defmodule Day07.AddressTest do
  use ExUnit.Case

  alias Day07.Address

  test "parses an address" do
    input  = "abc[def]ghi[jkl]mno"
    output = input |> Address.parse

    assert output.supernet == ["abc", "ghi", "mno"]
    assert output.hypernet == ["def", "jkl"]
  end
end
