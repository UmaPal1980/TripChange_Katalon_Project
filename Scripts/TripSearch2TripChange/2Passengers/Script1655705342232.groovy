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

def SearchResponse = WS.sendRequestAndVerify(findTestObject('2Passengers/1_Search Request_9Passengers'))

CustomKeywords.'methods.APIKeywords.SearchCatalogueIDAndProductID'(SearchResponse)

//---------------------------------------------------------------------------------------------------------------------
def PriceResponse = WS.sendRequestAndVerify(findTestObject('9Passengers/2_Price Request_9Passengers'))

WS.verifyResponseStatusCode(PriceResponse, 200)

//---------------------------------------------------------------------------------------------------------------------
def InitiateWorkBenchResponse = WS.sendRequestAndVerify(findTestObject('9Passengers/3_TS Intiate_Workbench_9Passengers'))

CustomKeywords.'methods.APIKeywords.InitiateWorkBench'(InitiateWorkBenchResponse)

WS.verifyResponseStatusCode(InitiateWorkBenchResponse, 200)

//---------------------------------------------------------------------------------------------------------------------
WS.sendRequestAndVerify(findTestObject('9Passengers/4_Add_Offer Catalogue_9Passengers'))

//---------------------------------------------------------------------------------------------------------------------
for (def i = 1; i < 10; i++) {

def sResponse = WS.sendRequestAndVerify(findTestObject(('9Passengers/5_Add_Traveler_9Passengers_' + i) + 'Pax'))

CustomKeywords.'methods.APIKeywords.AddTravellerErrorMessage'(sResponse)
}

//---------------------------------------------------------------------------------------------------------------------
def Commit = WS.sendRequestAndVerify(findTestObject('OneWayTrip/6_Commit'))

CustomKeywords.'methods.APIKeywords.Commit'(Commit)

WS.verifyResponseStatusCode(Commit, 200)

def WorkBenchWithPNR = WS.sendRequestAndVerify(findTestObject('OneWayTrip/7_Intiate workbench with PNR'))

CustomKeywords.'methods.APIKeywords.WorkBenchWithPNR'(WorkBenchWithPNR)

WS.verifyResponseStatusCode(WorkBenchWithPNR, 200)

def AddFoP = WS.sendRequestAndVerify(findTestObject('OneWayTrip/8_Add FOP'))

WS.verifyResponseStatusCode(AddFoP, 200)

def AddPayment = WS.sendRequestAndVerify(findTestObject('OneWayTrip/9_Add Payment'))

WS.verifyResponseStatusCode(AddPayment, 200)

def TicketIssuance = WS.sendRequestAndVerify(findTestObject('OneWayTrip/R_Ticket Issuance'))

WS.verifyResponseStatusCode(TicketIssuance, 200)

CustomKeywords.'methods.APIKeywords.TicketNumber'(TicketIssuance)

//-----------------------------------------GetEligibility-------------------------------------------------------------------------------------
//def Eligibility = WS.sendRequest(findTestObject('OneWayTrip/S_GetEligibilityRequest'))

//WS.verifyResponseStatusCode(Eligibility, 200)

//CustomKeywords.'methods.APIKeywords.Eligibility'(Eligibility)

//----------------------------------------ReservationWorkBench---------------------------------------------------------------------------------
def ReservationWorkBench = WS.sendRequest(findTestObject('OneWayTrip/T_Reservationworkbench'))

WS.verifyResponseStatusCode(ReservationWorkBench, 200)

CustomKeywords.'methods.APIKeywords.ReservationWorkBench'(ReservationWorkBench)

//------------------------------------------SearchDomainPNR-------------------------------------------------------------------------------------
def SearchDomainPNR = WS.sendRequestAndVerify(findTestObject('OneWayTrip/U-SearchDomainPNR'))

CustomKeywords.'methods.APIKeywords.ExchSearchIdentifierValue'(SearchDomainPNR)

WS.verifyResponseStatusCode(SearchDomainPNR, 200)

//--------------------------------------------Modify Offer----------------------------------------------------------------
def ModifyOffer = WS.sendRequest(findTestObject('OneWayTrip/V_Modify Offer'))

CustomKeywords.'methods.APIKeywords.OfferIdentifierValue'(ModifyOffer)

WS.verifyResponseStatusCode(ModifyOffer, 200)

//--------------------------------------------Add Payment------------------------------------------------------------------
AddPayment = WS.sendRequest(findTestObject('OneWayTrip/X_Add Payment PNR'))

WS.verifyResponseStatusCode(AddPayment, 200)

//--------------------------------------------------------------------------------------------------------------------------
def ReservationCommit = WS.sendRequest(findTestObject('OneWayTrip/Y_Reservation Commit'))

CustomKeywords.'methods.APIKeywords.ReservationCommit'(ReservationCommit)

WS.verifyResponseStatusCode(ReservationCommit, 200)

