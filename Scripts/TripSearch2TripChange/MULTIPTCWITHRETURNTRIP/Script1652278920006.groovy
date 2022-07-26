import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

def slurper = new groovy.json.JsonSlurper()

def SearchResponse = WS.sendRequestAndVerify(findTestObject('MultiPTC/1_Search_ReturnTrip_MultiPTC'))

CustomKeywords.'methods.APIKeywords.SearchCatalogueIDAndProductID'(SearchResponse)


WS.sendRequestAndVerify(findTestObject('MultiPTC/2_Price_ReturnTrip_MultiPTC'))

def InitiateWorkBenchResponse = WS.sendRequestAndVerify(findTestObject('MultiPTC/3_TS Intiate_Workbench_Return_MultiPTC'))

CustomKeywords.'methods.APIKeywords.InitiateWorkBench'(InitiateWorkBenchResponse)

WS.sendRequestAndVerify(findTestObject('MultiPTC/4_Add_Offer Catalogue_Return_MultiPTC'))

WS.sendRequestAndVerify(findTestObject('MultiPTC/5_Add_Traveler_2ADT1CHD1INF'))

WS.sendRequestAndVerify(findTestObject('MultiPTC/6_Commit_Return_MultiPTC'))

//-----------------------------------------InitiateWorkBenchWithPNR---------------------------------------------------------------------
WorkBenchWithPNR = WS.sendRequest(findTestObject('OneWayTrip/7_Intiate workbench with PNR'))

WS.verifyResponseStatusCode(WorkBenchWithPNR, 200)

def sWorkBenchWithPNR = slurper.parseText(WorkBenchWithPNR.getResponseBodyContent())

sWorkBenchIdentifierWithPNR = sWorkBenchWithPNR.ReservationResponse.Identifier.value

//println('The WorkBenchIdentifierWithPNR is ' + sWorkBenchIdentifierWithPNR)
GlobalVariable.eCacheId = sWorkBenchIdentifierWithPNR

AddFoP = WS.sendRequest(findTestObject('OneWayTrip/8_Add FOP - CreditCard'))

WS.verifyResponseStatusCode(AddFoP, 200)

AddPayment = WS.sendRequestAndVerify(findTestObject('OneWayTrip/9_Add Payment - CreditCard'))

WS.verifyResponseStatusCode(AddPayment, 200)

//----------------------------------------Ticket_Issuance---------------------------------------------------------------------------------
TicketIssuance = WS.sendRequest(findTestObject('OneWayTrip/R_Ticket Issuance'))

WS.verifyResponseStatusCode(TicketIssuance, 200)

CustomKeywords.'methods.APIKeywords.TicketNumber'(TicketIssuance)

