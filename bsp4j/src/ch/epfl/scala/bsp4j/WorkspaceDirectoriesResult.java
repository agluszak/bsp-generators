package ch.epfl.scala.bsp4j;

import java.util.List;
import org.eclipse.lsp4j.jsonrpc.validation.NonNull;
import org.eclipse.lsp4j.util.Preconditions;
import org.eclipse.xtext.xbase.lib.Pure;
import org.eclipse.xtext.xbase.lib.util.ToStringBuilder;

@SuppressWarnings("all")
public class WorkspaceDirectoriesResult {
  @NonNull
  private List<DirectoryItem> includedDirectories;

  @NonNull
  private List<DirectoryItem> excludedDirectories;

  public WorkspaceDirectoriesResult(@NonNull final List<DirectoryItem> includedDirectories, @NonNull final List<DirectoryItem> excludedDirectories) {
    this.includedDirectories = includedDirectories;
    this.excludedDirectories = excludedDirectories;
  }

  @Pure
  @NonNull
  public List<DirectoryItem> getIncludedDirectories() {
    return this.includedDirectories;
  }

  public void setIncludedDirectories(@NonNull final List<DirectoryItem> includedDirectories) {
    this.includedDirectories = Preconditions.checkNotNull(includedDirectories, "includedDirectories");
  }

  @Pure
  @NonNull
  public List<DirectoryItem> getExcludedDirectories() {
    return this.excludedDirectories;
  }

  public void setExcludedDirectories(@NonNull final List<DirectoryItem> excludedDirectories) {
    this.excludedDirectories = Preconditions.checkNotNull(excludedDirectories, "excludedDirectories");
  }

  @Override
  @Pure
  public String toString() {
    ToStringBuilder b = new ToStringBuilder(this);
    b.add("includedDirectories", this.includedDirectories);
    b.add("excludedDirectories", this.excludedDirectories);
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
    WorkspaceDirectoriesResult other = (WorkspaceDirectoriesResult) obj;
    if (this.includedDirectories == null) {
      if (other.includedDirectories != null)
        return false;
    } else if (!this.includedDirectories.equals(other.includedDirectories))
      return false;
    if (this.excludedDirectories == null) {
      if (other.excludedDirectories != null)
        return false;
    } else if (!this.excludedDirectories.equals(other.excludedDirectories))
      return false;
    return true;
  }

  @Override
  @Pure
  public int hashCode() {
    final int prime = 31;
    int result = 1;
    result = prime * result + ((this.includedDirectories== null) ? 0 : this.includedDirectories.hashCode());
    return prime * result + ((this.excludedDirectories== null) ? 0 : this.excludedDirectories.hashCode());
  }
}
