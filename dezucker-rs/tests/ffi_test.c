#include "../dezucker.h"
#include <stdio.h>
#include <assert.h>

int main() {
    printf("Starting C FFI verification test...\n");
    // Test basic memory management symbol visibility
    // In a real test we'd link against the library and call them, this just tests we can build against it for now.
    char* test_str = NULL;
    dezucker_free_string(test_str);
    
    struct CFormattedPost* post = NULL;
    dezucker_free_post(post);

    printf("C FFI verification test compiled and ran basic null-checks successfully!\n");
    return 0;
}
