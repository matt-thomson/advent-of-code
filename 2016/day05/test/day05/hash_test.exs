defmodule Day05.HashTest do
  use ExUnit.Case

  alias Day05.Hash

  test "computes the hash correctly" do
    hash = Hash.hash("abc", 123) |> Base.encode16 |> String.downcase
    assert hash == "e99a18c428cb38d5f260853678922e03"
  end

  test "determines whether a hash is interesting" do
    assert Hash.interesting?(<<0, 0, 10, 100>>)
    refute Hash.interesting?(<<0, 0, 16, 100>>)
    refute Hash.interesting?(<<0, 1,  0,   0>>)
    refute Hash.interesting?(<<1, 0,  0,   0>>)
  end

  test "selects the password character from a hash" do
    assert Hash.password_char(<<0, 0,  8, 100>>) == "8"
    assert Hash.password_char(<<0, 0, 12, 100>>) == "c"
  end
end
