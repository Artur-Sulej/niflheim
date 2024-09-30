defmodule Niflheim.NifDemo do
  use Rustler, otp_app: :niflheim, crate: "niflheim_nifdemo"

  # When your NIF is loaded, it will override this function.
  def add(_a, _b), do: error()
  def init(), do: error()
  def read(_resource), do: error()
  def add_items(_resource, _items), do: error()
  def get_data(_resource), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end
