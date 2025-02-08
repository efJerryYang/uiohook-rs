/* Only for testing MacOS API */
#include <stdio.h>
#include <dlfcn.h>

// gcc -o check_ax check_ax.c -framework CoreFoundation -framework ApplicationServices
int main(void) {
    void *handle = dlopen("/System/Library/Frameworks/ApplicationServices.framework/ApplicationServices", RTLD_NOW);
    if (!handle) {
        printf("dlopen failed: %s\n", dlerror());
        return 1;
    }
    
    dlerror();

    void *axFunc = dlsym(handle, "AXIsProcessTrustedWithOptions");
    char *error = dlerror();
    if (error != NULL) {
        printf("AXIsProcessTrustedWithOptions Not Found: %s\n", error);
    } else {
        printf("AXIsProcessTrustedWithOptions Found, Addr: %p\n", axFunc);
    }
    
    dlclose(handle);
    return 0;
}

