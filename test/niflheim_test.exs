defmodule NiflheimTest do
  use ExUnit.Case
  alias Niflheim.NifDemo

  test "init, read" do
    assert NifDemo.init() |> NifDemo.read() == [1.1, 4.5, 6.7]
  end

  test "add_items" do
    result =
      NifDemo.init()
      |> NifDemo.add_items([4, 5, 6])
      |> NifDemo.read()

    assert result == [1.1, 4.5, 6.7, 4.0, 5.0, 6.0]
  end
end
