#include <iostream>
#include <filesystem>
#include <string>
#include "csv_ffi.h"

int main() {
    CSV* csv = csv_new();

    std::string input = std::filesystem::current_path().append("data/examples/output.csv").string();
    csv_read(csv, input.c_str());

    // Get and print rows and columns
    const size_t rows = csv_rows(csv);
    printf("Rows = %ld\n", rows);
    const size_t columns = csv_columns(csv);
    printf("Columns = %ld\n", columns);
    printf("----------\n");

    // Print entire csv
    csv_print(csv);
    printf("----------\n");

    // Get and print csv headers
    size_t headers_len;
    char** headers = csv_get_headers(csv, &headers_len);
    for(size_t i = 0; i < headers_len; i++) {
        printf("%s", headers[i]);
        if(i < headers_len-1)
            printf(", ");
    }
    printf("\n----------\n");

    // Get and print all columns
    for(size_t i = 0; i < headers_len; i++) {
        printf("Column %ld with header %s: ", i+1, headers[i]);
        size_t column_len;
        char** column = csv_get_column(csv, headers[i], &column_len);
        for(size_t j = 0; j < column_len; j++) {
            if(column[j] == nullptr || *column[j] == '\0')
                printf("NULL");
            else
                printf("%s", column[j]);
            if(j < column_len-1)
                printf(", ");
        }
        if(i < headers_len-1)
            printf("\n");
        // Free memory before exiting loop
        for(size_t j = 0; j < column_len; j++) {
            delete column[j];
        }
        delete[] column;
    }
    printf("\n----------\n");

    for(size_t i = 0; i < rows; i++) {
        printf("Row %ld: ", i+1);
        size_t row_len;
        char** row = csv_get_row(csv, i, &row_len);
        for(size_t j = 0; j < row_len; j++) {
            if(row[j] == nullptr || *row[j] == '\0')
                printf("NULL");
            else
                printf("%s", row[j]);
            if(j < row_len-1)
                printf(", ");
        }
        if(i < rows-1)
            printf("\n");
        for(size_t j = 0; j < row_len; j++) {
            delete row[j];
        }
        delete[] row;
    }
    printf("\n----------\n");

    std::string output = std::filesystem::current_path().append("data/examples/output1.csv").string();
    csv_write(csv, output.c_str());

    // Free memory before exit
    csv_free(csv);
    for(size_t i = 0; i < headers_len; i++) {
        delete headers[i];
    }
    delete[] headers;

    return 0;
}
