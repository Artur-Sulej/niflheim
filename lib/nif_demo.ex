defmodule Niflheim.NifDemo do
  use Rustler, otp_app: :niflheim, crate: "niflheim_nifdemo"

  # When your NIF is loaded, it will override this function.
  def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
  def init(), do: :erlang.nif_error(:nif_not_loaded)
  def read(_a), do: :erlang.nif_error(:nif_not_loaded)
  def add_items(_resource, _items), do: :erlang.nif_error(:nif_not_loaded)
end
