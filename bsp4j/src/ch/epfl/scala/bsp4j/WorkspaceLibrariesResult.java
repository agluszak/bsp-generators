package ch.epfl.scala.bsp4j;

import java.util.List;
import org.eclipse.lsp4j.jsonrpc.validation.NonNull;
import org.eclipse.lsp4j.util.Preconditions;
import org.eclipse.xtext.xbase.lib.Pure;
import org.eclipse.xtext.xbase.lib.util.ToStringBuilder;

@SuppressWarnings("all")
public class WorkspaceLibrariesResult {
  @NonNull
  private List<LibraryItem> libraries;

  public WorkspaceLibrariesResult(@NonNull final List<LibraryItem> libraries) {
    this.libraries = libraries;
  }

  @Pure
  @NonNull
  public List<LibraryItem> getLibraries() {
    return this.libraries;
  }

  public void setLibraries(@NonNull final List<LibraryItem> libraries) {
    this.libraries = Preconditions.checkNotNull(libraries, "libraries");
  }

  @Override
  @Pure
  public String toString() {
    ToStringBuilder b = new ToStringBuilder(this);
    b.add("libraries", this.libraries);
    return b.toString();
  }

  @Override
  @Pure
  public boolean equals(final Object obj) {
    if (this == obj)
      return true;
    if (obj == null)
      return false;
    if (getClass() != obj.getClass())
      return false;
    WorkspaceLibrariesResult other = (WorkspaceLibrariesResult) obj;
    if (this.libraries == null) {
      if (other.libraries != null)
        return false;
    } else if (!this.libraries.equals(other.libraries))
      return false;
    return true;
  }

  @Override
  @Pure
  public int hashCode() {
    return 31 * 1 + ((this.libraries== null) ? 0 : this.libraries.hashCode());
  }
}
