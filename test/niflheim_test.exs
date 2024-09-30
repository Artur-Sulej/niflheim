defmodule NiflheimTest do
  use ExUnit.Case
  alias Niflheim.NifDemo

  test "greets the world" do
    assert NifDemo.init() |> NifDemo.read() == [1, 2, 3]
  end
end
