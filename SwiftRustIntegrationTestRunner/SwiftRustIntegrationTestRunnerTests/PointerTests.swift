//
//  PointerTests.swift
//  SwiftRustIntegrationTestRunnerTests
//
//  Created by Frankie Nwafili on 11/22/21.
//

import XCTest
import Foundation
@testable import SwiftRustIntegrationTestRunner

class PointerTests: XCTestCase {
    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    func testSwiftCallRustCvoid() throws {
        var u8 = UInt8(4)
        
        withUnsafePointer(to: u8, {(value) in
            let pointer = UnsafeRawPointer(value)
            let pointer_mut = UnsafeMutableRawPointer(mutating: value)
            let pointer_nil: UnsafeRawPointer? = nil
            let pointer_mut_nil: UnsafeMutableRawPointer? = nil
            
            let pointer_copy = rust_echo_const_c_void(pointer)
            let pointer_nil_copy = rust_echo_const_c_void(pointer_nil)
            let pointer_mut_copy = rust_echo_mut_c_void(pointer_mut)
            let pointer_mut_copy2 = rust_echo_optional_non_null_c_void(pointer_mut)
            let pointer_mut_nil_copy = rust_echo_mut_c_void(pointer_mut_nil)
            let pointer_mut_nil_copy2 = rust_echo_optional_non_null_c_void(pointer_mut_nil)
            
            let pointer_non_null = UnsafeMutableRawPointer(mutating: value)
            let pointer_non_null_copy = rust_echo_non_null_c_void(pointer_non_null)
            
            XCTAssertEqual(pointer, pointer_copy)
            XCTAssertEqual(pointer_nil, pointer_nil_copy)
            XCTAssertEqual(pointer_mut, pointer_mut_copy)
            XCTAssertEqual(pointer_mut, pointer_mut_copy2)
            XCTAssertEqual(pointer_mut_nil, pointer_mut_nil_copy)
            XCTAssertEqual(pointer_mut_nil, pointer_mut_nil_copy2)
            XCTAssertEqual(pointer_non_null, pointer_non_null_copy)
        })
    }

    func testSwiftCallRustUInt8() throws {
        var u8 = UInt8(4)

        withUnsafePointer(to: u8, {(value) in
            let pointer = UnsafePointer<UInt8>(value)
            let pointer_mut = UnsafeMutablePointer<UInt8>(mutating: value)
            let pointer_nil: UnsafePointer<UInt8>? = nil
            let pointer_mut_nil: UnsafeMutablePointer<UInt8>? = nil
            
            let pointer_copy = rust_echo_const_u8(pointer)
            let pointer_nil_copy = rust_echo_const_u8(pointer_nil)
            let pointer_mut_copy = rust_echo_mut_u8(pointer_mut)
            let pointer_mut_copy2 = rust_echo_optional_non_null_u8(pointer_mut)
            let pointer_mut_nil_copy = rust_echo_mut_u8(pointer_mut_nil)
            let pointer_mut_nil_copy2 = rust_echo_optional_non_null_u8(pointer_mut_nil)
            
            let pointer_non_null = UnsafeMutablePointer<UInt8>(&u8)
            let pointer_non_null_copy = rust_echo_non_null_u8(pointer_non_null)
            
            XCTAssertEqual(pointer, pointer_copy)
            XCTAssertEqual(pointer_nil, pointer_nil_copy)
            XCTAssertEqual(pointer_mut, pointer_mut_copy)
            XCTAssertEqual(pointer_mut, pointer_mut_copy2)
            XCTAssertEqual(pointer_mut_nil, pointer_mut_nil_copy)
            XCTAssertEqual(pointer_mut_nil, pointer_mut_nil_copy2)
            XCTAssertEqual(pointer_non_null, pointer_non_null_copy)
        })
    }

    func testRustCallSwiftCvoid() throws {
        rust_run_opaque_pointer_tests()
    }
    
    func testRustCallSwiftUInt8() throws {
        rust_run_u8_pointer_tests()
    }
}
