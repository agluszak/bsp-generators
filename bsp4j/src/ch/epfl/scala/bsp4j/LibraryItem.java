package ch.epfl.scala.bsp4j;

import java.util.List;
import org.eclipse.lsp4j.jsonrpc.validation.NonNull;
import org.eclipse.lsp4j.util.Preconditions;
import org.eclipse.xtext.xbase.lib.Pure;
import org.eclipse.xtext.xbase.lib.util.ToStringBuilder;

@SuppressWarnings("all")
public class LibraryItem {
  @NonNull
  private BuildTargetIdentifier id;

  @NonNull
  private List<BuildTargetIdentifier> dependencies;

  @NonNull
  private List<String> jars;

  public LibraryItem(@NonNull final BuildTargetIdentifier id, @NonNull final List<BuildTargetIdentifier> dependencies, @NonNull final List<String> jars) {
    this.id = id;
    this.dependencies = dependencies;
    this.jars = jars;
  }

  @Pure
  @NonNull
  public BuildTargetIdentifier getId() {
    return this.id;
  }

  public void setId(@NonNull final BuildTargetIdentifier id) {
    this.id = Preconditions.checkNotNull(id, "id");
  }

  @Pure
  @NonNull
  public List<BuildTargetIdentifier> getDependencies() {
    return this.dependencies;
  }

  public void setDependencies(@NonNull final List<BuildTargetIdentifier> dependencies) {
    this.dependencies = Preconditions.checkNotNull(dependencies, "dependencies");
  }

  @Pure
  @NonNull
  public List<String> getJars() {
    return this.jars;
  }

  public void setJars(@NonNull final List<String> jars) {
    this.jars = Preconditions.checkNotNull(jars, "jars");
  }

  @Override
  @Pure
  public String toString() {
    ToStringBuilder b = new ToStringBuilder(this);
    b.add("id", this.id);
    b.add("dependencies", this.dependencies);
    b.add("jars", this.jars);
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
    LibraryItem other = (LibraryItem) obj;
    if (this.id == null) {
      if (other.id != null)
        return false;
    } else if (!this.id.equals(other.id))
      return false;
    if (this.dependencies == null) {
      if (other.dependencies != null)
        return false;
    } else if (!this.dependencies.equals(other.dependencies))
      return false;
    if (this.jars == null) {
      if (other.jars != null)
        return false;
    } else if (!this.jars.equals(other.jars))
      return false;
    return true;
  }

  @Override
  @Pure
  public int hashCode() {
    final int prime = 31;
    int result = 1;
    result = prime * result + ((this.id== null) ? 0 : this.id.hashCode());
    result = prime * result + ((this.dependencies== null) ? 0 : this.dependencies.hashCode());
    return prime * result + ((this.jars== null) ? 0 : this.jars.hashCode());
  }
}
