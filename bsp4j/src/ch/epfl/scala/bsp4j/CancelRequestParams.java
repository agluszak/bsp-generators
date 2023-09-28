package ch.epfl.scala.bsp4j;

import org.eclipse.lsp4j.jsonrpc.validation.NonNull;
import org.eclipse.lsp4j.util.Preconditions;
import org.eclipse.xtext.xbase.lib.Pure;
import org.eclipse.xtext.xbase.lib.util.ToStringBuilder;

@SuppressWarnings("all")
public class CancelRequestParams {
  @NonNull private String id;

  public CancelRequestParams(@NonNull final String id) {
    this.id = id;
  }

  @Pure
  @NonNull
  public String getId() {
    return this.id;
  }

  public void setId(@NonNull final String id) {
    this.id = Preconditions.checkNotNull(id, "id");
  }

  @Override
  @Pure
  public String toString() {
    ToStringBuilder b = new ToStringBuilder(this);
    b.add("id", this.id);
    return b.toString();
  }

  @Override
  @Pure
  public boolean equals(final Object obj) {
    if (this == obj) return true;
    if (obj == null) return false;
    if (getClass() != obj.getClass()) return false;
    CancelRequestParams other = (CancelRequestParams) obj;
    if (this.id == null) {
      if (other.id != null) return false;
    } else if (!this.id.equals(other.id)) return false;
    return true;
  }

  @Override
  @Pure
  public int hashCode() {
    return 31 * 1 + ((this.id == null) ? 0 : this.id.hashCode());
  }
}
